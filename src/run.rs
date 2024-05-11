use crate::cli::{Cli, Commands};
use crate::error::ScpError;
use crate::scp::{Connect, SshOpts};
use crate::utils::get_private_key_path;
use clap::Parser;
use std::path::PathBuf;

pub async fn run() -> anyhow::Result<(), ScpError> {
    let args = Cli::parse();

    match args.command {
        Commands::Receive {
            source,
            destination,
            host,
            user: username,
            private_key,
        } => {
            let private_key = get_private_key_path(&private_key)?;

            let scp_opts = SshOpts {
                host: format!("{}:22", host),
                private_key,
                username,
            };

            return Connect::new(scp_opts)?
                .receive(&PathBuf::from(source), &PathBuf::from(destination))
                .await;
        }
    }
}
