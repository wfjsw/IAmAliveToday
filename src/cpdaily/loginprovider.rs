use reqwest::blocking::Client;

pub mod cas;
pub mod iap;
pub mod rsa;

pub trait LoginProvider {
    fn get_type(&self) -> &'static str;
    fn login(&self, session: &Client, username: &str, password: &str) -> anyhow::Result<String>;
}
