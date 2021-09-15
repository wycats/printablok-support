use crate::{utils::ext::Fallible, AbsoluteDir, AbsoluteRegularFile, AsPath};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProjectLocation {
    #[serde(rename = "root")]
    manifest: String,
}

impl ProjectLocation {
    pub fn manifest(&self) -> Fallible<AbsoluteRegularFile> {
        AbsoluteRegularFile::verified(&self.manifest)
    }

    pub fn root(&self) -> Fallible<AbsoluteDir> {
        let manifest = self.manifest()?;
        let parent = manifest
            .as_path()
            .parent()
            .expect("WEIRD: Cargo.toml existed but its parent didn't exist");

        AbsoluteDir::verified(parent)
    }
}
