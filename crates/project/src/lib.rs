#[macro_use]
pub mod utils;

pub mod project_path;

use std::{
    fmt::{Debug, Display},
    hash::Hash,
    path::Path,
};

use indexmap::IndexMap;
use path_abs::PathDir;
use utils::ext::{ExistingDirectory, ExistingRegularFile, Fallible};

pub use crate::project_path::{dir, file, AsPath};
pub use crate::project_path::{
    AbsoluteDir, AbsolutePath, AbsoluteRegularFile, ProjectPath, RelativePath,
};
pub use crate::utils::cargo::Workspace;
pub use crate::utils::error::{Failure, Nothing, Outcome};

pub trait PathKey: Debug + Display + Eq + Hash + Copy + Send + 'static {}

impl<T> PathKey for T where T: Debug + Display + Eq + Hash + Copy + Send + 'static {}

#[macro_export]
macro_rules! path_key {
    ($ty:ty) => {
        impl $crate::phf_shared::PhfBorrow<Self> for $ty
        where
            $ty: std::borrow::Borrow<Self>,
        {
            fn borrow(&self) -> &Self {
                self.borrow()
            }
        }

        impl $crate::phf_shared::PhfHash for $ty
        where
            $ty: std::hash::Hash,
        {
            fn phf_hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.hash(state)
            }
        }
    };
}

#[derive(Debug)]
pub struct Project<Key>
where
    Key: PathKey,
{
    root: PathDir,
    paths: ProjectPaths<Key>,
}

impl<Key> Project<Key>
where
    Key: PathKey,
{
    pub fn load(root: impl AsRef<Path>, paths: ProjectPaths<Key>) -> Fallible<Project<Key>> {
        Ok(Project {
            root: PathDir::new(root.as_ref())?,
            paths,
        })
    }

    pub fn file(&self, name: Key) -> ExistingRegularFile {
        self.paths.file(&self.root, name)
    }

    pub fn dir(&self, name: Key) -> ExistingDirectory {
        self.paths.dir(&self.root, name)
    }
}

// pub type PathMap<P: ProjectPath> = phf::Map<&'static str, AbsolutePath<P>>;

#[derive(Debug)]
pub struct PathMap<Key, P>
where
    Key: PathKey,
    P: ProjectPath + 'static,
{
    kind: &'static str,
    map: IndexMap<Key, RelativePath<P>>,
}

impl<Key, P> PathMap<Key, P>
where
    P: ProjectPath + 'static,
    Key: PathKey,
{
    pub fn new(kind: &'static str, map: IndexMap<Key, RelativePath<P>>) -> PathMap<Key, P> {
        PathMap { kind, map }
    }

    pub fn get(&self, root: &PathDir, index: Key) -> P::Output {
        let path = &self
            .map
            .get(&index)
            .unwrap_or_else(|| panic!("No {} found for {}", self.kind, index));

        path.assert(root)
            .unwrap_or_else(|e| {
                panic!(
                    "There was an error while getting {} ({}): {}",
                    index, self.kind, e
                )
            })
            .clone()
    }
}

#[derive(Debug)]
pub struct ProjectPaths<Key>
where
    Key: PathKey,
{
    pub regular_file_map: PathMap<Key, AbsoluteRegularFile>,
    pub directory_map: PathMap<Key, AbsoluteDir>,
    // TODO: Glob map
}

pub use indexmap;

#[macro_export]
macro_rules! project {
    (files: { $( $filename:expr => $file:expr ),* }, directories: { $( $dirname:expr => $dir:expr ),* }) => {
        $crate::ProjectPaths {
            regular_file_map: $crate::PathMap::new("regular file", $crate::indexmap::indexmap! { $($filename => $file),* }),
            directory_map: $crate::PathMap::new("directory", $crate::indexmap::indexmap! { $($dirname => $dir),* })
        }
    };
}

impl<Key> ProjectPaths<Key>
where
    Key: PathKey,
{
    fn file(&self, root: &PathDir, name: Key) -> ExistingRegularFile {
        let files = &self.regular_file_map;

        ExistingRegularFile::from(files.get(&root, name))
    }

    fn dir(&self, root: &PathDir, name: Key) -> ExistingDirectory {
        let dirs = &self.directory_map;

        ExistingDirectory::from(dirs.get(&root, name))
    }
}
