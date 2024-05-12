use core::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ScpError {
    Io(std::io::Error),
    Ssh(ssh2::Error),
    Other(String),
}

impl Display for ScpError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ScpError::Io(e) => write!(f, "IO error: {}", e),
            ScpError::Ssh(e) => write!(f, "SSH error: {}", e.message()),
            ScpError::Other(e) => write!(f, "Error: {}", e),
        }
    }
}

impl From<&str> for ScpError {
    fn from(e: &str) -> Self {
        ScpError::Other(e.to_string())
    }
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

pub type Result<T = ()> = anyhow::Result<T, ScpError>;
