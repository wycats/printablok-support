use std::{error::Error, path::Path};

use amplify_derive::{From, Wrapper};
use path_abs::{PathDir, PathFile, PathOps};

use crate::{AbsoluteDir, AbsoluteRegularFile};

pub type Fallible<T> = Result<T, Failure>;
pub type Failure = Box<dyn Error>;

#[derive(Debug, From, Wrapper)]
pub struct ExistingRegularFile(PathFile);

impl AsRef<Path> for ExistingRegularFile {
    fn as_ref(&self) -> &Path {
        self.0.as_path()
    }
}

#[derive(Debug, From, Wrapper)]
pub struct ExistingDirectory(PathDir);

impl AsRef<Path> for ExistingDirectory {
    fn as_ref(&self) -> &Path {
        self.0.as_path()
    }
}

impl ExistingDirectory {
    pub fn dir(self, relative: impl AsRef<Path>) -> Fallible<ExistingDirectory> {
        let path = self.0.join(relative.as_ref());
        let path = PathDir::new(path)?;

        Ok(ExistingDirectory(path))
    }

    pub fn file(self, relative: impl AsRef<Path>) -> Fallible<ExistingRegularFile> {
        let path = self.0.join(relative.as_ref());
        let path = PathFile::new(path)?;

        Ok(ExistingRegularFile(path))
    }

    pub fn dirname(self, relative: impl AsRef<Path>) -> Fallible<AbsoluteDir> {
        let path = self.0.join(relative.as_ref());

        Ok(AbsoluteDir::new(path))
    }

    pub fn filename(self, relative: impl AsRef<Path>) -> Fallible<AbsoluteRegularFile> {
        let path = self.0.join(relative.as_ref());

        Ok(AbsoluteRegularFile::new(path))
    }
}
