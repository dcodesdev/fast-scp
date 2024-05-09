use anyhow::Ok;
use ssh2::Session;
use std::{
    fs::{self, File},
    io::{Read, Write},
    net::TcpStream,
    path::{Path, PathBuf},
};

pub struct Receiver {
    /// Destination directory
    dest: PathBuf,
    session: Session,
}

impl Receiver {
    pub fn new(ssh_opts: SshOpts) -> anyhow::Result<Self> {
        Ok(Self {
            dest: PathBuf::from("."),
            session: create_session(&ssh_opts)?,
        })
    }

    pub fn dir(mut self, dir: PathBuf) -> Self {
        self.dest = dir;
        self
    }

    pub fn receive(&self, path: &PathBuf) -> anyhow::Result<()> {
        if self.is_dir(path)? {
            self.handle_dir(path)
        } else {
            copy_file_from_vps(
                &self.session,
                path,
                &self.dest.join(path.file_name().unwrap()),
            )
        }
    }

    fn handle_dir(&self, dir: &PathBuf) -> anyhow::Result<()> {
        let dirs = list_dir(&self.session, &dir);

        for item in dirs.iter() {
            println!("File: {:?}", item);
        }

        unimplemented!("Handle dir")
    }

    fn is_dir(&self, path: &PathBuf) -> anyhow::Result<bool> {
        let sftp = self.session.sftp().unwrap();
        let metadata = sftp.stat(path)?;

        Ok(metadata.is_dir())
    }
}

pub struct SshOpts {
    pub host: String,
    pub username: String,
    pub private_key: PathBuf,
}

fn copy_file_from_vps(
    session: &Session,
    remote_file_path: &PathBuf,
    local_file_path: &PathBuf,
) -> anyhow::Result<()> {
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

fn list_dir(session: &Session, dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let sftp = session.sftp()?;
    let dir = sftp.readdir(dir)?;
    let dirs = dir.into_iter().map(|entry| entry.0).collect();
    Ok(dirs)
}

pub fn create_session(ssh_opts: &SshOpts) -> anyhow::Result<Session> {
    // Connect to the host
    let tcp = TcpStream::connect(&ssh_opts.host)?;
    let mut session = Session::new()?;
    session.set_tcp_stream(tcp);
    session.handshake()?;

    // Authenticate using a private key
    session.userauth_pubkey_file(&ssh_opts.username, None, &ssh_opts.private_key, None)?;

    Ok(session)
}
