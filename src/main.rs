mod command;

use clap::Parser;

fn main() {
    let _args = command::Command::parse();

    match _args.action {
        command::SubCommand::Login(login) => {
            println!("{:?}", login);
        }
        command::SubCommand::Logout(logout) => {
            println!("{:?}", logout);
        }
    }
}
