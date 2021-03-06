mod structs;

use crate::{
    config::User,
    cpdaily::crypto::{
        ciphers::md5,
        traits::first_v2::{self, FirstV2},
    },
};
use anyhow::{anyhow, Result};
use reqwest::{blocking::Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use structs::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CounselorFormFillAction {
    pub form_data: Vec<QA>,
    pub force_submit: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QA {
    pub question: String,
    pub answer: String,
}

pub fn perform(
    session: &Client,
    base_url: &str,
    config: &CounselorFormFillAction,
    user: &User,
    encryptor: &dyn FirstV2,
) -> Result<()> {
    let form_list = get_form_list(session, base_url, 20, 1)?;

    for form in form_list {
        if form.is_handled == 1 && !config.force_submit {
            // skip filled forms
            crate::logger::log(sentry::Breadcrumb {
                category: Some("counselor_form_fill".to_string()),
                message: Some(format!(
                    "skipping [{}]{} as it is already filled",
                    &form.wid, &form.subject
                )),
                level: sentry::Level::Info,
                ..Default::default()
            });
            continue;
        }

        crate::logger::log(sentry::Breadcrumb {
            category: Some("counselor_form_fill".to_string()),
            message: Some(format!("filling [{}]{}", &form.wid, &form.subject)),
            level: sentry::Level::Info,
            ..Default::default()
        });

        let form_detail = get_form_detail(session, base_url, &form.wid, form.instance_wid)?;
        let mut form_fields =
            get_form_fields(session, base_url, &form.wid, &form.form_wid, 100, 1)?;

        crate::logger::log(sentry::Breadcrumb {
            category: Some("counselor_form_fill".to_string()),
            message: Some(format!(
                "({}) fetched {} fields",
                &form.wid,
                form_fields.len()
            )),
            level: sentry::Level::Debug,
            ..Default::default()
        });

        let fill_resp = fill_fields(&mut form_fields, config);
        if let Err(err) = fill_resp {
            sentry::integrations::anyhow::capture_anyhow(&err);
            return Err(anyhow!(err));
        }

        let form_data = FormContentForSubmit {
            form_wid: form.form_wid,
            address: user.address.clone(),
            collect_wid: form.wid.clone(),
            school_task_wid: form_detail.collector.school_task_wid,
            form: serde_json::Value::Array(form_fields),
            ua_is_cpadaily: true,
            latitude: user.device_info.lat,
            longitude: user.device_info.lon,
            instance_wid: form.instance_wid,
        };

        post_form(session, base_url, &form_data, user, encryptor)?;

        crate::logger::log(sentry::Breadcrumb {
            category: Some("counselor_form_fill".to_string()),
            message: Some(format!("({}) posted", &form.wid)),
            level: sentry::Level::Debug,
            ..Default::default()
        });
    }

    Ok(())
}

fn fill_fields(
    form_fields: &mut Vec<Value>,
    config: &CounselorFormFillAction,
) -> anyhow::Result<()> {
    for field in form_fields.iter_mut() {
        let field_type: i32 = field
            .get("fieldType")
            .unwrap()
            .as_str()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let title = field.get("title").unwrap().as_str().unwrap().to_string();
        let answer = get_answer_from_config(config, &title);

        if let Some(answer_str) = answer {
            // 1.?????? 2.????????? 3.????????? 4.???????????? 5???????????? 6???????????? 7???????????? 8 ?????? 9 ?????? 10 ???????????? 11????????? 12 ????????? 13 ???????????? 14 ???????????? 15 ???????????? 16 ???????????? 17 ???????????? 18 ???????????? 19????????? 20????????? 21 ???????????? 22 ???????????? 23????????????
            match field_type {
                1 | 5 | 6 | 7 => {
                    // text
                    field
                        .as_object_mut()
                        .unwrap()
                        .insert("value".to_string(), json!(&answer_str));

                    crate::logger::log(sentry::Breadcrumb {
                        category: Some("counselor_form_fill".to_string()),
                        message: Some(format!("filled text field: {} => {}", &title, &answer_str)),
                        level: sentry::Level::Debug,
                        ..Default::default()
                    });
                }
                2 => {
                    // single choice
                    let options: Vec<Value> = field
                        .get("fieldItems")
                        .unwrap()
                        .as_array()
                        .unwrap()
                        .to_vec()
                        .into_iter()
                        .filter(|item| {
                            item.get("content")
                                .unwrap()
                                .as_str()
                                .unwrap()
                                .contains(&answer_str)
                        })
                        .collect();
                    assert_eq!(options.len(), 1, "Unexpected filtered option length");
                    let f = field.as_object_mut().unwrap();
                    let wid = options[0]
                        .get("itemWid")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string();
                    f.insert("fieldItems".to_string(), serde_json::Value::Array(options));
                    f.insert("value".to_string(), json!(&wid));

                    crate::logger::log(sentry::Breadcrumb {
                        category: Some("counselor_form_fill".to_string()),
                        message: Some(format!(
                            "filled single-choice field: {} => {}",
                            &title, &answer_str
                        )),
                        level: sentry::Level::Debug,
                        ..Default::default()
                    });
                }
                3 => {
                    unimplemented!("multi choice");
                }
                4 => {
                    unimplemented!("upload photo");
                }
                _ => {
                    // other
                    unimplemented!("unimplemented field type");
                }
            }
        } else if field.get("isRequired").unwrap().as_bool().unwrap() {
            // required field
            return Err(anyhow!(
                "Required field \"{}\" not found",
                field.get("title").unwrap().as_str().unwrap()
            ));
        }
    }
    Ok(())
}

fn get_form_list(
    session: &Client,
    base_url: &str,
    page_size: u32,
    page_number: u32,
) -> Result<Vec<CollectorFormInstance>> {
    let result = session
        .post(format!(
            "{}/wec-counselor-collector-apps/stu/collector/queryCollectorProcessingList",
            base_url
        ))
        .json(&json!({
            "pageSize": page_size,
            "pageNumber": page_number,
        }))
        .send()?;

    match result.status() {
        StatusCode::OK => {
            let resp: CounselorResponse<CounselorPaginator<CollectorFormInstance>> =
                result.json()?;
            let data = resp.datas;
            if data.total_size == 0 {
                Ok(vec![])
            } else {
                Ok(data.rows)
            }
        }
        StatusCode::NOT_FOUND => Err(anyhow!("Form list not found")),
        _ => {
            // TODO: refine
            unreachable!()
        }
    }
}

fn get_form_detail(
    session: &Client,
    base_url: &str,
    wid: &str,
    instance_wid: Option<i64>,
) -> Result<FormDetail> {
    let result: CounselorResponse<FormDetail> = session
        .post(format!(
            "{}/wec-counselor-collector-apps/stu/collector/detailCollector",
            base_url
        ))
        .json(&json!({
            "collectorWid": wid,
            "instanceWid": instance_wid,
        }))
        .send()?
        .json()?;
    Ok(result.datas)
}

fn get_form_fields(
    session: &Client,
    base_url: &str,
    wid: &str,
    form_wid: &str,
    page_size: u32,
    page_number: u32,
) -> Result<Vec<Value>> {
    let result: CounselorResponse<CounselorPaginator<Value>> = session
        .post(format!(
            "{}/wec-counselor-collector-apps/stu/collector/getFormFields",
            base_url
        ))
        .json(&json!({
            "pageSize": page_size,
            "pageNumber": page_number,
            "formWid": form_wid,
            "collectorWid": wid,
        }))
        .send()?
        .json()?;
    Ok(result.datas.rows)
}

fn post_form(
    session: &Client,
    base_url: &str,
    form_data: &FormContentForSubmit,
    user: &User,
    encryptor: &dyn FirstV2,
) -> Result<()> {
    let json_stringifyed_form = serde_json::to_string(form_data)?;
    let encrypted_form = encryptor.encrypt(&json_stringifyed_form, first_v2::KeyType::F)?;
    let key = encryptor.get_key(first_v2::KeyType::F);
    let ext = user.get_cpdaily_extension().to_urlencoded();
    let sign_hash = md5::hash(&format!("{}&{}", &ext, &key))?;
    let payload = FormSubmitRequest {
        app_version: user.device_info.app_version.clone(),
        system_name: user.device_info.system_name.clone(),
        body_string: encrypted_form,
        sign: sign_hash,
        model: user.device_info.model.clone(),
        lat: form_data.latitude,
        lon: form_data.longitude,
        cal_version: "firstv".to_string(),
        system_version: user.device_info.system_version.clone(),
        device_id: user.device_info.device_id.clone(),
        user_id: user.username.clone(),
        version: "first_v2".to_string(),
    };
    let result = session
        .post(format!(
            "{}/wec-counselor-collector-apps/stu/collector/submitForm",
            base_url
        ))
        .json(&payload)
        .send()?;

    println!("{}", result.status());
    println!("{}", result.text()?);
    Ok(())
}

fn get_answer_from_config(config: &CounselorFormFillAction, question: &str) -> Option<String> {
    for qa in config.form_data.iter() {
        if question.contains(&qa.question) {
            return Some(qa.answer.clone());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use std::env;

    use reqwest::{
        blocking::Client,
        header::{HeaderMap, HeaderValue, COOKIE},
    };

    use crate::actions::counselor_form_fill::{
        CollectorFormInstance, CounselorPaginator, CounselorResponse, FormDetail,
    };

    #[test]
    fn test_counselor_form_list_deserialise() {
        let response = r#"{"code":"0","message":"SUCCESS","datas":{"totalSize":1,"pageSize":20,"pageNumber":1,"rows":[{"wid":"1234","instanceWid":2345,"formWid":"3456","priority":"4","subject":"test","content":"https://wecres.cpdaily.com/counselor/test/html/test.html","senderUserName":"test(test)","createTime":"2021-11-08 00:16","startTime":"2021-11-08 06:00","endTime":"2021-11-08 23:59","currentTime":"2021-11-08 14:25:38","isHandled":1,"isRead":1}]}}"#;
        let parsed_response: CounselorResponse<CounselorPaginator<CollectorFormInstance>> =
            serde_json::from_str(response).unwrap();
        assert_eq!(parsed_response.code, "0");
        assert_eq!(parsed_response.message, "SUCCESS");
        assert_eq!(parsed_response.datas.total_size, 1);
        assert_eq!(parsed_response.datas.page_size, 20);
        assert_eq!(parsed_response.datas.page_number, 1);
        assert_eq!(parsed_response.datas.rows.len(), 1);
        assert_eq!(parsed_response.datas.rows[0].wid, "1234");
        assert_eq!(parsed_response.datas.rows[0].instance_wid, Some(2345));
        assert_eq!(parsed_response.datas.rows[0].form_wid, "3456");
        assert_eq!(parsed_response.datas.rows[0].priority, "4");
        assert_eq!(parsed_response.datas.rows[0].subject, "test");
        assert_eq!(
            parsed_response.datas.rows[0].content,
            "https://wecres.cpdaily.com/counselor/test/html/test.html"
        );
        assert_eq!(parsed_response.datas.rows[0].sender_user_name, "test(test)");
        assert_eq!(
            parsed_response.datas.rows[0].create_time,
            "2021-11-08 00:16"
        );
        assert_eq!(parsed_response.datas.rows[0].start_time, "2021-11-08 06:00");
        assert_eq!(parsed_response.datas.rows[0].end_time, "2021-11-08 23:59");
        assert_eq!(
            parsed_response.datas.rows[0].current_time,
            "2021-11-08 14:25:38"
        );
        assert_eq!(parsed_response.datas.rows[0].is_handled, 1);
        assert_eq!(parsed_response.datas.rows[0].is_read, 1);
    }

    #[test]
    fn test_counselor_form_detail_deserialise() {
        let response = r#"{"code":"0","message":"SUCCESS","datas":{"collector":{"wid":"1234","instanceWid":2345,"formWid":"3456","priority":"4","endTime":"2021-11-08 23:59:00","currentTime":"2021-11-08 15:34:41","schoolTaskWid":"4567","isConfirmed":0,"senderUserName":"test(test)","createTime":"2021-11-08 00:16:31","attachmentUrls":null,"attachmentNames":null,"attachmentSizes":null,"isUserSubmit":1,"fetchStuLocation":true,"isLocationFailedSub":false,"address":"test123","subject":"test234"},"form":{"wid":"1234","formType":"0","formTitle":"test345","examTime":-1,"formContent":"https://wecres.cpdaily.com/counselor/test/html/test.html","backReason":null,"isBack":0,"attachments":[],"score":0,"stuScore":null,"confirmDesc":"?????????????????????????????????????????????","isshowOrdernum":1,"isAnonymous":0,"isallowUpdated":1,"isshowScore":0,"isshowResult":1}}}"#;
        let parsed_response: CounselorResponse<FormDetail> =
            serde_json::from_str(response).unwrap();
        assert_eq!(parsed_response.code, "0");
        assert_eq!(parsed_response.message, "SUCCESS");
        assert_eq!(parsed_response.datas.collector.wid, "1234");
        assert_eq!(parsed_response.datas.collector.instance_wid, Some(2345));
        assert_eq!(parsed_response.datas.collector.form_wid, "3456");
        assert_eq!(parsed_response.datas.collector.priority, "4");
        assert_eq!(
            parsed_response.datas.collector.end_time,
            "2021-11-08 23:59:00"
        );
        assert_eq!(
            parsed_response.datas.collector.current_time,
            "2021-11-08 15:34:41"
        );
        assert_eq!(parsed_response.datas.collector.school_task_wid, "4567");
        assert_eq!(parsed_response.datas.collector.is_confirmed, 0);
        assert_eq!(
            parsed_response.datas.collector.sender_user_name,
            "test(test)"
        );
        assert_eq!(
            parsed_response.datas.collector.create_time,
            "2021-11-08 00:16:31"
        );
        // assert_eq!(parsed_response.datas.collector.attachment_urls, None);
        // assert_eq!(parsed_response.datas.collector.attachment_names, None);
        // assert_eq!(parsed_response.datas.collector.attachment_sizes, None);
        assert_eq!(parsed_response.datas.collector.is_user_submit, 1);
        assert_eq!(parsed_response.datas.collector.fetch_stu_location, true);
        assert_eq!(
            parsed_response.datas.collector.is_location_failed_sub,
            false
        );
        assert_eq!(
            parsed_response.datas.collector.address,
            Some("test123".to_string())
        );
        assert_eq!(parsed_response.datas.collector.subject, "test234");
        assert_eq!(parsed_response.datas.form.wid, "1234");
        assert_eq!(parsed_response.datas.form.form_type, "0");
        assert_eq!(parsed_response.datas.form.form_title, "test345");
        // assert_eq!(parsed_response.datas.form.exam_time, -1);
        assert_eq!(
            parsed_response.datas.form.form_content,
            "https://wecres.cpdaily.com/counselor/test/html/test.html"
        );
        // assert_eq!(parsed_response.datas.form.back_reason, None);
        // assert_eq!(parsed_response.datas.form.is_back, 0);
        // assert_eq!(parsed_response.datas.form.attachments, vec![]);
        // assert_eq!(parsed_response.datas.form.score, 0);
        // assert_eq!(parsed_response.datas.form.stu_score, None);
        assert_eq!(
            parsed_response.datas.form.confirm_desc,
            "?????????????????????????????????????????????"
        );
        assert_eq!(parsed_response.datas.form.is_show_ordernum, 1);
        assert_eq!(parsed_response.datas.form.is_anonymous, 0);
        assert_eq!(parsed_response.datas.form.is_allow_updated, 1);
        assert_eq!(parsed_response.datas.form.is_show_score, 0);
        assert_eq!(parsed_response.datas.form.is_show_result, 1);
    }

    #[test]
    fn test_get_form_list() {
        let platform_host = env::var_os("PLATFORM_HOST");
        let mod_auth_cas = env::var_os("MOD_AUTH_CAS");
        if platform_host.is_none() || mod_auth_cas.is_none() {
            println!("PLATFORM_HOST or MOD_AUTH_CAS is not set. Skipping...");
            return;
        }

        let platform_host = platform_host.unwrap().to_str().unwrap().to_owned();
        let mod_auth_cas = mod_auth_cas.unwrap().to_str().unwrap().to_owned();

        let mut headers = HeaderMap::new();
        headers.append(
            COOKIE,
            HeaderValue::from_str(&format!("MOD_AUTH_CAS={}", &mod_auth_cas)).unwrap(),
        );

        let client = Client::builder()
            .default_headers(headers)
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();

        let form_list = super::get_form_list(&client, &platform_host, 20, 1).unwrap();
        println!("{:#?}", form_list);
    }
}
