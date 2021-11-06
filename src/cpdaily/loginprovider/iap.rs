use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::LoginProvider;
use crate::cpdaily::client::Client;

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
        let lt_info = session.post_json(&format!("{}/security/lt", &self.url), json!({}), true)?;
        let mut params = LoginParams {
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

        let login_result = session.post(&format!("{}/doLogin", &self.url), "", true)?;
        if login_result.0 == 302 {
            session.get(&login_result.1, true)?;
            Ok(())
        } else {
            let result_obj : Value = serde_json::from_str(&login_result.1).unwrap();
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
        let result = session.post_json(&format!("{}/needCaptcha?username={}", &self.url, username), json!({}), false)?;
        Ok(result.get("needCaptcha").unwrap().as_bool().unwrap())
    }

}
