use openssl::{base64::{decode_block, encode_block}, error::ErrorStack};

pub fn encode(input: &[u8]) -> String {
    let mut output = String::with_capacity(input.len() * 5 / 3);
    let chunks = input.chunks(15 * 3).enumerate();
    let len_chunks = chunks.len();
    chunks.for_each(|(i, chunk)| {
        output.push_str(&encode_block(&chunk));
        if i < len_chunks - 1 {
            output.push(' ');
        }
    });

    output
}

pub fn decode(input: &str) -> Result<Vec<u8>, ErrorStack> {
    decode_block(&input.replace(" ", ""))
}

#[cfg(test)]
mod tests {
    use crate::cpdaily::crypto::ciphers::base64::{encode, decode};

    #[test]
    fn test_base64() {
        let c1 = b"hello";
        let e1 = "aGVsbG8=";
        let c2 = b"8ff26b249dd7430860e54b1e4b7a21a9e2199734eba0554051913a463236419aabbd0e0759149526cce204aba30b1a09634c062636c673a8afa434cce50df99fefc3bd22aa766995f1fd09ca05dcb95950031f6d0b2af4b8ea3f943d1572d969ec5132dc3d1626c64849de64bdbf62d6c6c5acaca15d96b11e8cf53c0ebd495f";
        let e2 = "OGZmMjZiMjQ5ZGQ3NDMwODYwZTU0YjFlNGI3YTIxYTllMjE5OTczNGViYTA1 NTQwNTE5MTNhNDYzMjM2NDE5YWFiYmQwZTA3NTkxNDk1MjZjY2UyMDRhYmEz MGIxYTA5NjM0YzA2MjYzNmM2NzNhOGFmYTQzNGNjZTUwZGY5OWZlZmMzYmQy MmFhNzY2OTk1ZjFmZDA5Y2EwNWRjYjk1OTUwMDMxZjZkMGIyYWY0YjhlYTNm OTQzZDE1NzJkOTY5ZWM1MTMyZGMzZDE2MjZjNjQ4NDlkZTY0YmRiZjYyZDZj NmM1YWNhY2ExNWQ5NmIxMWU4Y2Y1M2MwZWJkNDk1Zg==";

        assert_eq!(e1, encode(c1));
        assert_eq!(e2, encode(c2));
        assert_eq!(c1, decode(e1).unwrap().as_slice());
        assert_eq!(c2, decode(e2).unwrap().as_slice());
    }
}
