use reqwest::blocking::Client;

use super::LoginProvider;

pub struct RSA {
    pub url: String,
}

impl LoginProvider for RSA {
    fn login(&self, session: &Client, username: &str, password: &str) -> anyhow::Result<()> {
        todo!()
    }
}
