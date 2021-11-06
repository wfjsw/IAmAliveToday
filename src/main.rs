mod cpdaily;
mod config;
mod actions;

use std::str;
use clap::{App, Arg};

use cpdaily::crypto::traits::first_v2::FirstV2;
use cpdaily::crypto::providers::first_v2;
use cpdaily::client::Client;
use config::Action;

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
    let config = config::load_config(config_file_path).expect("Config file not found");

    // Initialize crypto providers
    let first_v2_provider = first_v2::Local::new();

    // Fetch tenant list
    let tenant_list = cpdaily::get_all_tenants().unwrap();

    // For each user
    for user in config.users {
        let client = Client::new(None, &user);
        let tenant = cpdaily::match_school_from_tenant_list(&tenant_list, &user.school).unwrap();
        let login_provider = tenant.create_login();
        login_provider.login(&client, &user.username, &user.password).unwrap();

        let tenant_detail = tenant.get_info().unwrap();

        for action in user.actions {
            match action {
                Action::CounselorFormFill(form_fill) => {
                    actions::counselor_form_fill::perform(&client, &form_fill)
                }
                _ => unimplemented!("Unknown action type.")
            }
        }
    }
}
