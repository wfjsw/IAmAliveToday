pub trait FirstV2 {
    fn encrypt(&self, text: &str, key_type: KeyType) -> anyhow::Result<String>;
    fn decrypt(&self, text: &str, key_type: KeyType) -> anyhow::Result<String>;
    fn get_key(&self, key_type: KeyType) -> String;
}

pub enum KeyType {
    C,
    F,
}
