mod v0;
mod v1;

use std::path::Path;

use anyhow::{anyhow, bail};
use git2::{ErrorCode, Repository};

pub use self::v1::{Note, Repo, VERSION};

const VERSION_FILE: &str = "VERSION";

pub fn init(path: &Path) -> anyhow::Result<()> {
    Repository::init_bare(path)?;
    Ok(())
}

fn read_version(repo: &Repository) -> anyhow::Result<u32> {
    let head = match repo.head() {
        Ok(head) => head,
        Err(error) if error.code() == ErrorCode::UnbornBranch => return Ok(0),
        Err(error) => Err(error)?,
    };

    let object = head
        .peel_to_commit()?
        .tree()?
        .get_path(VERSION_FILE.as_ref())?
        .to_object(repo)?;

    let blob = object
        .as_blob()
        .ok_or(anyhow!("Failed to read file {VERSION_FILE}"))?;

    let content = blob.content().to_vec();
    let version = String::from_utf8(content)?.trim().parse::<u32>()?;

    Ok(version)
}

pub fn load_version(path: &Path) -> anyhow::Result<u32> {
    let repo = Repository::open_bare(path)?;
    let version = read_version(&repo)?;
    Ok(version)
}

pub fn load(path: &Path) -> anyhow::Result<Repo> {
    let repository = Repository::open_bare(path)?;
    let version = read_version(&repository)?;

    #[expect(unused_qualifications)]
    let repo = match version {
        v0::VERSION => v0::Repo::load().migrate(),
        v1::VERSION => v1::Repo::load(&repository)?.migrate(),
        n => bail!("invalid repo version {n}"),
    };

    Ok(repo)
}
