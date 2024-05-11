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
        #[arg(help = "Remote source to copy from")]
        source: String,

        #[arg(help = "Local destination to copy to")]
        destination: String,

        #[arg(long, help = "Remote host to connect to")]
        host: String,

        #[arg(short, long, help = "Remote username to connect as")]
        user: String,

        #[arg(short, long, help = "Path to private key")]
        private_key: Option<PathBuf>,
    },
}
