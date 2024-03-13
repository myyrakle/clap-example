use std::{fs::OpenOptions, io::Write};

use crate::command::login::ConfigOptions;

pub fn run(option: ConfigOptions) {
    println!("login: {:?}", option);

    let mut credentials = String::new();
    credentials.push_str(format!("id: {}\n", option.id).as_str());
    credentials.push_str(format!("pwd: {}\n", option.password).as_str());

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("./credentials.txt")
        .unwrap();

    file.write(credentials.as_bytes()).unwrap();
}
