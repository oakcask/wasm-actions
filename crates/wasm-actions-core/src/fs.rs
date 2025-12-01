mod file;

use crate::io::AsyncReadExt;
pub use file::File;
pub use file::OpenOptions;

pub async fn read_to_string<P: AsRef<str>>(path: P) -> std::io::Result<String> {
    let mut f = File::open(path.as_ref()).await?;
    let mut s = String::new();
    f.read_to_string(&mut s).await?;
    Ok(s)
}
