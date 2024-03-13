use std::path::Path;

use crate::command::logout::ConfigOptions;

pub fn run(_: ConfigOptions) {
    let credentials_path = Path::new("./credentials.txt");

    if !credentials_path.exists() {
        println!("No credentials found");
        return;
    }

    std::fs::remove_file(credentials_path).unwrap();
}
