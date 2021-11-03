use crate::config::User;

use curl::easy::{Easy, List};
use std::str;

struct Client {
    extension: String, 
    user_agent: String,
    base_url: String,
}

impl Client {
    fn new(base_url: Option<&str>, user: &User) -> Client {
        Client {
            base_url: base_url.unwrap_or("").to_string(),
            extension: user.get_cpdaily_extension(), // TODO: DES
            user_agent: user.device_info.user_agent.to_string(),
        }
    }

    fn clone(&self, base_url: Option<&str>) -> Client {
        Client {
            base_url: base_url.unwrap_or(self.base_url.as_str()).to_owned(),
            extension: self.extension.clone(),
            user_agent: self.user_agent.clone(),
        }
    }

    fn get(&self, url: &str) -> Result<String, curl::Error> {
        let mut headers = List::new();
        headers.append(&("User-Agent: ".to_owned() + &self.user_agent));
        headers.append(&("X-Extension: ".to_owned() + &self.extension));
        let mut req = Easy::new();
        req.http_headers(headers)?;
        req.url(&(self.base_url + url))?;
        req.get(true)?;

        let mut dst = Vec::new();
        req.write_function(|data| {
            dst.extend_from_slice(data);
            Ok(data.len())
        })?;

        req.perform()?;
        Ok(str::from_utf8(&dst).expect("Invalid UTF-8 Sequence").to_owned())
    }

    fn post(&self, url: &str, data: &str) -> Result<String, curl::Error> {
        let mut headers = List::new();
        headers.append(&("User-Agent: ".to_owned() + &self.user_agent));
        let mut req = Easy::new();
        req.http_headers(headers)?;
        req.url(&(self.base_url + url))?;
        req.post(true)?;
        req.post_field_size(data.len() as u64)?;
        req.post_fields_copy(data.as_bytes())?;

        let mut dst = Vec::new();
        req.write_function(|data| {
            dst.extend_from_slice(data);
            Ok(data.len())
        })?;

        req.perform()?;
        Ok(str::from_utf8(&dst).expect("Invalid UTF-8 Sequence").to_owned())
    }
}
