use std::{
    fmt::{Debug, Display},
    path::{Path, PathBuf},
};

use amplify_derive::Display;
use path_abs::{PathAbs, PathDir, PathFile, PathInfo, PathOps};
use serde::{Deserialize, Serialize};

use crate::utils::{
    error::ProjectError,
    ext::{Failure, Fallible},
    file::FileType,
};

#[derive(Debug, Copy, Clone)]
pub enum FileExpectation {
    /// Expect this path to exist. If it doesn't exist, it's an error.
    MustAlreadyExist,
    /// Expect this path not to exist. If it exists, it's an error.
    MayNotAlreadyExist,
    /// This path may or may not exist. If it doesn't exist, make it.
    Touch,
}

impl Default for FileExpectation {
    fn default() -> Self {
        FileExpectation::Touch
    }
}

pub type Touch<T> = fn(&PathAbs) -> Result<T, Failure>;

#[derive(Debug, Clone, Display)]
#[display(AbsoluteRegularFile::print)]
pub struct AbsoluteRegularFile {
    path: PathAbs,
}

impl AbsoluteRegularFile {
    pub fn new(path: impl Into<PathAbs>) -> AbsoluteRegularFile {
        AbsoluteRegularFile { path: path.into() }
    }

    pub fn verified(path: impl AsRef<Path>) -> Fallible<AbsoluteRegularFile> {
        let path = PathFile::new(path.as_ref())?;

        Ok(AbsoluteRegularFile { path: path.into() })
    }

    pub fn verify(self) -> Fallible<AbsoluteRegularFile> {
        AbsoluteRegularFile::verified(self.path)
    }
}

#[derive(Clone, Display)]
#[display(AbsoluteDir::print)]
pub struct AbsoluteDir {
    path: PathAbs,
}

impl Debug for AbsoluteDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AbsoluteDir")
            .field("path", &self.path)
            .field("touch_fn", &"<touch>")
            .finish()
    }
}

impl AbsoluteDir {
    pub fn new(path: impl Into<PathAbs>) -> AbsoluteDir {
        AbsoluteDir { path: path.into() }
    }

    pub fn verified(path: impl AsRef<Path>) -> Fallible<AbsoluteDir> {
        let dir = PathDir::new(path.as_ref())?;

        Ok(AbsoluteDir { path: dir.into() })
    }

    pub fn verify(self) -> Fallible<AbsoluteDir> {
        AbsoluteDir::verified(self.path)
    }
}

pub trait ProjectPath: Debug + Clone {
    type Output: PathInfo + Clone;
    const KIND: FileType;

    fn path(&self) -> &PathAbs;
    fn default_touch() -> Touch<Self::Output>;
    fn to_existing(&self) -> Fallible<Self::Output>;
    fn from(path: PathAbs) -> Fallible<Self>;
    fn print(&self) -> String;

    fn check(&self, expectation: FileExpectation) -> FileStatus {
        let path = self.path();
        let exists = path.exists();

        match (expectation, exists) {
            (FileExpectation::MustAlreadyExist, true) => FileStatus::Exists,
            (FileExpectation::MustAlreadyExist, false) => FileStatus::DoesNotExistError,
            (FileExpectation::MayNotAlreadyExist, true) => FileStatus::DoesExistError,
            (FileExpectation::MayNotAlreadyExist, false) => FileStatus::ShouldTouch,
            (FileExpectation::Touch, true) => FileStatus::Exists,
            (FileExpectation::Touch, false) => FileStatus::ShouldTouch,
        }
    }

    fn touch(&self, touch: Touch<Self::Output>) -> Fallible<Self::Output> {
        touch(self.path())
    }

