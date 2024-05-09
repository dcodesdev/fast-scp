use crate::cli::{Cli, Commands};
use crate::scp::copy_file_from_vps;
use clap::Parser;

pub async fn run() -> anyhow::Result<()> {
    let args = Cli::parse();

    let private_key_path = "/path/to/private/key";

    match args.command {
        Commands::Copy { host, username } => copy_file_from_vps(
            &format!("{}:22", host),
            &username,
            private_key_path,
            "/path/to/remote/file",
            "/path/to/local/file",
        ),
    }
}
