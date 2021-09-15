pub mod args;
pub mod child;
pub mod envs;
pub mod pipes;

use std::process::{Child, Command, ExitStatus, Output, Stdio};

use amplify_derive::{Display, From};

use crate::{AbsoluteDir, AsPath};

use self::{args::Args, envs::Envs, pipes::CommandPipes};

use super::{error::ProjectError, ext::Fallible};

pub trait ApplyToCommand {
    fn apply_to_command(&self, command: &mut Command);
}

#[derive(Debug, Display, From)]
#[display(WorkingDirectory::display)]
pub enum WorkingDirectory {
    ProcessWorkingDirectory,
    #[from]
    Path(AbsoluteDir),
}

impl ApplyToCommand for WorkingDirectory {
    fn apply_to_command(&self, command: &mut Command) {
        match self {
            WorkingDirectory::ProcessWorkingDirectory => {}
            WorkingDirectory::Path(path) => {
                command.current_dir(path.as_path());
            }
        };
    }
}

impl WorkingDirectory {
    fn display(&self) -> String {
        match self {
            WorkingDirectory::ProcessWorkingDirectory => "process working directory".to_string(),
            WorkingDirectory::Path(path) => path.as_path().to_string_lossy().into(),
        }
    }
}

#[derive(Debug, Display)]
#[display(CommandExecution::display)]
pub struct CommandExecution {
    program: String,
    args: Args,
    current_dir: WorkingDirectory,
    envs: Envs,
    pipes: CommandPipes,
}

impl CommandExecution {
    pub fn program(program: impl Into<String>) -> CommandExecution {
        CommandExecution {
            program: program.into(),
            args: Args::default(),
            current_dir: WorkingDirectory::ProcessWorkingDirectory,
            envs: Envs::default(),
            pipes: CommandPipes::inherit(),
        }
    }

    pub fn args(mut self, args: impl Into<Args>) -> CommandExecution {
        self.args = args.into();
        self
    }

    pub fn working_directory(mut self, directory: impl Into<WorkingDirectory>) -> CommandExecution {
        self.current_dir = directory.into();
        self
    }

    pub fn output(&self) -> Fallible<CommandOutcome> {
        let output = self
            .to_command()
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        Ok(CommandOutcome::wrap(output))
    }

    fn to_command(&self) -> Command {
        let Self {
            program,
            args,
            current_dir,
            envs,
            pipes,
        } = self;

        let mut command = Command::new(program.clone());

        args.apply_to_command(&mut command);
        current_dir.apply_to_command(&mut command);
        envs.apply_to_command(&mut command);
        pipes.apply_to_command(&mut command);

        command
    }

    fn display(&self) -> String {
        self.print(false)
    }

    fn print(&self, verbose: bool) -> String {
        if verbose {
            format!(
                "{} {} (working directory: {}, envs: {}, pipes: {})",
                self.program,
                itertools::join(self.args.iter(), " "),
                self.current_dir.display(),
                self.envs,
                self.pipes
            )
        } else {
            format!(
                "{} {}",
                self.program,
                itertools::join(self.args.iter(), " ")
            )
        }
    }

    pub fn run(self) -> Fallible<CommandOutcome> {
        let mut command = self.to_command();
        let outcome: CommandOutcome = CommandOutcome::wrap(command.output()?);

        Ok(outcome)
    }

    pub fn run_in_background(self) -> Fallible<Child> {
        let mut command = self.to_command();

        ret!(command.spawn())
    }
}

pub trait CommandOutcomeOps {
    fn wrap(output: Output) -> Self;
    fn inner(&self) -> &Output;
    fn into_result(self, command: CommandExecution) -> Result<CommandSuccess, ProjectError>;

    fn status(&self) -> ExitStatus {
        self.inner().status
    }

    fn stdout(&self) -> String {
        String::from_utf8_lossy(&self.inner().stdout).into()
    }

    fn stderr(&self) -> String {
        String::from_utf8_lossy(&self.inner().stderr).into()
    }
}

pub enum CommandOutcome {
    Success(CommandSuccess),
    Failure(CommandFailure),
}

impl CommandOutcomeOps for CommandOutcome {
    fn wrap(output: Output) -> Self {
        if output.status.success() {
            CommandOutcome::Success(CommandSuccess::wrap(output))
        } else {
            CommandOutcome::Failure(CommandFailure::wrap(output))
        }
    }

    fn inner(&self) -> &Output {
        match self {
            CommandOutcome::Success(success) => success.inner(),
            CommandOutcome::Failure(failure) => failure.inner(),
        }
    }

    fn into_result(self, command: CommandExecution) -> Result<CommandSuccess, ProjectError> {
        match self {
            CommandOutcome::Success(success) => success.into_result(command),
            CommandOutcome::Failure(failure) => failure.into_result(command),
        }
    }
}

#[derive(Debug)]
pub struct CommandSuccess {
    inner: Output,
}

impl CommandOutcomeOps for CommandSuccess {
    fn wrap(output: Output) -> Self {
        CommandSuccess { inner: output }
    }

    fn inner(&self) -> &Output {
        &self.inner
    }

    fn into_result(self, _command: CommandExecution) -> Result<CommandSuccess, ProjectError> {
        Ok(self)
    }
}

pub struct CommandFailure {
    inner: Output,
}

impl CommandOutcomeOps for CommandFailure {
    fn inner(&self) -> &Output {
        &self.inner
    }

    fn wrap(output: Output) -> Self {
        CommandFailure { inner: output }
    }

    fn into_result(self, command: CommandExecution) -> Result<CommandSuccess, ProjectError> {
        Err(ProjectError::FailedExec {
            command,
            code: self
                .status()
                .code()
                .expect("WEIRD: status code was missing in CommandOutcome"),
            stdout: self.stdout(),
            stderr: self.stderr(),
        })
    }
}

pub mod outcome {
    pub use super::{CommandFailure, CommandOutcome, CommandOutcomeOps, CommandSuccess};
}
