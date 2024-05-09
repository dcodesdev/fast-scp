use anyhow::Ok;
use ssh2::Session;
use std::{
    fs,
    fs::File,
    io::{Read, Write},
    net::TcpStream,
    path::PathBuf,
};

pub fn create_session(host: &str) -> anyhow::Result<Session> {
    // Connect to the host
    let tcp = TcpStream::connect(host)?;
    let mut session = Session::new()?;
    session.set_tcp_stream(tcp);
    session.handshake()?;

    Ok(session)
}

pub struct ReceiveFile {
    host: String,
    username: String,
    remote_file_path: PathBuf,
    dest_dir: PathBuf,
    private_key_path: PathBuf,
}

impl ReceiveFile {
    pub fn new(host: String, remote_file_path: PathBuf) -> Self {
        Self {
            host: host.to_string(),
            dest_dir: PathBuf::from("."),
            remote_file_path,
            username: "root".to_string(),
            private_key_path: PathBuf::from("~/.ssh/id_rsa"),
        }
    }

    pub fn dir(mut self, to_dir: PathBuf) -> Self {
        self.dest_dir = to_dir;
        self
    }

    pub fn private_key(mut self, private_key_path: PathBuf) -> Self {
        self.private_key_path = private_key_path;
        self
    }

    pub fn username(mut self, username: String) -> Self {
        self.username = username;
        self
    }

    pub fn receive(self) -> anyhow::Result<()> {
        copy_file_from_vps(
            &format!("{}:22", self.host),
            &self.username,
            &self.remote_file_path,
            &self
                .dest_dir
                .join(self.remote_file_path.file_name().unwrap()),
            &self.private_key_path,
        )
    }
}

pub fn copy_file_from_vps(
    host: &str,
    username: &str,
    remote_file_path: &PathBuf,
    local_file_path: &PathBuf,
    private_key_path: &PathBuf,
) -> anyhow::Result<()> {
    let session = create_session(host)?;

    // Authenticate using a private key
    session.userauth_pubkey_file(username, None, &private_key_path, None)?;

    // Create a SCP channel for receiving the file
    let (mut remote_file, stat) = session.scp_recv(&remote_file_path)?;
    let mut contents = Vec::with_capacity(stat.size() as usize);
    remote_file.read_to_end(&mut contents)?;

    // make the dir if not exists
    fs::create_dir_all(local_file_path.parent().unwrap())?;

    // Create local file and write to it
    let mut local_file = File::create(local_file_path)?;
    local_file.write_all(&contents)?;

    Ok(())
}
