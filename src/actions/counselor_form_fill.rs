use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::cpdaily::client::Client;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CounselorFormFillAction {
    pub form_data: Vec<QA>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct QA {
    pub question: String,
    pub answer: String,
}

pub fn perform(session: &Client, config: &CounselorFormFillAction) {
    
}

// pub fn get_form_list(session: &Client, page_size: u32, page_number: u32) -> Result<Vec<()>> {
//     let result = session.get_json();
// }
