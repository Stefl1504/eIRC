use std::io::prelude::*;
use std::io::Error;
use std::net::TcpStream;
use std::str::FromStr;
use std::fs;
use std::fs::FileType;
use std::fs::DirEntry;
use std::path::Path;

use bufstream::BufStream;
use shared_library::*;
use libc::c_int;

use config::Config;

pub struct Irc
{
    config: Config,
    connected: bool,
}

struct IrcMessage {
    raw_message: String,
    prefix: String,
    command: String,
    params: Vec<String>
}

impl IrcMessage {
    fn new(raw_msg: &str) -> IrcMessage {
        let mut raw: &str = raw_msg;
        let mut new_msg = IrcMessage {
            raw_message: String::new(),
            prefix: String::new(),
            command: String::new(),
            params: Vec::with_capacity(15)
        };

        new_msg.raw_message = String::from_str(raw).unwrap();
        if raw.starts_with(":") {
            let first_whitespace: u32 = match raw.find(' ') {
                Some(x) => x as u32,
                None => 0u32
            };
            new_msg.prefix = String::from_str(raw.substr(1, first_whitespace - 1)).unwrap();
            raw = raw.substr(first_whitespace + 1, raw.len() as u32 - (first_whitespace+1));
        }

        if raw.contains(' ') {
            let space_index = match raw.find(' ') {
                Some(x) => x as u32,
                None => 0u32,
            };
            new_msg.command = String::from(raw.substr(0, space_index));
            raw = raw.substr(space_index + 1, raw.len() as u32 - (space_index + 1));

            // Parse parameters
            let mut parameters: Vec<String> = Vec::new();
            while raw != "" {
                if raw.starts_with(":") {
                    parameters.push(String::from(raw.substr(1, raw.len() as u32 - 1)));
                    break;
                }

                if !raw.contains(' ') {
                    parameters.push(String::from(raw));
                    break;
                }
                let space_index = match raw.find(' ') {
                    Some(x) => x as u32,
                    None => 0u32
                };
                parameters.push(String::from(raw.substr(0, space_index)));
                raw = raw.substr(space_index + 1, raw.len() as u32 - (space_index + 1));
            }
            new_msg.params = parameters;
        }

        return new_msg;
    }
}

trait Substring {
    fn substr(&self, start_index: u32, length: u32) -> &str;
}

impl Substring for str {
    fn substr(&self, start_index: u32, length: u32) -> &str {
        return &self[start_index as usize .. start_index as usize + length as usize];
    }
}

shared_library!(IrcPlugin,
    pub fn command() -> c_int,
);

impl Irc
{
    pub fn new(config: Config) -> Irc
    {
        return Irc { config: config, connected: false }
    }


    pub fn run(mut self: Self)
    {
        self.connected = false;

        let paths = fs::read_dir("./").unwrap();

        for path in paths {
            let path_unwrapped = path.unwrap();

            match path_unwrapped.path().extension() {
                Some(name) => if name.to_str().unwrap() != "dll" && name.to_str().unwrap() != "so" { continue; },
                None => continue
            }

            info!("Found plugin {}", path_unwrapped.path().display());

            let test = IrcPlugin::open(path_unwrapped.path().as_path()).unwrap();

            unsafe { (test.command)(); }
        }

        // top kek this line
        match TcpStream::connect(&format!("{}:{}", self.config.server.hostname, self.config.server.port)[..]) {
            Ok(tcp_stream) => {
                info!("Connected");
                self.connected = true;

                let mut stream = BufStream::new(&tcp_stream);
                let mut buffer = String::new();

                send_raw_message(&mut stream, &format!("NICK {}", self.config.server.nickname));
                send_raw_message(&mut stream, &format!("USER {} 0 * :eIRC bot", self.config.server.username));
                send_raw_message(&mut stream, &format!("JOIN {}", self.config.server.channels.join(" ")));

                while stream.read_line(&mut buffer).unwrap() > 0 {
                    let message = IrcMessage::new(&buffer);
                    info!(">> {}", message.raw_message.trim());

                    if message.command == "PING" {
                        let mut reply = String::from("PONG :");
                        reply = reply + &message.params[0];
                        send_raw_message(&mut stream, &reply);
                    }

                    if message.command == "PRIVMSG" {
                        if message.params[1].starts_with(".ping") {
                            let mut reply = String::from("PRIVMSG ");
                            reply = reply + &message.params[0];
                            reply = reply + " :PONG right in yo face!";
                            send_raw_message(&mut stream, &reply);
                        }
                    }

                    buffer.clear();
                }
            },

            Err(e) => panic!("Failed to connect to host: {}", e),
        };
    }
}

fn send_raw_message<W: Write>(w: &mut W, msg: &String) -> Result<(), Error> {
    let message = format!("{}\r\n", msg);
    try!(w.write(message.as_bytes()));
    w.flush();

    info!("<< {}", msg.trim());

    Ok(())
}
