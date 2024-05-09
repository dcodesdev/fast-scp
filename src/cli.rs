use clap::{command, Parser, Subcommand};

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

        #[arg(short, long)]
        host: String,

        #[arg(short, long)]
        username: String,

        #[arg(short, long, default_value = "~/.ssh/id_rsa")]
        private_key_path: String,
    },
}
