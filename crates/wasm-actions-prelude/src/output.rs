use wasm_actions_core::{error::Error, fs::OpenOptions, io::AsyncWriteExt as _};

use crate::env;

pub async fn set_output(name: &str, value: &str) -> Result<(), Error> {
    use std::io::Write;
    let mut buf = Vec::<u8>::new();
    writeln!(&mut buf, "{}={}", name, value).map_err(Error::new)?;
    let path = env::var("GITHUB_OUTPUT").ok_or_else(|| Error::from("$GITHUB_OUTPUT unset"))?;
    let mut f = OpenOptions::new()
        .append(true)
        .open(path)
        .await
        .map_err(Error::new)?;
    f.write_all(&buf).await.map_err(Error::new)?;
    f.shutdown().await.map_err(Error::new)?;
    Ok(())
}

pub async fn save_state(name: &str, value: &str) -> Result<(), Error> {
    use std::io::Write;
    let mut buf = Vec::<u8>::new();
    writeln!(&mut buf, "{}={}", name, value).map_err(Error::new)?;
    let path = env::var("GITHUB_STATE").ok_or_else(|| Error::from("$GITHUB_STATE unset"))?;
    let mut f = OpenOptions::new()
        .append(true)
        .open(path)
        .await
        .map_err(Error::new)?;
    f.write_all(&buf).await.map_err(Error::new)?;
    f.shutdown().await.map_err(Error::new)?;
    Ok(())
}
