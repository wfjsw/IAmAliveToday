use super::client::Client;

pub mod cas;
pub mod iap;
pub mod rsa;

pub trait LoginProvider {
    fn login(&self, session: &Client, username: &str, password: &str) -> anyhow::Result<()>;
}
