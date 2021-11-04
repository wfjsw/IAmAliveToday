use openssl::{error::ErrorStack, hash::{hash as openssl_hash, MessageDigest}};
use hex::encode as hex_encode;

pub fn hash(input: &str) -> Result<String, ErrorStack> {
    // let mut out = String::with_capacity(32);
    let result = openssl_hash(MessageDigest::md5(), input.as_bytes())?;
    Ok(hex_encode(result))
}

#[cfg(test)]
mod tests {
    use crate::cpdaily::crypto::ciphers::md5::hash;

    #[test]
    fn test_md5() {
        assert_eq!("bcf7dcea849a6efc9d3d5ca1bfd14b14", hash("thisismd5test").unwrap());
    }
}
