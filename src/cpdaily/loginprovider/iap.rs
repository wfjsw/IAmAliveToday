use reqwest::{blocking::Client};
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

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IAPResponse<T> {
    pub code: i64,
    pub message: String,
    pub result: T,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LTResponse {
    #[serde(rename = "_encryptSalt")]
    pub encrypt_salt: String,
    #[serde(rename = "_lt")]
    pub lt: String,
    pub forget_pwd_url: String,
    pub need_captcha: bool,
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

        let login_result = session.post(&format!("{}/doLogin", &self.url))
            .form(&params)
            .body("")
            .send()?;

        let result_obj : Value = login_result.json()?;
        let result_code = result_obj.get("resultCode").unwrap().as_str().unwrap();
        match result_code {
            "REDIRECT" => {
                let redirect_url = result_obj.get("url").unwrap().get("redirectUrl").unwrap().as_str().unwrap();
                session.get(redirect_url).send()?;
                Ok(())
            },
            "CAPTCHA_NOTMATCH" => Err(anyhow::anyhow!("CAPTCHA_NOTMATCH")),
            "FAIL_UPNOTMATCH" => Err(anyhow::anyhow!("FAIL_UPNOTMATCH")),
            "LT_NOTMATCH" => panic!("LT_NOTMATCH"),
            _ => unimplemented!(),
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


#[cfg(test)]
mod tests {
    use reqwest::blocking::Client;

    use super::LoginProvider;
    use super::{IAP, IAPResponse, LTResponse};

    #[test]
    fn test_lt_deserialise() {
        let response = r#"{"code":200,"message":"操作成功","result":{"_encryptSalt":"6044cb8792f0452c","_lt":"8adf66e6c0944f8da02d0befde246517","forgetPwdUrl":"/personCenter/new_password_retrieve/index.html","needCaptcha":false}}"#;
        let parsed_response : IAPResponse<LTResponse> = serde_json::from_str(response).unwrap();
        assert_eq!(parsed_response.code, 200);
        assert_eq!(parsed_response.message, "操作成功");
        assert_eq!(parsed_response.result.lt, "8adf66e6c0944f8da02d0befde246517");
        assert_eq!(parsed_response.result.encrypt_salt, "6044cb8792f0452c");
        assert_eq!(parsed_response.result.forget_pwd_url, "/personCenter/new_password_retrieve/index.html");
        assert_eq!(parsed_response.result.need_captcha, false);
    }

    #[test]
    fn test_iap_login() {
        let iap_url = option_env!("IAP_URL");
        let username = option_env!("IAP_USERNAME");
        let password = option_env!("IAP_PASSWORD");
        if iap_url.is_none() || username.is_none() || password.is_none() {
            println!("IAP_URL, IAP_USERNAME, IAP_PASSWORD must be set. Skipping...");
            return;
        }
        
        let client = Client::builder().cookie_store(true).build().unwrap();
        let iap = IAP {
            url: iap_url.unwrap().to_string(),
        };
        iap.login(&client, &username.unwrap(), &password.unwrap()).unwrap();
    }
}
