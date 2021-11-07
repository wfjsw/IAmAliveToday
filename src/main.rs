mod cpdaily;
mod config;
mod actions;

use std::{env, str};
use cpdaily::client;
use getopts::{Matches, Options};

use cpdaily::crypto::traits::first_v2::FirstV2;
use cpdaily::crypto::providers::first_v2;
use config::Action;

fn main() {
    #[cfg(build = "release")]
    #[cfg(feature = "sentry")]
    let _guard = {
        let dsn = env!("SENTRY_DSN");

        sentry::init((dsn.unwrap(), sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        }))
    };
    let cwd = env::current_dir().unwrap();
    let default_config_path = cwd.join("config.yml");

    let matches = parse_options();

    if matches.opt_present("h") {
        return;
    }
    
    // Load Config
    let config_file_path = matches.opt_str("c").unwrap_or(default_config_path.to_str().unwrap().to_string());
    let config = config::load_config(&config_file_path).expect("Config file not found");

    // Initialize crypto providers
    let first_v2_provider = first_v2::Local::new();

    // Fetch tenant list
    let tenant_list = cpdaily::get_all_tenants().unwrap();

    // For each user
    for user in config.users {
        let client = client::new(&user).unwrap();
        let tenant = cpdaily::match_school_from_tenant_list(&tenant_list, &user.school).unwrap();
        let login_provider = tenant.create_login();
        login_provider.login(&client, &user.username, &user.password).unwrap();

        let tenant_detail = tenant.get_info().unwrap();

        let base_url = tenant_detail.get_url().unwrap();
        for action in user.actions {
            match action {
                Action::CounselorFormFill(form_fill) => {
                    actions::counselor_form_fill::perform(&client, &base_url, &form_fill).unwrap()
                }
                _ => unimplemented!("Unknown action type.")
            };
        }
    }
}


fn parse_options() -> Matches {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("c", "config", "config file (default: config.yml)", "PATH");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
    }
    matches
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}
