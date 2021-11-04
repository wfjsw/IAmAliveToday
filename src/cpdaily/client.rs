use crate::config::User;
use crate::cpdaily::crypto::ciphers::{des, base64};

use curl::easy::{Easy, List};
use serde_json::Value;
use std::str;

pub struct Client {
    extension: String, 
    user_agent: String,
    base_url: String,
}

impl Client {
    pub fn new(base_url: Option<&str>, user: &User) -> Client {
        Client {
            base_url: base_url.unwrap_or("").to_string(),
            extension: base64::encode(des::encrypt(&user.get_cpdaily_extension(), None, None).unwrap().as_slice()), // TODO: DES
            user_agent: user.device_info.user_agent.to_string(),
        }
    }

    pub fn clone(&self, base_url: Option<&str>) -> Client {
        Client {
            base_url: base_url.unwrap_or(self.base_url.as_str()).to_owned(),
            extension: self.extension.clone(),
            user_agent: self.user_agent.clone(),
        }
    }

    pub fn get(&self, url: &str) -> Result<String, curl::Error> {
        let mut headers = List::new();
        headers.append(&("User-Agent: ".to_owned() + &self.user_agent))?;
        headers.append("clientType: cpdaily_student")?;
        headers.append("deviceType: 1")?;
        headers.append("CpdailyClientType: CPDAILY")?;
        headers.append("CpdailyStandAlone: 0")?;

        let mut req = Easy::new();
        req.http_headers(headers)?;
        req.url(&(self.base_url.to_owned() + url))?;
        req.get(true)?;

        let mut dst  = Vec::new();
        {
            let mut transfer = req.transfer();
            transfer.write_function(|data| {
                dst.extend_from_slice(data);
                Ok(data.len())
            })?;
        }
        req.perform()?;
        Ok(str::from_utf8(&dst).expect("Invalid UTF-8 Sequence").to_owned())
    }

    pub fn get_json(&self, url: &str) -> Result<serde_json::Value, curl::Error> {
        let body = self.get(url)?;
        Ok(serde_json::from_str(&body).expect("Invalid JSON"))
    }

    pub fn post(&self, url: &str, data: &str) -> Result<String, curl::Error> {
        let mut headers = List::new();
        headers.append(&("User-Agent: ".to_owned() + &self.user_agent))?;
        headers.append("clientType: cpdaily_student")?;
        headers.append("deviceType: 1")?;
        headers.append("CpdailyClientType: CPDAILY")?;
        headers.append("CpdailyStandAlone: 0")?;
        
        let mut req = Easy::new();
        req.http_headers(headers)?;
        req.url(&(self.base_url.to_owned() + url))?;
        req.post(true)?;
        req.post_field_size(data.len() as u64)?;
        req.post_fields_copy(data.as_bytes())?;

        let mut dst = Vec::new();
        {
            let mut transfer = req.transfer();
            transfer.write_function(|data| {
                dst.extend_from_slice(data);
                Ok(data.len())
            })?;
        }

        req.perform()?;
        Ok(str::from_utf8(&dst).expect("Invalid UTF-8 Sequence").to_owned())
    }

    pub fn post_json(&self, url: &str, data: Value) -> Result<serde_json::Value, curl::Error> {
        let body = self.post(url, &data.to_string())?;
        Ok(serde_json::from_str(&body).expect("Invalid JSON"))
    }
}
