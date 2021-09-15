pub mod location;

use std::path::Path;

use crate::{
    project_path::AbsoluteDir,
    utils::command::{CommandExecution, CommandOutcomeOps},
};

use self::location::ProjectLocation;

use super::ext::Fallible;

pub struct Workspace {
    pub cargo: ProjectLocation,
}

impl Workspace {
    pub fn search_from(from: impl AsRef<Path>) -> Fallible<Workspace> {
        let project = locate_project(AbsoluteDir::verified(from)?)?;

        Ok(Workspace { cargo: project })
    }

    pub fn search() -> Fallible<Workspace> {
        let project = locate_project(AbsoluteDir::verified(std::env::current_dir()?)?)?;

        Ok(Workspace { cargo: project })
    }
}

fn locate_project(working_directory: impl Into<AbsoluteDir>) -> Fallible<ProjectLocation> {
    let command = CommandExecution::program("cargo")
        .args(["locate-project", "--workspace"])
        .working_directory(working_directory.into());

    let output = command.output()?.into_result(command)?;

    Ok(serde_json::from_str(&output.stdout())?)
}
