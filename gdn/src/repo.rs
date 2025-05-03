mod v0;
mod v1;

use std::path::Path;

use anyhow::{anyhow, bail};
use git2::{Commit, ErrorCode, Oid, Reference, Repository};
use jiff::Zoned;

pub use self::v1::{Repo, VERSION};

const VERSION_FILE: &str = "VERSION";

pub fn init(path: &Path) -> anyhow::Result<()> {
    Repository::init_bare(path)?;
    Ok(())
}

fn read_head(repository: &Repository) -> anyhow::Result<Option<Reference<'_>>> {
    match repository.head() {
        Ok(head) => Ok(Some(head)),
        Err(error) if error.code() == ErrorCode::UnbornBranch => Ok(None),
        Err(error) => Err(error)?,
    }
}

fn read_version(repository: &Repository, commit: &Commit<'_>) -> anyhow::Result<u32> {
    let object = commit
        .tree()?
        .get_path(VERSION_FILE.as_ref())?
        .to_object(repository)?;

    let blob = object
        .as_blob()
        .ok_or(anyhow!("Failed to read file {VERSION_FILE}"))?;

    let content = blob.content().to_vec();
    let version = String::from_utf8(content)?.trim().parse::<u32>()?;

    Ok(version)
}

pub fn load_version(path: &Path) -> anyhow::Result<u32> {
    let repository = Repository::open_bare(path)?;
    let Some(head) = read_head(&repository)? else {
        return Ok(v0::VERSION);
    };
    let commit = head.peel_to_commit()?;
    let version = read_version(&repository, &commit)?;
    Ok(version)
}

pub fn load(path: &Path) -> anyhow::Result<Repo> {
    let repository = Repository::open_bare(path)?;
    let Some(head) = read_head(&repository)? else {
        return Ok(v0::Repo::load().migrate());
    };
    let commit = head.peel_to_commit()?;
    let version = read_version(&repository, &commit)?;
    let tree = commit.tree()?;

    #[expect(unused_qualifications)]
    let repo = match version {
        v1::VERSION => v1::Repo::load_from_tree(&repository, &tree)?.migrate(),
        n => bail!("invalid repo version {n}"),
    };

    Ok(repo)
}

pub fn save(path: &Path, repo: Repo) -> anyhow::Result<Oid> {
    let repository = Repository::open_bare(path)?;

    let tree = repo.save_to_tree(&repository)?;

    let signature = repository.signature()?;
    let message = Zoned::now().to_string();

    // TODO Check that the repo is actually based on this commit.
    let parent = match read_head(&repository)? {
        None => None,
        Some(parent) => Some(parent.peel_to_commit()?),
    };
    let parents = match &parent {
        None => vec![],
        Some(parent) => vec![parent],
    };

    let oid = repository.commit(
        Some("HEAD"),
        &signature,
        &signature,
        &message,
        &tree,
        &parents,
    )?;

    Ok(oid)
}
