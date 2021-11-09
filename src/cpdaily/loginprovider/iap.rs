use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

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
    fn login(&self, session: &Client, username: &str, password: &str) -> anyhow::Result<String> {
        let portal_url = self.url.clone().replace("/iap", "/portal/login");
        let anchor_response = session
            .get(&format!("{}/login", &self.url))
            .query(&[("service", &portal_url)])
            .send()?;

        if !anchor_response.status().is_redirection() {
            return Err(anyhow::anyhow!(
                "Unexpected non-redirection response on login page"
            ));
        }

        let anchor_url = anchor_response
            .headers()
            .get("Location")
            .unwrap()
            .to_str()
            .unwrap();

        let prior_lt = &anchor_url[anchor_url.find('=').unwrap() + 1..anchor_url.len()];

        let mut headers = reqwest::header::HeaderMap::new();
        headers.append(
            "Accept",
            reqwest::header::HeaderValue::from_static("application/json, text/plain, */*"),
        );
        headers.append(
            "X-Requested-With",
            reqwest::header::HeaderValue::from_static("XMLHttpRequest"),
        );
        headers.append(
            "Referer",
            reqwest::header::HeaderValue::from_str(anchor_url)?,
        );

        let lt_info: IAPResponse<LTResponse> = session
            .post(&format!("{}/security/lt", &self.url))
            .headers(headers.clone())
            .form(&[("lt", prior_lt)])
            .send()?
            .json()?;
        let params = LoginParams {
            lt: lt_info.result.lt,
            remember_me: false,
            dllt: "".to_string(),
            mobile: "".to_string(),
            username: username.to_string(),
            password: password.to_string(),
            captcha: "".to_string(),
        };

        let need_captcha = {
            let result: Value = session
                .post(&format!("{}/checkNeedCaptcha", &self.url))
                .headers(headers.clone())
                .query(&[("username", username)])
                .json(&json!({}))
                .send()?
                .json()?;
            result.get("needCaptcha").unwrap().as_bool().unwrap()
        };

        if need_captcha {
            todo!();
        }

        let login_result = session
            .post(&format!("{}/doLogin", &self.url))
            .headers(headers.clone())
            .form(&params)
            .send()?;

        let result_obj: Value = login_result.json()?;
        let result_code = result_obj.get("resultCode").unwrap().as_str().unwrap();
        match result_code {
            "REDIRECT" => {
                let redirect_url = result_obj.get("url").unwrap().as_str().unwrap();

                let token = &redirect_url[redirect_url.find('=').unwrap() + 1..redirect_url.len()];

                assert_ne!(token, "", "Token is empty");

                session.get(redirect_url).send()?;
                Ok(token.to_owned())
            }
            "CAPTCHA_NOTMATCH" => Err(anyhow::anyhow!("CAPTCHA_NOTMATCH")),
            "FAIL_UPNOTMATCH" => Err(anyhow::anyhow!("FAIL_UPNOTMATCH")),
            "LT_NOTMATCH" => panic!("LT_NOTMATCH"),
            _ => unimplemented!(),
        }
    }

    fn get_type(&self) -> &'static str {
        "IAP"
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use reqwest::blocking::Client;

    use super::LoginProvider;
    use super::{IAPResponse, LTResponse, IAP};

    #[test]
    fn test_lt_deserialise() {
        let response = r#"{"code":200,"message":"操作成功","result":{"_encryptSalt":"6044cb8792f0452c","_lt":"8adf66e6c0944f8da02d0befde246517","forgetPwdUrl":"/personCenter/new_password_retrieve/index.html","needCaptcha":false}}"#;
        let parsed_response: IAPResponse<LTResponse> = serde_json::from_str(response).unwrap();
        assert_eq!(parsed_response.code, 200);
        assert_eq!(parsed_response.message, "操作成功");
        assert_eq!(
            parsed_response.result.lt,
            "8adf66e6c0944f8da02d0befde246517"
        );
        assert_eq!(parsed_response.result.encrypt_salt, "6044cb8792f0452c");
        assert_eq!(
            parsed_response.result.forget_pwd_url,
            "/personCenter/new_password_retrieve/index.html"
        );
        assert_eq!(parsed_response.result.need_captcha, false);
    }

    #[test]
    fn test_iap_login() {
        let iap_url = env::var_os("IAP_URL");
        let username = env::var_os("IAP_USERNAME");
        let password = env::var_os("IAP_PASSWORD");
        if iap_url.is_none() || username.is_none() || password.is_none() {
            println!("IAP_URL, IAP_USERNAME, IAP_PASSWORD must be set. Skipping...");
            return;
        }

        let client = Client::builder()
            .cookie_store(true)
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();
        let iap = IAP {
            url: iap_url.unwrap().to_str().unwrap().to_string(),
        };

        let token = iap
            .login(
                &client,
                &username.unwrap().to_str().unwrap().to_string(),
                &password.unwrap().to_str().unwrap().to_string(),
            )
            .unwrap();

        println!("{}", token);
    }
}
