use crate::config::User;
use cached::proc_macro::cached;
use reqwest::blocking::ClientBuilder;
use reqwest::header::{HeaderMap, HeaderValue};

pub fn new(user: &User) -> Result<reqwest::blocking::Client, reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.insert("clientType", HeaderValue::from_static("cpdaily_student"));
    headers.insert("deviceType", HeaderValue::from_static("1"));
    headers.insert("CpdailyClientType", HeaderValue::from_static("CPDAILY"));
    headers.insert("CpdailyStandAlone", HeaderValue::from_static("0"));
    ClientBuilder::new()
        .user_agent(&user.device_info.user_agent)
        .default_headers(headers)
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()
}

#[cached(name = "CLIENT_UNAUTH", result = true)]
pub fn unauth() -> Result<reqwest::blocking::Client, reqwest::Error> {
    ClientBuilder::new()
        .user_agent("Mozilla/5.0 (Linux; U; Android 8.1.0; zh-cn; BLA-AL00 Build/HUAWEIBLA-AL00) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/57.0.2987.132 MQQBrowser/8.9 Mobile Safari/537.36")
        .redirect(reqwest::redirect::Policy::none())
        .build()
}
