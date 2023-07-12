extern crate anyhow;
extern crate clap;
#[macro_use]
extern crate combine;
extern crate aes_gcm;
extern crate base64;
extern crate serde;
extern crate serde_json;

mod envy;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version , about, long_about=None)]
struct Cli {
    name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// convert to json
    Json {
        #[arg(short = 'f', long)]
        input: PathBuf,
    },
    /// load dotenv file
    Load {
        #[arg(short = 'f', long)]
        input: PathBuf,

        #[arg(long, allow_hyphen_values = true, num_args = 1.., value_delimiter = ' ')]
        cmd: Vec<String>,
    },
    /// generate key
    Key {},
    /// encrypt dotenv file
    Encrypt {
        #[arg(short = 'f', long, value_name = "FILE")]
        input: PathBuf,

        #[arg(short = 'k', long, value_name = "BASE64")]
        key: String,
    },
    /// decrypt dotenv file
    Decrypt {
        #[arg(short = 'f', long, value_name = "FILE")]
        input: PathBuf,

        #[arg(short = 'k', long, value_name = "BASE64")]
        key: String,
    },
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Json { input }) => envy::json::action(input),
        Some(Commands::Load { input, cmd }) => envy::load::action(input, cmd),
        Some(Commands::Key {}) => envy::key::action(),
        Some(Commands::Encrypt { input, key }) => envy::encrypt::action(input, key),
        Some(Commands::Decrypt { input, key }) => envy::decrypt::action(input, key),
        None => Err(anyhow::Error::msg("unexpected subcommand")),
    }
}
