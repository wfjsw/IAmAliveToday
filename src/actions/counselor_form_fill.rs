use reqwest::{StatusCode, blocking::Client};
use serde::{Deserialize, Serialize};
use anyhow::{anyhow, Result};
use serde_json::json;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WiseduResponse<T> {
    pub code: i64,
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
