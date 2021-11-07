use reqwest::{StatusCode, blocking::Client};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::LoginProvider;

pub struct IAP {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct LoginParams {
    pub lt: String,
    pub remember_me: bool,
    pub dllt: String,
    pub mobile: String,
    pub username: String,
    pub password: String,
    pub captcha: String,
}

impl LoginProvider for IAP {
    fn login(&self, session: &Client, username: &str, password: &str) -> anyhow::Result<()> {
        // url = something.com/iap
        // let lt_info = session.post_json(&format!("{}/security/lt", &self.url), json!({}), None, true)?;
        let lt_info : Value = session.post(&format!("{}/security/lt", &self.url))
            .json(&json!({}))
            .send()?.json()?;
        let params = LoginParams {
            lt: lt_info.get("result").unwrap().get("_lt").unwrap().as_str().unwrap().to_string(),
            remember_me: false,
            dllt: "".to_string(),
            mobile: "".to_string(),
            username: username.to_string(),
            password: password.to_string(),
            captcha: "".to_string(),
        };

        let need_captcha = self.get_need_captcha_url(session, username)?;

        if need_captcha {
            todo!();
        }

        // TODO: Fix this. This is wrong.

        let login_result = session.post(&format!("{}/doLogin", &self.url))
            .query(&params)
            .body("")
            .send()?;

        if login_result.status() == StatusCode::FOUND {
            // session.get(&login_result.1, None, true)?;
            session.get(&login_result.headers().get("location").unwrap().to_str().unwrap().to_string()).send()?;
            Ok(())
        } else {
            let result_obj : Value = login_result.json()?;
            let result_code = result_obj.get("resultCode").unwrap().as_str().unwrap();
            match result_code {
                "CAPTCHA_NOTMATCH" => Err(anyhow::anyhow!("CAPTCHA_NOTMATCH")),
                "FAIL_UPNOTMATCH" => Err(anyhow::anyhow!("FAIL_UPNOTMATCH")),
                "LT_NOTMATCH" => panic!("LT_NOTMATCH"),
                _ => unimplemented!(),
            }
        }
    }
}

impl IAP {
    fn get_need_captcha_url(&self, session: &Client, username: &str) -> anyhow::Result<bool> {
        let result : Value = session.post(&format!("{}/needCaptcha", &self.url))
            .query(&[("username", username)])
            .json(&json!({}))
            .send()?
            .json()?;
        Ok(result.get("needCaptcha").unwrap().as_bool().unwrap())
    }

}
