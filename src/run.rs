use crate::cli::{Cli, Commands};
use crate::scp::ReceiveFile;
use clap::Parser;
use dirs_next::home_dir;
use std::path::PathBuf;

pub async fn run() -> anyhow::Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Receive {
            source,
            destination,
            host,
            username,
            private_key_path,
        } => {
            let private_key = match private_key_path {
                Some(path) => PathBuf::from(path),
                None => home_dir()
                    .ok_or(anyhow::anyhow!(
                        "Could not find home directory, please provide 
                        the private key path using the --private-key-path <key> flag"
                    ))?
                    .join(".ssh/id_rsa"),
            };

            return ReceiveFile::new(host, PathBuf::from(source))
                .dir(PathBuf::from(destination))
                .username(username)
                .private_key(private_key)
                .receive();
        }
    }
}
