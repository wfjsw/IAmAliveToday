use serde_json::{json, Value};
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

impl Local {
    fn from_server_response(response: Value) -> Self {
        // example: {"errCode":0,"errMsg":null,"data":"sWBzAnDXCwawQ8V3qcXmG24HqHqPjRQwo98N2ADKGO2ghA37lveE+oirR0w7EubkGZx7bsi578P+gab8FUJEGPe/S8Bx1QCrWAbdEaeBFl6IEIuzWraxSBTguVAXtN0+9dh1w1rJK9Vkd1iLa72X233zCURdXLKhgb5zEpzpVok="}
        let encrypted_data = response["data"].as_str().unwrap();
        let raw_data =
            rsa::private_decrypt(&base64::decode(encrypted_data).unwrap(), None).unwrap();
        let splits: Vec<&str> = raw_data.split('|').collect();

        let chk = splits[1].to_string();
        let fhk = splits[2].to_string();

        #[cfg(feature = "sentry")]
        sentry::add_breadcrumb(sentry::Breadcrumb {
            category: Some("crypto.first_v2".to_string()),
            message: Some("fetched first_v2 crypto keys".to_string()),
            level: sentry::Level::Info,
            data: {
                let mut bt = BTreeMap::new();
                bt.insert(
                    "chk".to_string(),
                    serde_json::to_value(chk.clone()).unwrap(),
                );
                bt.insert(
                    "fhk".to_string(),
                    serde_json::to_value(fhk.clone()).unwrap(),
                );
                bt
            },
            ..Default::default()
        });

        Local { chk, fhk }
    }
}

impl Local {
    pub fn new() -> Self {
        // fetch from getSecret

        let uuid = Uuid::new_v4();
        let cleartext_p = format!("{}|first_v2", uuid.to_hyphenated().to_string());
        let ciphertext_p = rsa::public_encrypt(&cleartext_p, None).unwrap();
        let encoded_p = base64::encode(&ciphertext_p);
        let s = format!(
            "p={}&2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
            encoded_p
        );

        // let result = post_json("/app/auth/dynamic/secret/getSecretKey/v-8222", json!({
        //     "p": encoded_p,
        //     "s": hash(&s).unwrap(),
        // })).unwrap();
        let result = client::unauth()
            .unwrap()
            .post("https://mobile.campushoy.com/app/auth/dynamic/secret/getSecretKey/v-8222")
            .json(&json!({
                "p": encoded_p,
                "s": hash(&s).unwrap(),
            }))
            .send()
            .unwrap()
            .json()
            .unwrap();

        Local::from_server_response(result)
    }
}

impl FirstV2 for Local {
    fn encrypt(&self, text: &str, key_type: KeyType) -> anyhow::Result<String> {
        let key = self.get_key(key_type);

        let ciphertext = aes::encrypt(text.as_bytes(), &key.as_bytes(), IV).unwrap();
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
    use crate::cpdaily::crypto::providers::first_v2::Local;
    use serde_json::json;

    #[test]
    fn test_parse_first_v2_get_secret_response() {
        let key_object = Local::from_server_response(
            json!({"errCode":0,"errMsg":null,"data":"sWBzAnDXCwawQ8V3qcXmG24HqHqPjRQwo98N2ADKGO2ghA37lveE+oirR0w7EubkGZx7bsi578P+gab8FUJEGPe/S8Bx1QCrWAbdEaeBFl6IEIuzWraxSBTguVAXtN0+9dh1w1rJK9Vkd1iLa72X233zCURdXLKhgb5zEpzpVok="}),
        );
        assert_eq!("7Cf7my4F", key_object.chk);
        assert_eq!("7Llv2JZZ", key_object.fhk);
    }
}
