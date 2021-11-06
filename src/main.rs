mod encrypt;
mod cpdaily;
mod config;

use std::str;
use clap::{App, Arg};

use cpdaily::crypto::traits::first_v2::FirstV2;
use crate::cpdaily::crypto::providers::first_v2;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
    let cwd = std::env::current_dir().unwrap();
    let default_config_path = cwd.join("config.yml");
    let app = App::new(APP_NAME)
        .version(VERSION)
        .about(DESCRIPTION)
        .arg(Arg::new("config").short('c').long("config").default_value(default_config_path.to_str().unwrap()).takes_value(true));
        
    let matches = app.get_matches();
    
    // Load Config
    let config_file_path = matches.value_of("config").unwrap();
    let config = config::load_config(config_file_path);

    // Initialize first_v2 crypto provider
    let first_v2_provider = first_v2::Local::new();

    // For each user
    for user in config.users {
        
    }
}
