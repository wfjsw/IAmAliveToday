mod encrypt;
mod config;
use std::str;
use std::io;
use clap::{App, Arg};

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
    let AESiv = b"\x01\x02\x03\x04\x05\x06\x07\x08\t\x01\x02\x03\x04\x05\x06\x07";
    let AESKey = "ytUQ7l2ZZu8mLvJZ";
    let DESiv = b"\x01\x02\x03\x04\x05\x06\x07\x08";
    let DESKey = "b3L26XNL";
    let app = App::new(APP_NAME)
        .version(VERSION)
        .about(DESCRIPTION);
    let matches = app.get_matches();
}
