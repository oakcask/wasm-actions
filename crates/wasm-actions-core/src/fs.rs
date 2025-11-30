mod file;

pub use file::File;
pub use file::OpenOptions;
use crate::io::AsyncReadExt;

pub async fn read_to_string<P: AsRef<str>>(path: P) -> std::io::Result<String> {
    let path = path.as_ref().to_owned();
    let mut f = File::open(&path).await?;
    let mut s = String::new();
    f.read_to_string(&mut s).await?;
    Ok(s)
}
