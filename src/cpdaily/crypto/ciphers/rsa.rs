use openssl::{error::ErrorStack, rsa::{Rsa, Padding}};

const CPDAILY_RSA_PUBLIC : &str = "-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQC5L0EDkXvc9DpIfDryhW3XeG2n
dTImJSnXMbl5I0Fg9gL1/JkcQLaSnRCAtCtlgdO6ux5tYVwfPzRYVRTSTwN9DsfC
JXoczEx0qkEc92P/JkBqNJf7nHNNnfjNFyAqLAp1+oAtjPT8Kv8kLq2QWjvP11AB
4N4aDfA/ZiEFKAkRyQIDAQAB
-----END PUBLIC KEY-----";

const CPDAILY_RSA_PRIVATE : &str = "-----BEGIN PRIVATE KEY-----
MIICdgIBADANBgkqhkiG9w0BAQEFAASCAmAwggJcAgEAAoGBAM358ppVDBFK/0FT
weirKUuRquFlDxOPrEOdqEYWA3pxl4GStbyBHZrUZi3hZHaA9GVOFioHsxBZXyvG
nA6Tl6H1b0Dm0J6OlIsQ/UzSuuOeVtDFepnshm9zZKWNshmWhbVBLLHGYBA9lnzw
lWZRmzM8Q6cZR9V8vbC0iI4cnjGJAgMBAAECgYAuFmc6MR1qISXMMDmLHgE3b3iU
xlABSHx7BKPKStKsaw5DZ9hSPXGqWywhx/T6rxAAOuCqtt5SIi0xVldEy7F5nPYr
iCmw8h08RpT+e7kgKvGqeKD2h5ZqngXh2sVf0bRsAfWeWbBfbfSAGpxsyjgz5hbW
xB+4StGEW0+jmh1AAQJBAOX1ncR+tuI2udqpfsosKN5IXrc2sD7jHh4s65gI6rMD
Qwpqcz5W+gZ6cyfeyo91KU5XQTe6H+n/GhjXQOTXwQECQQDlTROXQ+tgs5kAZVBT
SI0mQb8b5GYbdyVmzmxvmVstdMyacuk1zAj6AjwT6XStQHe7vLBE4SZgJ8ScvXif
P+iJAkBpRNvZJKyxt52y5J5/DGIVB4ocUvOxhiS2aZfb/FD8a8TX0s04v3YrWwi2
Or39mAO1sinPyetsIfSfZIJ3f/EBAkEAgOZAMhNrONQdGVzat8acGjpxXROa1qu2
qcE2sdGKsNXswpIASU6maSxia2scPNx1smKS0FWlBf61Bst4CEWbyQJAZT69Xm1D
4ee8RXxhS3MjSqZ0L3+yg0J6m9C9dfCt6h6mmoL4u01hk1LPby0Nkfw+Ab6TY5x/
QbHI5l5ymh+btw==
-----END PRIVATE KEY-----";

pub fn public_encrypt<'a>(data: &'a str, key: Option<&str>) -> Result<Vec<u8>, ErrorStack> {
    let rsa = Rsa::public_key_from_pem(key.unwrap_or(CPDAILY_RSA_PUBLIC).as_bytes())?;
    let mut encrypted = vec![0; rsa.size() as usize];
    rsa.public_encrypt(data.as_bytes(), &mut encrypted, Padding::PKCS1)?;
    Ok(encrypted)
}

pub fn private_decrypt(data: &[u8], key: Option<&str>) -> Result<String, ErrorStack> {
    let rsa = Rsa::private_key_from_pem(key.unwrap_or(CPDAILY_RSA_PRIVATE).as_bytes())?;
    let mut decrypted = vec![0; rsa.size() as usize];
    rsa.private_decrypt(data, &mut decrypted, Padding::PKCS1)?;
    Ok(String::from_utf8(decrypted).unwrap().trim_end_matches('\0').to_owned())
}

#[cfg(test)]
mod tests {
    use crate::cpdaily::crypto::ciphers::rsa::{private_decrypt};
    use crate::cpdaily::crypto::ciphers::base64::decode;

    #[test]
    fn test_rsa_decrypt() {
        assert_eq!("4eb81128-4741-4879-b2cf-7079ed8c5d65|7Cf7my4F|7Llv2JZZ", private_decrypt(decode("sWBzAnDXCwawQ8V3qcXmG24HqHqPjRQwo98N2ADKGO2ghA37lveE+oirR0w7EubkGZx7bsi578P+gab8FUJEGPe/S8Bx1QCrWAbdEaeBFl6IEIuzWraxSBTguVAXtN0+9dh1w1rJK9Vkd1iLa72X233zCURdXLKhgb5zEpzpVok=").unwrap().as_slice(), None).unwrap());
    }
}
