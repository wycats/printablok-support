#[derive(Debug)]
pub enum FileType {
    Directory,
    RegularFile,
    Other { description: &'static str },
}

impl std::fmt::Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FileType::Directory => "directory",
                FileType::RegularFile => "regular file",
                FileType::Other { description } => description,
            }
        )
    }
}
