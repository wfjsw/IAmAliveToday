mod encrypt;
mod cpdaily;
mod config;

use std::str;
use clap::{App, Arg};

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
    let AESiv = b"\x01\x02\x03\x04\x05\x06\x07\x08\t\x01\x02\x03\x04\x05\x06\x07";
    let AESKey = "ytUQ7l2ZZu8mLvJZ";
    let DESiv = b"\x01\x02\x03\x04\x05\x06\x07\x08";
    let DESKey = "b3L26XNL";
    //Keys above for test temporarily
    
    let cwd = std::env::current_dir().unwrap();
    let default_config_path = cwd.join("config.yml");
    let app = App::new(APP_NAME)
        .version(VERSION)
        .about(DESCRIPTION)
        .arg(Arg::new("config").short('c').long("config").default_value(default_config_path.to_str().unwrap()).takes_value(true));
        
    let matches = app.get_matches();
    
    println!("{}",cpdaily::GetHostByName(String::from("青岛大学")));
    

}
