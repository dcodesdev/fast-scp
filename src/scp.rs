use anyhow::Ok;
use ssh2::{FileStat, Session, Sftp};
use std::{
    fs::{self, File},
    io::{Read, Write},
    net::TcpStream,
    path::PathBuf,
};

pub struct Connect {
    sftp: Sftp,
}

impl Connect {
    pub fn new(ssh_opts: SshOpts) -> anyhow::Result<Self> {
        let session = create_session(&ssh_opts)?;
        let sftp = session.sftp()?;

        Ok(Self { sftp })
    }

    pub fn receive(&self, from: &PathBuf, to: &PathBuf) -> anyhow::Result<()> {
        let items = self.recursive_list(&from)?;

        println!("Items: {:?}", items);
        Ok(())

        // if self.is_dir(from)? {
        //     let dirs = self.get_dir_paths_remote(from);
        //     for dir in dirs? {
        //         self.receive(&dir, &to.join(dir.file_name().unwrap()))?;
        //     }
        //     Ok(())
        // } else {
        //     copy_file_from_vps(&self.session, from, &to.join(from.file_name().unwrap()))
        // }
    }

    fn list(&self, dir: &PathBuf) -> anyhow::Result<Vec<(PathBuf, FileStat)>> {
        let dirs = self.sftp.readdir(dir)?;
        Ok(dirs)
    }

    fn recursive_list(&self, path: &PathBuf) -> anyhow::Result<Vec<PathBuf>> {
        let dir = self.list(&path)?;

        let mut results: Vec<PathBuf> = Vec::new();
        for (entry, stat) in dir {
            if stat.is_dir() {
                let items = self.recursive_list(&entry)?;
                results.extend(items);
            } else {
                results.push(entry);
            }
        }

        Ok(results)
    }
}

pub struct SshOpts {
    pub host: String,
    pub username: String,
    pub private_key: PathBuf,
}

fn copy_file_from_remote(
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
