use crate::env;

pub struct ClearEnvGuard {
    envs: Vec<(String, String)>
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

pub fn clear_env() -> ClearEnvGuard {
    ClearEnvGuard {
        envs: env::vars().collect()
    }
}
