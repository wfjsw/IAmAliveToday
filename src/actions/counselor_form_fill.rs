use reqwest::{StatusCode, blocking::Client};
use serde::{Deserialize, Serialize};
use anyhow::{anyhow, Result};
use serde_json::{Value, json};

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
pub fn get_form_list(session: &Client, base_url: &str, page_size: u32, page_number: u32) -> Result<Vec<Value>> {
    let result = session.get(format!("{}/wec-counselor-collector-apps/stu/collector/queryCollectorProcessingList", base_url))
        .json(&json!({
            "pageSize": page_size,
            "pageNumber": page_number,
        }))
        .send()?;
        
    match result.status() {
        StatusCode::OK => {
            let resp : Value = result.json::<Value>()?;
            let data = resp.get("datas").unwrap();
            if data.get("totalSize").unwrap() == 0 {
                return Ok(vec![])
            } else {
                Ok(data.get("datas").unwrap().get("rows").unwrap().as_array().unwrap().to_vec())
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
