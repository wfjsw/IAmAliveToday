use crate::config::User;
use crate::cpdaily::crypto::ciphers::{des, base64};

use curl::easy::{Easy, List};
use serde_json::Value;
use tempfile::NamedTempFile;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write, Seek};
use std::str;

pub struct Client {
    extension: String, 
    user_agent: String,
    base_url: String,
    cookie_jar: NamedTempFile,
}

impl Client {
    pub fn new(base_url: Option<&str>, user: &User) -> Client {
        Client {
            base_url: base_url.unwrap_or("").to_string(),
            extension: base64::encode(des::encrypt(&user.get_cpdaily_extension(), None, None).unwrap().as_slice()), // TODO: DES
            user_agent: user.device_info.user_agent.to_string(),
            cookie_jar: NamedTempFile::new().unwrap(),
        }
    }

    pub fn clone(&mut self, base_url: Option<&str>, fresh_cookie: bool) -> Client {
        Client {
            base_url: base_url.unwrap_or(self.base_url.as_str()).to_owned(),
            extension: self.extension.clone(),
            user_agent: self.user_agent.clone(),
            cookie_jar: match fresh_cookie {
                true => NamedTempFile::new().unwrap(),
                false =>  {
                    let mut cookie_file = NamedTempFile::new().unwrap();
                    // copy old data from self.cookie_jar to cookie_file
                    self.cookie_jar.rewind().unwrap();
                    let mut cookie_data = Vec::new();
                    self.cookie_jar.read_to_end(&mut cookie_data).unwrap();
                    cookie_file.write_all(&cookie_data).unwrap();
                    cookie_file.rewind().unwrap();
                    cookie_file
                }, // UNSAFE?
            }
        }
    }

    pub fn perform(url: &str, headers: Option<List>, method: Option<&str>, body: Option<&str>, fail_on_error: bool, cookie_file: Option<&NamedTempFile>) -> Result<(u32, String), curl::Error> {
        let mut data = Vec::new();
        let mut response_header = HashMap::new();

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

        if cookie_file.is_some() {
            easy.cookie_file(cookie_file.unwrap().path())?;
            easy.cookie_jar(cookie_file.unwrap().path())?;
        }

        easy.fail_on_error(fail_on_error)?;

        {
            let mut transfer = easy.transfer();
            transfer.write_function(|r| {
                data.extend_from_slice(r);
                Ok(r.len())
            }).expect("Failed to set write function");
            transfer.header_function(|r| {
                let hdr = str::from_utf8(r).unwrap().to_string();
                let sp = hdr.split_once(": ");
                if sp.is_some() {
                    let (k, v) = sp.unwrap();
                    response_header.insert(k.to_owned(), v.to_owned());
                }
                true
            }).expect("Failed to set header function");
            transfer.perform().expect("Unable to perform request");
        }

        let code = easy.response_code().unwrap();
        let body = str::from_utf8(&data).expect("Invalid UTF-8 Sequence").to_string();
        
        if fail_on_error {
            match code {
                200 => Ok((code, body)),
                301 | 302 => {
                    let location = response_header.get("Location").unwrap().to_owned();
                    Ok((code, location))
                },
                _ => unreachable!(),
            }
        } else {
            match code {
                301 | 302 => {
                    let location = response_header.get("Location").unwrap().to_owned();
                    Ok((code, location))
                },
                _ => Ok((code, body)),
            } 
        }
    }

    pub fn get(&self, url: &str, fail_on_error: bool) -> Result<(u32, String), curl::Error> {
        let mut headers = List::new();
        headers.append(&("User-Agent: ".to_owned() + &self.user_agent))?;
        headers.append("clientType: cpdaily_student")?;
        headers.append("deviceType: 1")?;
        headers.append("CpdailyClientType: CPDAILY")?;
        headers.append("CpdailyStandAlone: 0")?;

        Client::perform(url, Some(headers), Some("GET"), None, fail_on_error, Some(&self.cookie_jar))
    }

    pub fn get_json(&self, url: &str, fail_on_error: bool) -> Result<serde_json::Value, curl::Error> {
        let body = self.get(url, fail_on_error)?.1;
        Ok(serde_json::from_str(&body).expect("Invalid JSON"))
    }

    pub fn post(&self, url: &str, data: &str, fail_on_error: bool) -> Result<(u32, String), curl::Error> {
        let mut headers = List::new();
        headers.append(&("User-Agent: ".to_owned() + &self.user_agent))?;
        headers.append("clientType: cpdaily_student")?;
        headers.append("deviceType: 1")?;
        headers.append("CpdailyClientType: CPDAILY")?;
        headers.append("CpdailyStandAlone: 0")?;
        
        Client::perform(url, Some(headers), Some("POST"), Some(data), fail_on_error, Some(&self.cookie_jar))
    }

    pub fn post_json(&self, url: &str, data: Value, fail_on_error: bool) -> Result<serde_json::Value, curl::Error> {
        let body = self.post(url, &data.to_string(), fail_on_error)?.1;
        Ok(serde_json::from_str(&body).expect("Invalid JSON"))
    }
}
