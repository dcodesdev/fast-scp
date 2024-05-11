use dirs_next::home_dir;
use std::path::PathBuf;

use crate::error::ScpError;

pub fn with_retry<T, F>(f: F, max_retries: u32) -> anyhow::Result<T, ScpError>
where
    F: Fn() -> anyhow::Result<T, ScpError>,
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

pub fn get_private_key_path(private_key: &Option<PathBuf>) -> anyhow::Result<PathBuf, ScpError> {
    match private_key {
                Some(path) => Ok(PathBuf::from(path)),
                None => Ok(home_dir()
                    .ok_or(
                        ScpError::Io(
                            std::io::Error::new(
                                std::io::ErrorKind::Other,
                                "Could not find home directory, please provide the private key path using the --private-key-path <key> flag",
                            ),
                        ),
                    )?
                    .join(".ssh/id_rsa")),
            }
}
