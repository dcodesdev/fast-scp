#[derive(Debug)]
pub enum ScpError {
    Io(std::io::Error),
    Ssh(ssh2::Error),
}

impl From<std::io::Error> for ScpError {
    fn from(e: std::io::Error) -> Self {
        ScpError::Io(e)
    }
}

impl From<ssh2::Error> for ScpError {
    fn from(e: ssh2::Error) -> Self {
        ScpError::Ssh(e)
    }
}
