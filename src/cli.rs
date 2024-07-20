use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(about = "Copy a file to a remote host", alias = "r")]
    Receive {
        #[clap(help = "Remote source to copy from")]
        source: String,

        #[clap(help = "Local destination to copy to")]
        destination: String,

        #[clap(long, help = "Remote host to connect to")]
        host: String,

        #[clap(
            short,
            long,
            help = "Remote username to connect as",
            default_value = "root"
        )]
        user: String,

        #[clap(short, long, help = "Path to private key")]
        private_key: Option<PathBuf>,

        #[clap(help = "Replace the file if it exists", long, default_value = "false")]
        replace: bool,
    },
}
