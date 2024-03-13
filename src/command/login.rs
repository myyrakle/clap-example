use serde::Deserialize;

use clap::Args;

#[derive(Clone, Debug, Default, Deserialize, Args)]
pub struct ConfigOptions {
    #[clap(short, long, default_value = "", help = "id")]
    pub id: String,

    #[clap(short, long, default_value = "", help = "password")]
    pub password: String,

    #[clap(long, default_value = "false", help = "interactive mode")]
    pub interactive: bool,
}

#[derive(Clone, Debug, Args)]
#[clap(name = "login", about = "Try login")]
pub struct Command {
    #[clap(flatten)]
    pub value: ConfigOptions,
}
