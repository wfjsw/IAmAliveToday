use crate::cpdaily::client::Client;

pub trait FirstV2 {
    fn new(client: &Client) -> Self;
    fn aes_encrypt(&self, text: &str) -> Result<&str, &str>;
    fn aes_decrypt(&self, text: &str) -> Result<&str, &str>;
    // pub fn base64_encode(&self, text: &str) -> Result<&str, Error>;
    // pub fn base64_decode(&self, text: &str) -> Result<&str, Error>;
}
