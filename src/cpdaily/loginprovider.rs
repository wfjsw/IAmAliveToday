pub mod cas;
pub mod iap;
pub mod rsa;

pub trait LoginProvider {
    fn new(url: &str) -> Self;
    fn login(&self, username: &str, password: &str) -> Result<String, String>;
}
