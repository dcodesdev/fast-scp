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
