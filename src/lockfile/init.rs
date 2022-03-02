use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub struct Lockfile {
    pub path: PathBuf,
}

impl Lockfile {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn try_load_or_create(&self) -> anyhow::Result<()> {
        if !self.path.exists() {
            //log::debug!("lockfile at {} does not exist or is not readable", &self.path);
            let mut file = File::create(&self.path)?;
            file.write_all(b"")?;
            //log::debug!("lockfile at {} written to disk", &self.path);
        }

        Ok(())
    }
}
