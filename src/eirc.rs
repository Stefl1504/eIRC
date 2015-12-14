extern crate toml;
extern crate rustc_serialize;

mod config;

use std::env;
use config::Config;

fn main() {
    let mut args = env::args();
    let config_file_name = args.nth(1).unwrap();
    let config = Config::parse(config_file_name);
}
