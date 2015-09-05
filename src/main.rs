extern crate irc;
extern crate rand;

use std::default::Default;
use irc::client::prelude::*;
use rand::{thread_rng, sample};

fn main() {
    let mut rng = thread_rng();
    let choices = ["heads!", "tails!"];
    let config = Config {
        nickname: Some(format!("pdxferris")),
        alt_nicks: Some(vec![format!("pdxrust-bot"), format!("pdx-rust-bot")]),
        server: Some(format!("irc.freenode.net")),
        channels: Some(vec![format!("#pdxbots"),
                            format!("#pdxrust")]),
        .. Default::default()
    };
    let server = IrcServer::from_config(config).unwrap();
    server.identify().unwrap();
    for message in server.iter() {
        let message = message.unwrap(); // We'll just panic if there's an error.
        if &message.command[..] == "PRIVMSG" {
            if let Some(msg) = message.suffix {
                if msg.starts_with("!flip") {
                    let choice = sample(&mut rng, choices.iter(), 1);
                    server.send_privmsg(&message.args[0],
                                        &*format!("The coin goes whirling through the air... {}",
                                                  choice.get(0).unwrap())).unwrap();
                }
            }
        }
    }
}
