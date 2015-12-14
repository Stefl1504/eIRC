#[macro_use] extern crate log;
extern crate mio;
extern crate toml;
extern crate env_logger;
extern crate rustc_serialize;

mod config;

use std::env;
use config::Config;

fn main() {
    env_logger::init().unwrap();

    info!("Starting the application");

    let mut args = env::args();
    let config_file_name = args.nth(1).unwrap_or("eirc.toml".to_string());
    let config = Config::parse(config_file_name);

    info!("Exiting the application");
}
