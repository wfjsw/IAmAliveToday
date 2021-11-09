use reqwest::blocking::Client;

use super::LoginProvider;

pub struct RSA {
    pub url: String,
}

impl LoginProvider for RSA {
    // TODO: remove this lint hint when implementing this
    #[allow(unused_variables)]
    fn login(&self, session: &Client, username: &str, password: &str) -> anyhow::Result<String> {
        todo!()
    }

    fn get_type(&self) -> &'static str {
        "RSA"
    }
}
