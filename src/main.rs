mod action;
mod command;

use clap::Parser;

fn main() {
    let _args = command::Command::parse();

    match _args.action {
        command::SubCommand::Login(login) => {
            action::login::run(login.value);
        }
        command::SubCommand::Logout(logout) => {
            action::logout::run(logout.value);
        }
    }
}
