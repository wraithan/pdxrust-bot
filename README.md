# pdxrust-bot

Welcome! I'm starting a bot for our channel (#pdxrust on irc.freenode.net) and
would love you to contribute!

So far it is just using the [`irc`](https://crates.io/crates/irc) to all the
heavy lifting.

## Setup

So far everything is very simple. Clone this repo, then `cargo run`. You may
want to change the name in the source first.

## Contributing

Welcoming PRs for features, code refactoring, etc. The bot currently using `!`
as the prefix, but doesn't do any special parsing around that. All features will
be entertained, but if it is more out there, then perhaps open an issue to
discuss or bring it up in the channel

## Code of Conduct

As this bot lives in a channel governed by the
[Rust Code of Conduct](https://www.rust-lang.org/conduct.html) so is this repo,
the issue tracker, and pull requests. No features violating the CoC will be
tolerated in the repo. If you feel that someone has violated this, please notify
Wraithan.
