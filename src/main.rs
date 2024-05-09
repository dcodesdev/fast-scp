use dotenvy::dotenv;
use ssh2::Session;
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;

fn copy_file_from_vps(
    host: &str,
    username: &str,
    private_key_path: &str,
    remote_file_path: &str,
    local_file_path: &str,
) -> anyhow::Result<()> {
    // Connect to the host
    let tcp = TcpStream::connect(host)?;
    let mut session = Session::new()?;
    session.set_tcp_stream(tcp);
    session.handshake()?;

    // Authenticate using a private key
    session.userauth_pubkey_file(username, None, Path::new(private_key_path), None)?;

    // Create a SCP channel for receiving the file
    let (mut remote_file, stat) = session.scp_recv(Path::new(remote_file_path))?;
    let mut contents = Vec::with_capacity(stat.size() as usize);
    remote_file.read_to_end(&mut contents)?;

    // Create local file and write to it
    let mut local_file = File::create(Path::new(local_file_path))?;
    local_file.write_all(&contents)?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let host = std::env::var("VPS_HOST")?;
    let username = std::env::var("VPS_USERNAME")?;
    let private_key_path = std::env::var("VPS_PRIVATE_KEY_PATH")?;

    copy_file_from_vps(
        &format!("{}:22", host),
        &username,
        &private_key_path,
        "/path/to/remote/file",
        "/path/to/local/file",
    )
}
