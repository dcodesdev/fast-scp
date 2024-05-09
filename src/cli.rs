use clap::{command, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Receive {
        source: String,
        destination: String,

        #[arg(long)]
        host: String,

        #[arg(short, long)]
        username: String,

        #[arg(short, long)]
        private_key_path: Option<PathBuf>,
    },
}
