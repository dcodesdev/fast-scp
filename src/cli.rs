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
    #[command(about = "Copy a file to a remote host")]
    Receive {
        #[clap(help = "Remote source to copy from")]
        source: String,

        #[clap(help = "Local destination to copy to")]
        destination: String,

        #[clap(long, help = "Remote host to connect to")]
        host: String,

        #[clap(short, long, help = "Remote username to connect as")]
        user: String,

        #[clap(short, long, help = "Path to private key")]
        private_key: Option<PathBuf>,

        #[clap(help = "Replace the file if it exists", long, default_value = "false")]
        replace: bool,
    },
}
