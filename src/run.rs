use crate::cli::{Cli, Commands};
use crate::scp::copy_file_from_vps;
use clap::Parser;

pub async fn run() -> anyhow::Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Receive {
            source,
            destination,
            host,
            username,
            private_key_path,
        } => copy_file_from_vps(
            &format!("{}:22", host),
            &username,
            &source,
            &destination,
            &private_key_path,
        ),
    }
}
