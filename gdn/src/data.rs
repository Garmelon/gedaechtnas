use std::path::PathBuf;

use anyhow::Context;
use directories::ProjectDirs;

mod datadir;
mod lockfile;
mod v0;
mod v1;

pub use crate::repo::VERSION as REPO_VERSION;

pub use self::{
    datadir::{LockedDataDir, UnlockedDataDir},
    v1::{State, VERSION, add_repo, load_repo, load_repo_version, load_state, remove_repo, tidy},
};

fn migrate(dir: &LockedDataDir) -> anyhow::Result<()> {
    loop {
        let version = dir.read_version().context("failed to migrate data dir")?;
        match version {
            v0::VERSION => v0::migrate(dir),
            _ => break Ok(()),
        }
        .with_context(|| format!("failed to migrate data dir from version {version}"))?;
    }
}

pub fn open(path: PathBuf) -> anyhow::Result<UnlockedDataDir> {
    let dir = UnlockedDataDir::new(path);
    dir.require_version(VERSION)?;
    Ok(dir)
}

pub fn open_and_migrate(path: PathBuf) -> anyhow::Result<LockedDataDir> {
    let dir = UnlockedDataDir::new(path).lock()?;
    migrate(&dir)?;
    dir.require_version(VERSION)?;
    Ok(dir)
}

pub fn path() -> anyhow::Result<PathBuf> {
    let dirs = ProjectDirs::from("de", "plugh", crate::TECHNICAL_NAME)
        .context("failed to locate data dir")?;
    Ok(dirs.data_dir().to_path_buf())
}
