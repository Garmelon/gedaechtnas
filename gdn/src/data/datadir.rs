use std::{
    ffi::{OsStr, OsString},
    fs,
    io::ErrorKind,
    ops::Deref,
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail, Context};
use rand::{distr::Alphanumeric, Rng};
use serde::{de::DeserializeOwned, Serialize};

use super::lockfile::LockFile;

/// Create a temporary file name derived from an existing file name.
///
/// The temporary name has the form `.<name>.<rand_suffix>~gdn`.
fn tmp_file_name(name: &OsStr) -> OsString {
    let random_suffix = rand::rng()
        .sample_iter(Alphanumeric)
        .take(6)
        .map(char::from)
        .collect::<String>();

    let mut tmp_name = OsString::new();
    if !tmp_name.as_bytes().starts_with(b".") {
        tmp_name.push(".");
    }
    tmp_name.push(name);
    tmp_name.push(".");
    tmp_name.push(random_suffix);
    tmp_name.push("~");
    tmp_name.push(crate::ABBREVIATED_NAME);
    tmp_name
}

fn atomic_write(path: &Path, contents: impl AsRef<[u8]>) -> anyhow::Result<()> {
    let name = path
        .file_name()
        .ok_or_else(|| anyhow!("path has no file name: {}", path.display()))?;

    let parent = path
        .parent()
        .ok_or_else(|| anyhow!("path has no parent: {}", path.display()))?;

    let tmp_path = path.with_file_name(tmp_file_name(name));

    fs::create_dir_all(parent)?;
    fs::write(&tmp_path, contents)?;
    fs::rename(&tmp_path, path)?;

    Ok(())
}

pub struct UnlockedDataDir {
    path: PathBuf,
}

impl UnlockedDataDir {
    pub(super) fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub(super) fn path(&self) -> &Path {
        &self.path
    }

    fn version_file(&self) -> PathBuf {
        self.path.join("VERSION")
    }

    fn lock_file(&self) -> PathBuf {
        self.path.join("LOCK")
    }

    pub fn lock(self) -> anyhow::Result<LockedDataDir> {
        Ok(LockedDataDir {
            lockfile: LockFile::lock(self.lock_file())?,
            unlocked: self,
        })
    }

    pub(super) fn read_string(&self, path: &Path) -> anyhow::Result<String> {
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))
    }

    pub(super) fn read_string_optional(&self, path: &Path) -> anyhow::Result<Option<String>> {
        match fs::read_to_string(path) {
            Ok(string) => Ok(Some(string)),
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
        .with_context(|| format!("failed to read {}", path.display()))
    }

    pub(super) fn read_json<T: DeserializeOwned>(&self, path: &Path) -> anyhow::Result<T> {
        let string = self.read_string(path)?;
        let data = serde_json::from_str(&string)
            .with_context(|| format!("failed to parse {} as json", path.display()))?;
        Ok(data)
    }

    pub(super) fn read_version(&self) -> anyhow::Result<u32> {
        let path = self.version_file();
        match self.read_string_optional(&path)? {
            None => Ok(0),
            Some(string) => Ok(string.trim().parse().with_context(|| {
                format!("failed to parse {} as version number", path.display())
            })?),
        }
    }

    pub(super) fn require_version(&self, expected: u32) -> anyhow::Result<()> {
        let actual = self.read_version()?;
        if actual != expected {
            bail!(
                "expected version {expected}, but found {actual} at {}",
                self.version_file().display()
            );
        }
        Ok(())
    }
}

pub struct LockedDataDir {
    unlocked: UnlockedDataDir,
    lockfile: LockFile,
}

impl LockedDataDir {
    pub fn unlock(self) -> anyhow::Result<UnlockedDataDir> {
        self.lockfile.unlock()?;
        Ok(self.unlocked)
    }

    pub(super) fn write_string(&self, path: &Path, string: &str) -> anyhow::Result<()> {
        atomic_write(path, string).with_context(|| format!("failed to write {}", path.display()))
    }

    pub(super) fn write_json<T: Serialize>(&self, path: &Path, contents: &T) -> anyhow::Result<()> {
        let string = serde_json::to_string_pretty(contents)
            .with_context(|| format!("failed to format json for {}", path.display()))?;
        self.write_string(path, &string)?;
        Ok(())
    }

    pub(super) fn write_version(&self, version: u32) -> anyhow::Result<()> {
        self.write_string(&self.version_file(), &format!("{version}\n"))
    }
}

impl Deref for LockedDataDir {
    type Target = UnlockedDataDir;

    fn deref(&self) -> &Self::Target {
        &self.unlocked
    }
}
