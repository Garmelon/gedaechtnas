use super::v1;

pub const VERSION: u32 = 0;

pub struct Repo;

impl Repo {
    pub fn load() -> Self {
        Self
    }

    pub fn migrate(self) -> super::Repo {
        v1::Repo { notes: vec![] }.migrate()
    }
}
