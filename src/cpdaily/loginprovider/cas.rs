use reqwest::blocking::Client;

use super::LoginProvider;

pub struct CAS {
    pub url: String,
}

impl LoginProvider for CAS {
    // TODO: remove this lint hint when implementing this
    #[allow(unused_variables)]
    fn login(&self, session: &Client, username: &str, password: &str) -> anyhow::Result<String> {
        todo!()
    }
}
