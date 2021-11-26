use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::cpdaily::client;
use crate::cpdaily::crypto::ciphers::aes::{self, getak};
use crate::cpdaily::crypto::ciphers::{base64, md5::hash, rsa};
use crate::cpdaily::crypto::traits::first_v2::{FirstV2, KeyType};
const CKEY: &str = "CNCytgOo";
const FKEY: &str = "yZtuU8Qm";
const IV: &[u8; 16] = b"\x01\x02\x03\x04\x05\x06\x07\x08\t\x01\x02\x03\x04\x05\x06\x07";

pub struct Local {
    chk: String,
    fhk: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetSecretResponse {
    pub err_code: i32,
    pub err_msg: Option<String>,
    pub data: Option<String>,
}

struct SecretNoncePair {
    pub chk: String,
    pub fhk: String,
}

fn fetch_first_v2_secrets() -> Result<GetSecretResponse, reqwest::Error> {
    let uuid = Uuid::new_v4();
    let cleartext_p = format!("{}|first_v2", uuid.to_hyphenated().to_string());
    let ciphertext_p = rsa::public_encrypt(&cleartext_p, None).unwrap();
    let encoded_p = base64::encode(&ciphertext_p);
    let s = format!(
        "p={}&2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
        encoded_p
    );

    Ok(client::unauth()?
        .post("https://mobile.campushoy.com/app/auth/dynamic/secret/getSecretKey/v-8222")
        .json(&json!({
            "p": encoded_p,
            "s": hash(&s).unwrap(),
        }))
        .send()?
        .json::<GetSecretResponse>()?)
}

fn extract_nonce_from_secret_response(response: GetSecretResponse) -> anyhow::Result<SecretNoncePair> {
        // example: {"errCode":0,"errMsg":null,"data":"sWBzAnDXCwawQ8V3qcXmG24HqHqPjRQwo98N2ADKGO2ghA37lveE+oirR0w7EubkGZx7bsi578P+gab8FUJEGPe/S8Bx1QCrWAbdEaeBFl6IEIuzWraxSBTguVAXtN0+9dh1w1rJK9Vkd1iLa72X233zCURdXLKhgb5zEpzpVok="}
    let encrypted_data = response.data.unwrap();
    let raw_data =
        rsa::private_decrypt(&base64::decode(&encrypted_data).unwrap(), None).unwrap();
    let splits: Vec<&str> = raw_data.split('|').collect();
    if splits.len() != 3 {
        return Err(anyhow::anyhow!("Unexpected number of splits in secret response: {}", splits.len()));
    }
    let chk = splits[1].to_string();
    let fhk = splits[2].to_string();
    Ok(SecretNoncePair { chk, fhk })
}

impl Local {
    fn from_server_response(response: GetSecretResponse) -> Self {
        let nonce_pair = extract_nonce_from_secret_response(response).unwrap();

        crate::logger::log(sentry::Breadcrumb {
            category: Some("crypto.first_v2".to_string()),
            message: Some("fetched first_v2 crypto keys".to_string()),
            level: sentry::Level::Info,
            data: {
                let mut bt = BTreeMap::new();
                bt.insert(
                    "chk".to_string(),
                    serde_json::to_value(nonce_pair.chk.clone()).unwrap(),
                );
                bt.insert(
                    "fhk".to_string(),
                    serde_json::to_value(nonce_pair.fhk.clone()).unwrap(),
                );
                bt
            },
            ..Default::default()
        });

        Local { chk: nonce_pair.chk, fhk: nonce_pair.fhk }
    }
}

impl Local {
    pub fn new() -> Self {
        // fetch from getSecret

        let secrets = fetch_first_v2_secrets().unwrap();

        crate::logger::log(sentry::Breadcrumb {
            category: Some("crypto.first_v2".to_string()),
            message: Some("getSecret returns".to_string()),
            level: sentry::Level::Info,
            data: {
                let mut bt = BTreeMap::new();
                bt.insert(
                    "response".to_string(),
                    serde_json::to_value(secrets.clone()).unwrap(),
                );
                bt
            },
            ..Default::default()
        });

        assert_eq!(
            secrets.err_code,
            0,
            "getSecret returns non-zero: {}",
            secrets.err_msg.unwrap_or("Unknown".to_string())
        );

        Local::from_server_response(secrets)
    }
}

impl FirstV2 for Local {
    fn encrypt(&self, text: &str, key_type: KeyType) -> anyhow::Result<String> {
        let key = self.get_key(key_type);

        let ciphertext = aes::encrypt(text.as_bytes(), key.as_bytes(), IV).unwrap();
        Ok(base64::encode(&ciphertext))
    }

    fn decrypt(&self, text: &str, key_type: KeyType) -> anyhow::Result<String> {
        let key = self.get_key(key_type);

        let ciphertext = base64::decode(text)?;
        let cleartext = aes::decrypt(&ciphertext, key.as_bytes(), IV)?;
        Ok(String::from_utf8(cleartext).unwrap())
    }

    fn get_key(&self, key_type: KeyType) -> String {
        match key_type {
            KeyType::C => getak(CKEY, &self.chk),
            KeyType::F => getak(FKEY, &self.fhk),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GetSecretResponse, Local, fetch_first_v2_secrets, extract_nonce_from_secret_response};

    #[test]
    fn test_parse_first_v2_get_secret_response() {
        let resp: GetSecretResponse = serde_json::from_str(r#"{"errCode":0,"errMsg":null,"data":"sWBzAnDXCwawQ8V3qcXmG24HqHqPjRQwo98N2ADKGO2ghA37lveE+oirR0w7EubkGZx7bsi578P+gab8FUJEGPe/S8Bx1QCrWAbdEaeBFl6IEIuzWraxSBTguVAXtN0+9dh1w1rJK9Vkd1iLa72X233zCURdXLKhgb5zEpzpVok="}"#).unwrap();
        let key_object = extract_nonce_from_secret_response(resp.clone()).unwrap();
        assert_eq!("7Cf7my4F", key_object.chk);
        assert_eq!("7Llv2JZZ", key_object.fhk);
        let provider = Local::from_server_response(resp);
        assert_eq!("7Cf7my4F", provider.chk);
        assert_eq!("7Llv2JZZ", provider.fhk);
    }

    #[test]
    fn test_fetch_first_v2_secrets() {
        let secrets = fetch_first_v2_secrets().unwrap();
        assert_eq!(0, secrets.err_code);
        assert!(secrets.err_msg.is_none());
        let nonce_pair = extract_nonce_from_secret_response(secrets).unwrap();
        println!("CHK: {}", nonce_pair.chk);
        println!("FHK: {}", nonce_pair.fhk);
    }
}
