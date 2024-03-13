use serde::Deserialize;

use clap::Args;

#[derive(Clone, Debug, Default, Deserialize, Args)]
pub struct ConfigOptions {
    #[clap(short, long, help = "id")]
    pub id: String,

    #[clap(short, long, help = "password")]
    pub password: String,
}

#[derive(Clone, Debug, Args)]
#[clap(name = "login", about = "Try login")]
pub struct Command {
    #[clap(flatten)]
    pub value: ConfigOptions,
}
