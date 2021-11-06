pub trait FirstV2 {
    fn new() -> Self;
    fn encrypt(&self, text: &str) -> Result<&str, &str>;
    fn decrypt(&self, text: &str) -> Result<&str, &str>;
}
