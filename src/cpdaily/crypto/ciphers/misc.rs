pub fn getak(key: &str, hk: &str) -> String {
    let mut out = String::with_capacity(key.len() + hk.len());
    for i in (0..key.len()).step_by(2) {
        out.push(key.chars().nth(i).unwrap());
    }
    for i in (0..hk.len()).step_by(2) {
        out.push(hk.chars().nth(i).unwrap());
    }
    for i in (1..key.len()).step_by(2) {
        out.push(key.chars().nth(i).unwrap());
    }
    for i in (1..hk.len()).step_by(2) {
        out.push(hk.chars().nth(i).unwrap());
    }
    out
}

#[cfg(test)]
mod tests {
    use crate::cpdaily::crypto::ciphers::misc::getak;

    #[test]
    fn test_getak() {
        assert_eq!("ytUQ7l2ZZu8mLvJZ", getak("yZtuU8Qm", "7Llv2JZZ"));
        assert_eq!("CCtO7fm4NygoC7yF", getak("CNCytgOo", "7Cf7my4F"))
    }
}
