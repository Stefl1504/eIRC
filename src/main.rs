#[macro_use]
extern crate log;
#[macro_use]
extern crate shared_library;

extern crate toml;
extern crate libc;
extern crate bufstream;
extern crate env_logger;
extern crate rustc_serialize;

mod irc;
mod config;

use std::env;
use config::Config;
use irc::Irc;

fn main() {
    env_logger::init().unwrap();

    info!("Starting the application");

    let mut args = env::args();
    let config_file_name = args.nth(1).unwrap_or("eirc.toml".to_string());
    let config = Config::parse(config_file_name);

    let irc = Irc::new(config);
    irc.run();

    info!("Exiting the application");
}
