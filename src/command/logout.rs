use serde::Deserialize;

use clap::Args;

#[derive(Clone, Debug, Default, Deserialize, Args)]
pub struct ConfigOptions {}

#[derive(Clone, Debug, Args)]
#[clap(name = "logout", about = "Try logout")]
pub struct Command {
    #[clap(flatten)]
    pub value: ConfigOptions,
}
