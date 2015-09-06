extern crate irc;
extern crate rand;

use irc::client::prelude::*;
use rand::{thread_rng, sample};

fn main() {
    let commands: [fn(&String) -> Option<String>; 2]= [flip, source];
    let server = IrcServer::new("config.json").unwrap();
    server.identify().unwrap();
    for message in server.iter() {
        let message = message.unwrap(); // We'll just panic if there's an error.
        if &message.command[..] == "PRIVMSG" {
            if let Some(msg) = message.suffix {
                for command in commands.iter() {
                    if let Some(response) = command(&msg) {
                        server.send_privmsg(&message.args[0], &*response).unwrap();
                    }
                }
            }
        }
    }
}

fn flip(message: &String) -> Option<String> {
    let choices = ["heads!", "tails!"];
    let mut rng = thread_rng();
    if message.starts_with("!flip") {
        let choice = sample(&mut rng, choices.iter(), 1);
        Some(format!("The coin goes whirling through the air... {}",
                     choice.get(0).unwrap()))
    } else {
        None
    }
}

fn source(message: &String) -> Option<String> {
    if message.starts_with("!source") {
        Some("Find my source on github \
              ( https://github.com/wraithan/pdxrust-bot ) \
              contribute and teach me new things!".to_owned())
    } else {
        None
    }
}

#[test]
fn test_main() {
    main()
}
