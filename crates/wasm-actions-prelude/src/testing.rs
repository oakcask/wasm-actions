use std::path::{Path, PathBuf};

use wasm_actions_core::{crypto, env, error::Error, fs::File, os};

pub struct ClearEnvGuard {
    envs: Vec<(String, String)>,
}

impl Drop for ClearEnvGuard {
    fn drop(&mut self) {
        for (k, _) in env::vars() {
            env::remove_var(&k);
        }
        for (k, v) in self.envs.iter() {
            env::set_var(k, v);
        }
    }
}

/// Setup pseudo-runner environment for unit testing.
pub async fn clear_env() -> ClearEnvGuard {
    use crate::io::AsyncWriteExt;
    let snapshot = env::vars().collect();
    let tmpdir = os::tmpdir();
    env::set_var("TMPDIR", &tmpdir);
    env::set_var("RUNNER_TEMP", &tmpdir);
    let (state, mut f) = tempfile().await.unwrap();
    let _ = f.shutdown().await;
    let (output, mut f) = tempfile().await.unwrap();
    let _ = f.shutdown().await;
    env::set_var("GITHUB_STATE", state.to_str().unwrap());
    env::set_var("GITHUB_OUTPUT", output.to_str().unwrap());
    ClearEnvGuard { envs: snapshot }
}

/// Create writable temporary file.
/// Maybe insecure. For testing purpose only.
async fn tempfile() -> Result<(PathBuf, File), Error> {
    let tmpdir = os::tmpdir();
    let tmpdir = Path::new(&tmpdir);
    let mut attempt = 6;
    while attempt > 0 {
        attempt -= 1;
        let mut path = tmpdir.to_path_buf();
        path.push(crypto::random_uuid());
        if let Ok(f) = File::create_new(&path.to_str().unwrap()).await {
            return Ok((path, f));
        }
    }

    Err(Error::from(
        "retry attempt exceeded to create temporary file",
    ))
}
