use serde_json::json;
use uuid::Uuid;

use crate::cpdaily::client::Client;
use crate::cpdaily::crypto::traits::first_v2::FirstV2;
use crate::cpdaily::crypto::ciphers::{rsa::public_encrypt, base64::encode, md5::hash};
const ckey: &'static str = "CNCytgOo";
const fkey: &'static str = "yZtuU8Qm";

pub struct Local {
    chk: String,
    fhk: String,
}

impl FirstV2 for Local {
    fn new(client: &Client) -> Self {
        // fetch from getSecret

        let uuid = Uuid::new_v4();
        let cleartext_p = format!("{}|first_v2", uuid.to_hyphenated().to_string());
        let ciphertext_p = public_encrypt(&cleartext_p, None).unwrap();
        let encoded_p = encode(&ciphertext_p);
        let s = format!("p={}&2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824", encoded_p);

        let result = client.post_json("/app/auth/dynamic/secret/getSecretKey/v-8222", json!({
            "p": encoded_p,
            "s": hash(&s).unwrap(),
        })).unwrap();


        
        Local {
            chk: ckey.to_string(),
            fhk: fkey.to_string(),
        }
    }

    fn aes_encrypt(&self, text: &str) -> Result<&str, &str> {
        todo!()
    }

    fn aes_decrypt(&self, text: &str) -> Result<&str, &str> {
        todo!()
    }
}
