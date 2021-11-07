use reqwest::blocking::Client;

use super::LoginProvider;

pub struct CAS {
    pub url: String,
}

impl LoginProvider for CAS {
    fn login(&self, session: &Client, username: &str, password: &str) -> anyhow::Result<()> {
        todo!()
    }
}
