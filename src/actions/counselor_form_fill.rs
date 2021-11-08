use reqwest::{StatusCode, blocking::Client};
use serde::{Deserialize, Serialize};
use anyhow::{anyhow, Result};
use serde_json::{Value, json};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WiseduResponse<T> {
    pub code: i32,
    pub msg: String,
    pub datas: T,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WiseduPaginator<T> {
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
    pub form_wid: i64,
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

pub fn perform(session: &Client, base_url: &str, config: &CounselorFormFillAction) -> Result<()> {
    let form_list = get_form_list(session, base_url, 20, 1)?;

    Ok(())
}


/*
{
    "code": "0",
    "message": "SUCCESS",
    "datas": {
        "totalSize": 1,
        "pageSize": 20,
        "pageNumber": 1,
        "rows": [
            {
                "wid": "39366",
                "instanceWid": 243,
                "formWid": "2551",
                "priority": "4",
                "subject": "11月7日学生日报信息收集",
                "content": "https://wecres.cpdaily.com/counselor/1018615895163461/html/4b4aa14926824de0a1357a970c6c66d5.html",
                "senderUserName": "姜国华(信息科学技术学院)",
                "createTime": "2021-11-08 00:16",
                "startTime": "2021-11-08 06:00",
                "endTime": "2021-11-08 23:59",
                "currentTime": "2021-11-08 08:44:52",
                "isHandled": 1,
                "isRead": 1
            }
        ]
    }
}
*/

// TODO: replace Value with some strong type
pub fn get_form_list(session: &Client, base_url: &str, page_size: u32, page_number: u32) -> Result<Vec<CollectorFormInstance>> {
    let result = session.get(format!("{}/wec-counselor-collector-apps/stu/collector/queryCollectorProcessingList", base_url))
        .json(&json!({
            "pageSize": page_size,
            "pageNumber": page_number,
        }))
        .send()?;
        
    match result.status() {
        StatusCode::OK => {
            let resp : WiseduResponse<WiseduPaginator<CollectorFormInstance>> = result.json()?;
            let data = resp.datas;
            if data.total_size == 0 {
                return Ok(vec![])
            } else {
                Ok(data.rows)
            }
        },
        StatusCode::NOT_FOUND => {
            Err(anyhow!("Form list not found"))
        },
        _ => {
            // TODO: refine
            unreachable!()
        }
    }
}
