mod config;
use clap::{App, Arg};

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
    let app = App::new(APP_NAME)
        .version(VERSION)
        .about(DESCRIPTION);
    let matches = app.get_matches();
}
