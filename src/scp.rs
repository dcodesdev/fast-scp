use futures::future::join_all;
use indicatif::{ProgressBar, ProgressStyle};
use ssh2::{Session, Sftp};
use std::{
    ffi::OsStr,
    fs::{self, File},
    io::{Read, Write},
    net::TcpStream,
    path::PathBuf,
    time::Duration,
};

use crate::{error::Result, utils::with_retry};

pub struct Connect {
    session: Session,
    ssh_opts: SshOpts,
    mode: Mode,
    sftp: Sftp,
}

impl Connect {
    pub fn new(ssh_opts: SshOpts, mode: Mode) -> Result<Self> {
        let session = create_session(&ssh_opts)?;
        let sftp = session.sftp()?;

        Ok(Self {
            session,
            ssh_opts,
            mode,
            sftp,
        })
    }

    pub async fn receive(&self, from: &PathBuf, to: &PathBuf) -> Result<()> {
        let is_dir = self.stat(from)?;

        if is_dir {
            self.handle_dir(from, to).await
        } else {
            self.handle_file(from, to).await
        }
    }

    async fn handle_file(&self, from: &PathBuf, to: &PathBuf) -> Result<()> {
        let full_path = to.join(from.file_name().unwrap_or(OsStr::new("unknown")));
        let result =
            copy_file_from_remote(&self.ssh_opts, from.clone(), full_path, &self.mode).await;

        println!("✅ File received successfully");
        result
    }

    async fn handle_dir(&self, from: &PathBuf, to: &PathBuf) -> Result<()> {
        let mut files = self.list_files(from)?;

        #[cfg(any(target_os = "linux", target_os = "macos"))]
        if self.mode != Mode::Replace {
            let output = std::process::Command::new("find")
                .arg(to)
                .arg("-type")
                .arg("f")
                .output()?;

            let existing_files = String::from_utf8_lossy(&output.stdout)
                .lines()
                .map(|line| PathBuf::from(line))
                .collect::<Vec<_>>();

            files = files
                .into_iter()
                .filter(|file| !existing_files.contains(&to.join(file.strip_prefix(from).unwrap())))
                .collect::<Vec<_>>();
        }

        let pb = ProgressBar::new(files.len() as u64);
        pb.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})\n\n{msg}",
            )
            .unwrap()
            .progress_chars("#>-"),
        );
        pb.enable_steady_tick(Duration::from_millis(100));

        let mut handles = Vec::new();
        for item in &files {
            let to_path = to.join(item.strip_prefix(from).unwrap());
            let item_clone = item.clone();
            let ssh_opts = self.ssh_opts.clone();
            let pb = pb.clone();
            let mode = self.mode.clone();
            let handle = tokio::spawn(async move {
                let result =
                    copy_file_from_remote(&ssh_opts, item_clone.clone(), to_path, &mode).await;
                pb.inc(1);
                result
            });

            handles.push(handle);
        }

        let items = join_all(handles).await;

        if items.iter().all(|x| x.is_ok()) {
            pb.finish_with_message(format!(
                "✅ All files received successfully ({} files)",
                files.len()
            ));
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "One or more files failed to copy",
            )
            .into())
        }
    }

    fn stat(&self, path: &PathBuf) -> Result<bool> {
        let file = self.sftp.stat(&path)?;
        Ok(file.is_dir())
    }

    fn list_files(&self, dir: &PathBuf) -> Result<Vec<PathBuf>> {
        let mut channel = self.session.channel_session()?;

        channel.exec(&format!("find {} -type f", dir.display()))?;

        let mut buf = String::new();
        channel.read_to_string(&mut buf)?;

        let files_only = find_files(&buf);

        Ok(files_only)
    }
}

pub fn find_files(buf: &str) -> Vec<PathBuf> {
    buf.lines().map(|line| PathBuf::from(line.trim())).collect()
}

#[derive(Clone)]
pub struct SshOpts {
    pub host: String,
    pub username: String,
    pub private_key: PathBuf,
}

/// Mode to use when copying files
/// Replace will overwrite the file if it exists
/// Ignore will skip the file if it exists
#[derive(Clone, PartialEq)]
pub enum Mode {
    Replace,
    Ignore,
}

async fn copy_file_from_remote(
    ssh_opts: &SshOpts,
    remote_file_path: PathBuf,
    local_file_path: PathBuf,
    mode: &Mode,
) -> Result<()> {
    let create_session = || create_session(ssh_opts);
    let session = with_retry(create_session, 10)?;

    // Create a SCP channel for receiving the file
    let (mut remote_file, stat) = session.scp_recv(&remote_file_path)?;
    let mut contents = Vec::with_capacity(stat.size() as usize);
    remote_file.read_to_end(&mut contents)?;

    // make the dir if not exists
    fs::create_dir_all(local_file_path.parent().unwrap())?;

    match mode {
        Mode::Replace => {
            let mut local_file = File::create(&local_file_path)?;
            local_file.write_all(&contents)?;
        }
        Mode::Ignore => {
            if local_file_path.exists() {
                println!(
                    "Skipping already existing file: {}",
                    local_file_path.display()
                );
                return Ok(());
            }

            let mut local_file = File::create(local_file_path)?;
            local_file.write_all(&contents)?;
        }
    }

    session.disconnect(None, "Bye", None)?;

    Ok(())
}

pub fn create_session(ssh_opts: &SshOpts) -> Result<Session> {
    // Connect to the host
    let tcp = TcpStream::connect(&ssh_opts.host)?;
    let mut session = Session::new()?;
    session.set_tcp_stream(tcp);
    session.handshake()?;

    // Authenticate using a private key
    session.userauth_pubkey_file(&ssh_opts.username, None, &ssh_opts.private_key, None)?;

    Ok(session)
}
