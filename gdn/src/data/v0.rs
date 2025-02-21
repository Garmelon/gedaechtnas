// Just the empty directory

use super::{LockedDataDir, v1};

pub const VERSION: u32 = 0;

pub fn migrate(dir: &LockedDataDir) -> anyhow::Result<()> {
    dir.require_version(VERSION)?;
    v1::save_state(dir, &v1::State::default())?;
    dir.write_version(v1::VERSION)?;
    Ok(())
}
