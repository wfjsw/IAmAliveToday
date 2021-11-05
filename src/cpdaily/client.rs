use crate::config::User;
use crate::cpdaily::crypto::ciphers::{des, base64};

use curl::easy::{Easy, List};
use serde_json::Value;
use std::fs::File;
use std::str;

pub struct Client {
    extension: String, 
    user_agent: String,
    base_url: String,
    cookie_jar: File,
}

impl Client {
    pub fn new(base_url: Option<&str>, user: &User) -> Client {
        Client {
            base_url: base_url.unwrap_or("").to_string(),
            extension: base64::encode(des::encrypt(&user.get_cpdaily_extension(), None, None).unwrap().as_slice()), // TODO: DES
            user_agent: user.device_info.user_agent.to_string(),
            cookie_jar: tempfile::tempfile().unwrap(),
        }
    }

    pub fn clone(&self, base_url: Option<&str>, fresh_cookie: bool) -> Client {
        Client {
            base_url: base_url.unwrap_or(self.base_url.as_str()).to_owned(),
            extension: self.extension.clone(),
            user_agent: self.user_agent.clone(),
            cookie_jar: match fresh_cookie {
                true => tempfile::tempfile().unwrap(),
                false => self.cookie_jar, // UNSAFE?
            }
        }
    }

    pub fn perform(url: &str, headers: Option<List>, method: Option<&str>, body: Option<&str>, fail_on_error: bool) -> Result<(u32, String), curl::Error> {
        let mut data = Vec::new();

        let mut easy = Easy::new();
        easy.url(url)?;
        if headers.is_some() {
            easy.http_headers(headers.unwrap())?;
        }

        match method {
            Some("POST") => easy.post(true)?,
            _ => easy.get(true)?,
        }

        if body.is_some() {
            easy.post_field_size(body.unwrap().len() as u64)?;
            easy.post_fields_copy(body.unwrap().as_bytes())?;
        }

        easy.fail_on_error(fail_on_error)?;

        {
            let mut transfer = easy.transfer();
            transfer.write_function(|r| {
                data.extend_from_slice(r);
                Ok(r.len())
            }).expect("Failed to set write function");
            transfer.perform().expect("Unable to perform request");
        }

        Ok((easy.response_code().unwrap(), str::from_utf8(&data).expect("Invalid UTF-8 Sequence").to_string()))
    }

    pub fn get(&self, url: &str) -> Result<(u32, String), curl::Error> {
        let mut headers = List::new();
        headers.append(&("User-Agent: ".to_owned() + &self.user_agent))?;
        headers.append("clientType: cpdaily_student")?;
        headers.append("deviceType: 1")?;
        headers.append("CpdailyClientType: CPDAILY")?;
        headers.append("CpdailyStandAlone: 0")?;

        Client::perform(url, Some(headers), Some("GET"), None, true)
    }

    pub fn get_json(&self, url: &str) -> Result<serde_json::Value, curl::Error> {
        let body = self.get(url)?.1;
        Ok(serde_json::from_str(&body).expect("Invalid JSON"))
    }

    pub fn post(&self, url: &str, data: &str) -> Result<(u32, String), curl::Error> {
        let mut headers = List::new();
        headers.append(&("User-Agent: ".to_owned() + &self.user_agent))?;
        headers.append("clientType: cpdaily_student")?;
        headers.append("deviceType: 1")?;
        headers.append("CpdailyClientType: CPDAILY")?;
        headers.append("CpdailyStandAlone: 0")?;
        
        Client::perform(url, Some(headers), Some("POST"), Some(data), true)
    }

    pub fn post_json(&self, url: &str, data: Value) -> Result<serde_json::Value, curl::Error> {
        let body = self.post(url, &data.to_string())?.1;
        Ok(serde_json::from_str(&body).expect("Invalid JSON"))
    }
}
