use std::fs::File;
use std::io::prelude::*;
use toml::{Parser, Value, Decoder};
use toml;

/// Config holds global static configuration
/// Once read from disk, it is distributed to all threads as an immutable structure
#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct Config  {

    /// The configurations related to networking
    pub server: ServerConfig,
}

/// The configurations related to networking
#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct ServerConfig  {
    /// The hostname we connect to
    pub hostname: String,

    /// The port we connect to
    pub port: u16,

    /// Channels the bot is located in on startup
    pub channels: Vec<String>
}

impl ServerConfig {
    pub fn new() -> ServerConfig {
        ServerConfig {
            hostname: "irc.rizon.net".to_string(),
            port: 6667,
            channels: vec!["#eirc".to_string()]
        }
    }
}

impl Config {

    /// Returns a default configuration if we don't have/find a
    /// config file
    pub fn new() -> Config {
        Config {
            server: ServerConfig::new()
        }
    }

    /// Attempt to load and parse the config file into our Config struct.
    /// If a file cannot be found, return a default Config.
    /// If we find a file but cannot parse it, panic
    pub fn parse(path: String) -> Config {
        let mut config_toml = String::new();

        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(_)  => {
                error!("Could not find config file, using default!");
                return Config::new();
            }
        };

        file.read_to_string(&mut config_toml)
                .unwrap_or_else(|err| panic!("Error while reading config: [{}]", err));

        let mut parser = Parser::new(&config_toml);
        let toml = parser.parse();

        if toml.is_none() {
            for err in &parser.errors {
                let (loline, locol) = parser.to_linecol(err.lo);
                let (hiline, hicol) = parser.to_linecol(err.hi);
                println!("{}:{}:{}-{}:{} error: {}",
                         path, loline, locol, hiline, hicol, err.desc);
            }
            panic!("Exiting!");
        }

        let config = Value::Table(toml.unwrap());

        toml::decode(config).unwrap()
    }
}

