use std::{error::Error, fmt::Display};

use path_abs::{PathAbs, PathInfo};

use crate::utils::file::FileType;

#[derive(Debug)]
pub enum ProjectError {
    MissingFile { path: PathAbs, kind: FileType },
    UnexpectedPresentFile { path: PathAbs, kind: FileType },
}

impl Display for ProjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectError::MissingFile { path, kind } => {
                write!(f, "Unexpected missing {}: {}", kind, path.display())
            }
            ProjectError::UnexpectedPresentFile { path, kind } => {
                write!(f, "Unexpectedly present {}: {}", kind, path.display())
            }
        }
    }
}

impl Error for ProjectError {}

/**** CLARIFYING CONVENIENCES ****/

/// A Rust function that can fail returns a `Result`, which has:
///
/// - A type for when the function succeeds
/// - A type for when the function fails
///
/// In the case of our `main` function:
///
/// - the successful type is `()` (the "unit" type, which is similar to "void"
///   in other languages, and carries no information).
/// - the error type is `Box<dyn Error>`, which just means "any error".
///
/// Inside of a function that returns a `Result`, the `?` operator after an
/// expression means:
///
/// - If the result is successful, the expression evaluates to the successful
///   type
/// - If the result is an error, return the error immediately (and don't
///   continue running the function).
///
/// In essence, this creates something akin to a typed exception system with
/// explicit markings at the points where errors can occur.
pub type Outcome = Result<Nothing, Failure>;

pub type Nothing = ();
pub type Failure = Box<dyn Error>;

#[allow(non_upper_case_globals)]
pub const Nothing: () = ();
