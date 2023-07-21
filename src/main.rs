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
/// envy simple env tool
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// convert to json
    Json {
        #[arg(value_name = "ENV_FILE")]
        path: PathBuf,
    },
    /// load dotenv file and run command
    Load {
        #[arg(short='f',num_args = 1.., value_delimiter = ' ', value_name="ENV_FILE")]
        input: Vec<PathBuf>,

        #[arg(long = "", allow_hyphen_values = true, num_args = 1.., value_delimiter = ' ', value_name="CMD|ARGS")]
        cmd: Vec<String>,
    },
    /// generate key
    Key {},
    /// encrypt dotenv file
    Encrypt {
        #[arg(short = 'f', value_name = "ENV_FILE")]
        path: PathBuf,

        #[arg(short = 'k', long, value_name = "BASE64")]
        key: String,
    },
    /// decrypt dotenv file
    Decrypt {
        #[arg(short = 'f', value_name = "ENV_FILE")]
        path: PathBuf,

        #[arg(short = 'k', long, value_name = "BASE64")]
        key: String,
    },
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Json { path }) => envy::json::action(path),
        Some(Commands::Load { input, cmd }) => envy::load::action(input, cmd),
        Some(Commands::Key {}) => envy::key::action(),
        Some(Commands::Encrypt { path, key }) => envy::encrypt::action(path, key),
        Some(Commands::Decrypt { path, key }) => envy::decrypt::action(path, key),
        None => Err(anyhow::Error::msg("unexpected subcommand")),
    }
}
