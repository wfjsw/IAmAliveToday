use curl::easy::List;
use serde_json::{Value, json};
use uuid::Uuid;

use crate::cpdaily::client::Client;
use crate::cpdaily::crypto::traits::first_v2::FirstV2;
use crate::cpdaily::crypto::ciphers::{rsa, base64, md5::hash};
const ckey: &'static str = "CNCytgOo";
const fkey: &'static str = "yZtuU8Qm";

pub struct Local {
    chk: String,
    fhk: String,
}

impl Local {
    fn from_server_response(response: Value) -> Self {
        // example: {"errCode":0,"errMsg":null,"data":"sWBzAnDXCwawQ8V3qcXmG24HqHqPjRQwo98N2ADKGO2ghA37lveE+oirR0w7EubkGZx7bsi578P+gab8FUJEGPe/S8Bx1QCrWAbdEaeBFl6IEIuzWraxSBTguVAXtN0+9dh1w1rJK9Vkd1iLa72X233zCURdXLKhgb5zEpzpVok="}
        let encrypted_data = response["data"].as_str().unwrap();
        let raw_data = rsa::private_decrypt(&base64::decode(encrypted_data).unwrap(), None).unwrap();
        let splits: Vec<&str> = raw_data.split('|').collect();
        Local { 
            chk: splits[1].to_owned(), 
            fhk: splits[2].to_owned(), 
        }
    }
}

impl FirstV2 for Local {
    fn new() -> Self {
        // fetch from getSecret

        let uuid = Uuid::new_v4();
        let cleartext_p = format!("{}|first_v2", uuid.to_hyphenated().to_string());
        let ciphertext_p = rsa::public_encrypt(&cleartext_p, None).unwrap();
        let encoded_p = base64::encode(&ciphertext_p);
        let s = format!("p={}&2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824", encoded_p);

        let result = post_json("/app/auth/dynamic/secret/getSecretKey/v-8222", json!({
            "p": encoded_p,
            "s": hash(&s).unwrap(),
        })).unwrap();

        Local::from_server_response(result)
    }

    fn encrypt(&self, text: &str) -> Result<&str, &str> {
        todo!()
    }

    fn decrypt(&self, text: &str) -> Result<&str, &str> {
        todo!()
    }
}

fn post_json(url: &str, data: Value) -> anyhow::Result<Value> {
    let mut headers = List::new();
    headers.append("Accept: application/json").unwrap();
    headers.append("User-Agent: Mozilla/5.0 (Linux; U; Android 8.1.0; zh-cn; BLA-AL00 Build/HUAWEIBLA-AL00) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/57.0.2987.132 MQQBrowser/8.9 Mobile Safari/537.36").unwrap();
    let data = Client::perform(url, Some(headers), Some("GET"), Some(&data.to_string()), true, None)?.1;
    let parsed = serde_json::from_str(&data).expect("Parsing JSON");
    Ok(parsed)
}



#[cfg(test)]
mod tests {
    use serde_json::json;
    use crate::cpdaily::crypto::providers::first_v2::Local;

    #[test]
    fn test_parse_first_v2_get_secret_response() {
        let key_object = Local::from_server_response(json!({"errCode":0,"errMsg":null,"data":"sWBzAnDXCwawQ8V3qcXmG24HqHqPjRQwo98N2ADKGO2ghA37lveE+oirR0w7EubkGZx7bsi578P+gab8FUJEGPe/S8Bx1QCrWAbdEaeBFl6IEIuzWraxSBTguVAXtN0+9dh1w1rJK9Vkd1iLa72X233zCURdXLKhgb5zEpzpVok="}));
        assert_eq!("7Cf7my4F", key_object.chk);
        assert_eq!("7Llv2JZZ", key_object.fhk);
    }
}
