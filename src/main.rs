mod actions;
mod config;
mod cpdaily;
mod logger;

use cpdaily::client;
use getopts::{Matches, Options};
use serde_json::json;
use std::{collections::BTreeMap, env, str};

use config::Action;
use cpdaily::crypto::providers::first_v2;

fn main() {
    #[cfg(feature = "telemetry")]
    let _guard = {
        let dsn = env!("SENTRY_DSN");
        let nightly = option_env!("NIGHTLY");

        sentry::init((
            dsn,
            sentry::ClientOptions {
                release: match nightly {
                    Some(sha) => Some(sha.into()),
                    None => sentry::release_name!(),
                },
                debug: cfg!(build = "debug"),
                ..Default::default()
            },
        ))
    };
    let cwd = env::current_dir().unwrap();
    let default_config_path = cwd.join("config.yml");

    let matches = parse_options();

    if matches.opt_present("h") {
        return;
    }

    // Load Config
    let config_file_path = matches
        .opt_str("c")
        .unwrap_or_else(|| default_config_path.to_str().unwrap().to_string());
    let config = config::load_config(&config_file_path).expect("Config file not found");

    logger::log(sentry::Breadcrumb {
        category: Some("config".to_string()),
        message: Some("loaded config file".to_string()),
        data: {
            let mut bt = BTreeMap::new();
            for u in &config.users {
                bt.insert(
                    format!("user-{}-{}", &u.school, &u.username),
                    json!({
                        "school": u.school,
                        "username": u.username,
                        "address": u.address,
                        "device_info": u.device_info,
                        "actions": u.actions,
                    }),
                );
            }
            bt
        },
        level: sentry::Level::Info,
        ..Default::default()
    });

    // Initialize crypto providers
    let first_v2_provider = first_v2::Local::new();

    // Fetch tenant list
    let tenant_list = cpdaily::get_all_tenants().unwrap();

    // For each user
    for user in &config.users {
        logger::log(sentry::Breadcrumb {
            category: Some("bus".to_string()),
            message: Some(format!("start user {}:{}", &user.school, &user.username)),
            level: sentry::Level::Info,
            ..Default::default()
        });

        let client = client::new(user).unwrap();
        let tenant = cpdaily::match_school_from_tenant_list(&tenant_list, &user.school).unwrap();

        logger::log(sentry::Breadcrumb {
            category: Some("tenant_service".to_string()),
            message: Some(format!(
                "matched \"{}\" to tenant \"{}\"",
                &user.school, &tenant.name
            )),
            level: sentry::Level::Info,
            ..Default::default()
        });

        let login_provider = tenant.create_login();

        logger::log(sentry::Breadcrumb {
            category: Some("login".to_string()),
            message: Some(format!("use login provider {}", login_provider.get_type())),
            level: sentry::Level::Info,
            ..Default::default()
        });

        login_provider
            .login(&client, &user.username, &user.password)
            .unwrap();

        logger::log(sentry::Breadcrumb {
            category: Some("login".to_string()),
            message: Some("logged in".to_string()),
            level: sentry::Level::Info,
            ..Default::default()
        });

        let tenant_detail = tenant.get_info().unwrap();

        let base_url = tenant_detail.get_url().unwrap();

        logger::log(sentry::Breadcrumb {
            category: Some("bus".to_string()),
            message: Some(format!("set base url to \"{}\"", &base_url)),
            level: sentry::Level::Info,
            ..Default::default()
        });

        for action in &user.actions {
            match action {
                Action::CounselorFormFill(form_fill) => actions::counselor_form_fill::perform(
                    &client,
                    &base_url,
                    form_fill,
                    user,
                    &first_v2_provider,
                )
                .unwrap(),
            };
        }

        logger::log(sentry::Breadcrumb {
            category: Some("bus".to_string()),
            message: Some(format!("end user {}:{}", &user.school, &user.username)),
            level: sentry::Level::Info,
            ..Default::default()
        });
    }
}

fn parse_options() -> Matches {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("c", "config", "config file (default: config.yml)", "PATH");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
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