    fn assert(
        &self,
        expectation: FileExpectation,
        touch: Touch<Self::Output>,
    ) -> Fallible<Self::Output> {
        match self.check(expectation) {
            FileStatus::Exists => self.to_existing(),
            FileStatus::ShouldTouch => touch(self.path()),
            FileStatus::DoesNotExistError => fail!(ProjectError::MissingFile {
                path: self.path().clone(),
                kind: Self::KIND,
            }),
            FileStatus::DoesExistError => fail!(ProjectError::UnexpectedPresentFile {
                path: self.path().clone(),
                kind: Self::KIND
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub enum FileStatus {
    Exists,
    ShouldTouch,
    DoesNotExistError,
    DoesExistError,
}

impl ProjectPath for AbsoluteRegularFile {
    type Output = PathFile;
    const KIND: FileType = FileType::RegularFile;

    fn path(&self) -> &PathAbs {
        &self.path
    }

    fn default_touch() -> Touch<PathFile> {
        pub fn default_touch(path: &PathAbs) -> Fallible<PathFile> {
            std::fs::File::create(&path)?;

            Ok(PathFile::new(path)?)
        }

        default_touch
    }

    fn to_existing(&self) -> Fallible<PathFile> {
        ret!(PathFile::new(&self.path))
    }

    fn from(path: PathAbs) -> Fallible<Self> {
        Ok(AbsoluteRegularFile { path })
    }

    fn print(&self) -> String {
        self.path.display().to_string()
    }
}

impl ProjectPath for AbsoluteDir {
    type Output = PathDir;
    const KIND: FileType = FileType::Directory;

    fn path(&self) -> &PathAbs {
        &self.path
    }

    fn default_touch() -> Touch<PathDir> {
        fn default_touch(path: &PathAbs) -> Fallible<PathDir> {
            std::fs::create_dir_all(path)?;

            Ok(PathDir::new(path)?)
        }

        default_touch
    }

    fn to_existing(&self) -> Fallible<PathDir> {
        ret!(PathDir::new(&self.path))
    }

    fn from(path: PathAbs) -> Fallible<Self> {
        Ok(AbsoluteDir { path })
    }

    fn print(&self) -> String {
        self.path.display().to_string()
    }
}

#[derive(Clone)]
pub struct RelativePath<P>
where
    P: ProjectPath,
{
    path: PathBuf,
    touch_fn: Touch<P::Output>,
    expectation: FileExpectation,
}

impl<P> Debug for RelativePath<P>
where
    P: ProjectPath,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RelativePath")
            .field("path", &self.path)
            .field("touch_fn", &"<touch>")
            .field("expectation", &self.expectation)
            .finish()
    }
}

impl<P> RelativePath<P>
where
    P: ProjectPath,
{
    pub fn must_exist(self) -> Self {
        Self {
            expectation: FileExpectation::MustAlreadyExist,
            ..self
        }
    }

    pub fn create(self) -> Self {
        Self {
            expectation: FileExpectation::MayNotAlreadyExist,
            ..self
        }
    }

    pub fn with_touch_fn(self, touch_fn: Touch<P::Output>) -> Self {
        Self { touch_fn, ..self }
    }

    pub fn assert(&self, root: &PathDir) -> Fallible<P::Output> {
        let path = root.concat(&self.path)?;
        let abs = AbsolutePath {
            path: P::from(path)?,
            touch_fn: self.touch_fn,
            expectation: self.expectation,
        };

        abs.assert()
    }
}

pub fn file(path: impl AsRef<Path>) -> RelativePath<AbsoluteRegularFile> {
    RelativePath {
        path: path.as_ref().to_owned(),
        touch_fn: AbsoluteRegularFile::default_touch(),
        expectation: FileExpectation::default(),
    }
}

pub fn dir(path: impl AsRef<Path>) -> RelativePath<AbsoluteDir> {
    RelativePath {
        path: path.as_ref().to_owned(),
        touch_fn: AbsoluteDir::default_touch(),
        expectation: FileExpectation::default(),
    }
}

#[derive(Clone, Display)]
#[display(AbsolutePath::print)]
pub struct AbsolutePath<P>
where
    P: ProjectPath,
{
    path: P,
    touch_fn: Touch<P::Output>,
    expectation: FileExpectation,
}

impl<P> Debug for AbsolutePath<P>
where
    P: ProjectPath,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AbsolutePath")
            .field("path", &self.path)
            .field("touch_fn", &"<touch>")
            .field("expectation", &self.expectation)
            .finish()
    }
}

impl<P> AbsolutePath<P>
where
    P: ProjectPath,
{
    pub fn new<Q: ProjectPath>(path: Q) -> AbsolutePath<Q> {
        AbsolutePath {
            path: path.into(),
            touch_fn: Q::default_touch(),
            expectation: FileExpectation::Touch,
        }
    }

    pub fn must_exist(self) -> Self {
        Self {
            expectation: FileExpectation::MustAlreadyExist,
            ..self
        }
    }

    pub fn create(self) -> Self {
        Self {
            expectation: FileExpectation::MayNotAlreadyExist,
            ..self
        }
    }

    pub fn with_touch_fn(self, touch_fn: Touch<P::Output>) -> Self {
        Self { touch_fn, ..self }
    }

    pub fn assert(&self) -> Fallible<P::Output> {
        self.path.assert(self.expectation, self.touch_fn)
    }

    fn print(&self) -> String {
        self.path.print()
    }
}

pub trait AsPath: Display {
    fn as_path(&self) -> &Path;
}

macro_rules! as_path {
    ($ty:ty) => {
        impl AsPath for $ty {
            fn as_path(&self) -> &Path {
                self.path.as_path()
            }
        }

        impl AsRef<Path> for $ty {
            fn as_ref(&self) -> &Path {
                self.path.as_path()
            }
        }
    };
}

// impl AsPath for Path {
//     fn as_path(&self) -> &Path {
//         self
//     }
// }

// impl AsPath for PathBuf {
//     fn as_path(&self) -> &Path {
//         self.as_path()
//     }
// }

as_path!(AbsoluteDir);
as_path!(AbsoluteRegularFile);
as_path!(AbsolutePath<AbsoluteDir>);
as_path!(AbsolutePath<AbsoluteRegularFile>);
