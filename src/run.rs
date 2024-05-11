use crate::cli::{Cli, Commands};
use crate::error::ScpError;
use crate::scp::{Connect, SshOpts};
use clap::Parser;
use dirs_next::home_dir;
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
            let private_key = match private_key {
                Some(path) => PathBuf::from(path),
                None => home_dir()
                    .ok_or(
                        ScpError::Io(
                            std::io::Error::new(
                                std::io::ErrorKind::Other,
                                "Could not find home directory, please provide the private key path using the --private-key-path <key> flag",
                            ),
                        ),
                    )?
                    .join(".ssh/id_rsa"),
            };

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
