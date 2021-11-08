pub trait FirstV2 {
    fn encrypt(&self, text: &str, key_type: KeyType) -> anyhow::Result<String>;
    fn decrypt(&self, text: &str, key_type: KeyType) -> anyhow::Result<String>;
}

pub enum KeyType {
    C,
    F,
}
