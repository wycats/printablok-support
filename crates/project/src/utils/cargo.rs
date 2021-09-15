use crate::{
    project_path::{AbsoluteDir, AsPath},
    AbsoluteRegularFile,
};
use cargo::util::important_paths as cargo_paths;

use super::ext::Fallible;

pub struct Workspace {
    pub root: AbsoluteDir,
    pub manifest: AbsoluteRegularFile,
}

impl Workspace {
    pub fn search_from(from: impl AsPath) -> Fallible<Workspace> {
        let manifest = cargo_paths::find_project_manifest_exact(from.as_path(), "Cargo.toml")?;

        let root = manifest.parent().unwrap_or_else(|| {
            panic!("WEIRD: Found Cargo.toml but it didn't have a parent directory")
        });

        Ok(Workspace {
            root: AbsoluteDir::verified(root)?,
            manifest: AbsoluteRegularFile::verified(manifest)?,
        })
    }

    pub fn search() -> Fallible<Workspace> {
        let manifest = cargo_paths::find_project_manifest_exact(
            std::env::current_dir()?.as_ref(),
            "Cargo.toml",
        )?;

        let root = manifest.parent().unwrap_or_else(|| {
            panic!("WEIRD: Found Cargo.toml but it didn't have a parent directory")
        });

        Ok(Workspace {
            root: AbsoluteDir::verified(root)?,
            manifest: AbsoluteRegularFile::verified(manifest)?,
        })
    }
}
