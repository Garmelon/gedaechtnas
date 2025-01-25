use std::path::PathBuf;

use directories::ProjectDirs;

pub struct Paths(ProjectDirs);

impl Paths {
    pub fn on_linux() -> Option<Self> {
        ProjectDirs::from("de", "plugh", crate::TECHNICAL_NAME).map(Self)
    }

    pub fn on_windows() -> Option<Self> {
        ProjectDirs::from("de", "plugh", crate::PROPER_NAME).map(Self)
    }

    pub fn state_file(&self) -> PathBuf {
        self.0.data_local_dir().join("state.json")
    }

    pub fn repos_dir(&self) -> PathBuf {
        self.0.data_local_dir().join("repos")
    }

    pub fn repo_dir(&self, name: &str) -> PathBuf {
        self.repos_dir().join(format!("{name}.git"))
    }
}
