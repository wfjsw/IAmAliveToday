use openssl::{error::ErrorStack, symm::{Cipher, encrypt as cipher_encrypt, decrypt as cipher_decrypt}};

const CPDAILY_DES_KEY: &[u8; 8] = b"b3L26XNL";
const CPDAILY_DES_IV: &[u8; 8] = b"\x01\x02\x03\x04\x05\x06\x07\x08";

pub fn encrypt(plaintext: &str, key: Option<&[u8]>, iv: Option<&[u8]>) -> Result<Vec<u8>, ErrorStack> {
    let cipher = Cipher::des_cbc();
    cipher_encrypt(cipher, key.unwrap_or(CPDAILY_DES_KEY), Some(iv.unwrap_or(CPDAILY_DES_IV)) ,plaintext.as_bytes())
}

pub fn decrypt(ciphertext: &[u8], key: Option<&[u8]>, iv: Option<&[u8]>) -> Result<Vec<u8>, ErrorStack> {
    let cipher = Cipher::des_cbc();
    cipher_decrypt(cipher, key.unwrap_or(CPDAILY_DES_KEY), Some(iv.unwrap_or(CPDAILY_DES_IV)), ciphertext)
}
