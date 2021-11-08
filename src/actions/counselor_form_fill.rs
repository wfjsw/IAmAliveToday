use anyhow::{anyhow, Result};
use reqwest::{blocking::Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::cpdaily::crypto::traits::first_v2::FirstV2;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CounselorResponse<T> {
    pub code: String,
    #[serde(default)]
    pub message: String,
    pub datas: T,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CounselorPaginator<T> {
    pub total_size: i64,
    pub page_size: i64,
    pub page_number: i64,
    pub rows: Vec<T>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectorFormInstance {
    pub wid: String,
    pub instance_wid: i64,
    pub form_wid: String,
    pub priority: String,
    pub subject: String,
    pub content: String,
    pub sender_user_name: String,
    pub create_time: String,
    pub start_time: String,
    pub end_time: String,
    pub current_time: String,
    pub is_handled: i64,
    pub is_read: i64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CounselorFormFillAction {
    pub form_data: Vec<QA>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct QA {
    pub question: String,
    pub answer: String,
}

pub fn perform(
    session: &Client,
    base_url: &str,
    config: &CounselorFormFillAction,
    first_v2_provider: &dyn FirstV2,
) -> Result<()> {
    let form_list = get_form_list(session, base_url, 20, 1)?;

    for form in form_list {
        if form.is_handled == 1 {
            // skip filled forms
            continue;
        }
    }

    Ok(())
}

// TODO: replace Value with some strong type
pub fn get_form_list(
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
                return Ok(vec![]);
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

#[cfg(test)]
mod tests {
    use std::env;

    use reqwest::{
        blocking::Client,
        header::{HeaderMap, HeaderValue, COOKIE},
    };

    use crate::actions::counselor_form_fill::{
        CollectorFormInstance, CounselorPaginator, CounselorResponse,
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
        assert_eq!(parsed_response.datas.rows[0].instance_wid, 2345);
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
