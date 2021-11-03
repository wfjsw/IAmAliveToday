use openssl::error::ErrorStack;

use openssl::symm::{Cipher, encrypt as cipher_encrypt, decrypt as cipher_decrypt};

pub fn encrypt(plaintext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, ErrorStack> {
    let cipher = Cipher::aes_128_cbc();
    cipher_encrypt(cipher, key, Some(iv), plaintext)
}

pub fn decrypt(ciphertext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, ErrorStack> {
    let cipher = Cipher::aes_128_cbc();
    cipher_decrypt(cipher, key, Some(iv), ciphertext)
}
