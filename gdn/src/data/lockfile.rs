use std::{
    fs::{self, File},
    io,
    path::PathBuf,
};

pub struct LockFile {
    path: PathBuf,
    file: Option<File>,
}

impl LockFile {
    pub fn lock(path: PathBuf) -> io::Result<Self> {
        Ok(Self {
            file: Some(File::create_new(&path)?),
            path,
        })
    }

    fn unlock_ref(&mut self) -> io::Result<()> {
        let file = self
            .file
            .take()
            .expect("can't unlock lockfile more than once");

        drop(file);
        fs::remove_file(&self.path)?;
        Ok(())
    }

    pub fn unlock(mut self) -> io::Result<()> {
        self.unlock_ref()
    }
}

impl Drop for LockFile {
    fn drop(&mut self) {
        if let Err(_err) = self.unlock_ref() {
            // TODO Log error
        }
    }
}
