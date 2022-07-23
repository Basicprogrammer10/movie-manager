use std::path::PathBuf;

use super::ShowInterface;

pub struct NewShow {
    pub name: String,
    pub path: PathBuf,
}

impl NewShow {
    pub fn new(name: String, path: PathBuf) -> Self {
        Self { name, path }
    }
}

impl ShowInterface for NewShow {
    fn name(&self) -> String {
        format!("*{}", self.name)
    }

    fn path(&self) -> PathBuf {
        self.path.clone()
    }
}
