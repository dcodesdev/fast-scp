use futures::future::join_all;
use indicatif::ProgressBar;
use ssh2::Session;
use std::{
    fs::{self, File},
    io::{Read, Write},
    net::TcpStream,
    path::PathBuf,
};

pub struct Connect {
    session: Session,
    ssh_opts: SshOpts,
}

impl Connect {
    pub fn new(ssh_opts: SshOpts) -> anyhow::Result<Self> {
        let session = create_session(&ssh_opts)?;

        Ok(Self { session, ssh_opts })
    }

    pub async fn receive(&self, from: &PathBuf, to: &PathBuf) -> anyhow::Result<()> {
        let start = std::time::Instant::now();
        let files = self.list(from)?;

        let pb = ProgressBar::new(files.len() as u64);

        let mut handles = Vec::new();
        for item in files {
            let to_path = to.join(item.strip_prefix(from).unwrap());
            let item_clone = item.clone();
            let ssh_opts = self.ssh_opts.clone();
            let pb = pb.clone();
            let handle = tokio::task::spawn(async move {
                let result = copy_file_from_remote(&ssh_opts, item_clone.clone(), to_path).await;
                pb.inc(1);
                result
            });

            handles.push(handle);
        }

        let items = join_all(handles).await;

        if items.iter().all(|x| x.is_ok()) {
            println!("Done in {:.2?}", start.elapsed());
            Ok(())
        } else {
            Err(anyhow::anyhow!("One or more files failed to copy"))
        }
    }

    fn list(&self, dir: &PathBuf) -> anyhow::Result<Vec<PathBuf>> {
        let mut channel = self.session.channel_session()?;

        channel.exec(&format!("ls -R {}", dir.display()))?;

        let mut buf = String::new();
        channel.read_to_string(&mut buf)?;

        let mut dirs: Vec<PathBuf> = Vec::new();
        let structured = buf
            .split("\n\n")
            .map(|x| {
                let mut lines = x.lines();
                let dir: PathBuf = lines.next().unwrap().split(":").next().unwrap().into();

                let files = lines.collect::<Vec<_>>();

                let full_path = files
                    .iter()
                    .map(|x| PathBuf::new().join(x))
                    .map(|x| dir.join(x))
                    .collect::<Vec<_>>();

                dirs.push(dir);
                full_path
            })
            .collect::<Vec<_>>();

        let flattened = structured.iter().flatten().collect::<Vec<_>>();

        let files_only = flattened
            .iter()
            .filter(|x| !dirs.contains(x))
            .map(|x| x.to_path_buf())
            .collect::<Vec<_>>();

        Ok(files_only)
    }
}

#[derive(Clone)]
pub struct SshOpts {
    pub host: String,
    pub username: String,
    pub private_key: PathBuf,
}

async fn copy_file_from_remote(
    ssh_opts: &SshOpts,
    remote_file_path: PathBuf,
    local_file_path: PathBuf,
) -> anyhow::Result<()> {
    let create_session = || create_session(ssh_opts);
    let session = with_retry(create_session, 10)?;

    // Create a SCP channel for receiving the file
    let (mut remote_file, stat) = session.scp_recv(&remote_file_path)?;
    let mut contents = Vec::with_capacity(stat.size() as usize);
    remote_file.read_to_end(&mut contents)?;

    // make the dir if not exists
    fs::create_dir_all(local_file_path.parent().unwrap())?;

    // Create local file and write to it
    let mut local_file = File::create(local_file_path)?;
    local_file.write_all(&contents)?;
    session.disconnect(None, "Bye", None)?;

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

fn with_retry<T, F>(f: F, max_retries: u32) -> anyhow::Result<T>
where
    F: Fn() -> anyhow::Result<T>,
{
    let mut retries = 0;
    loop {
        match f() {
            Ok(x) => return Ok(x),
            Err(e) => {
                if retries >= max_retries {
                    return Err(e);
                }

                retries += 1;
            }
        }
    }
}
