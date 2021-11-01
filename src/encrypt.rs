use openssl::symm::{encrypt,Cipher};
use std::str;

pub fn des_encrypt(content:String,key:String,iv:&[u8]) -> String{
    let cipher = Cipher::des_cbc();
    let key = key.as_bytes();
    let citxt = encrypt(cipher, key, Some(iv),content.as_bytes()).unwrap();
    let s = openssl::base64::encode_block(&citxt);
    return String::from(s);
}

pub fn aes_encrypt(content:String,key:String,iv:&[u8]) -> String{
    let cipher = Cipher::aes_128_cbc();
    let key = key.as_bytes();
    let citxt = encrypt(cipher, key, Some(iv),content.as_bytes()).unwrap();
    let s = openssl::base64::encode_block(&citxt);
    return String::from(s);
}