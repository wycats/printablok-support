use std::{
    fmt::{Debug, Display},
    fs::File,
    process::{Command, Stdio},
};

use super::ApplyToCommand;

#[derive(Debug)]
pub enum CommandPipe {
    Inherit,
    Nowhere,
    MakePipe,
    #[cfg(any(windows, unix))]
    File(FileFn),
}

pub struct FileFn {
    inner: Box<dyn Fn() -> File>,
}

impl Debug for FileFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "() -> File")
    }
}

#[derive(Debug)]
pub enum StdPipe {
    Stdin,
    Stdout,
    Stderr,
}

impl CommandPipe {
    pub fn inherit() -> CommandPipe {
        CommandPipe::Inherit
    }

    pub fn nowhere() -> CommandPipe {
        CommandPipe::Nowhere
    }

    pub fn pipe() -> CommandPipe {
        CommandPipe::MakePipe
    }

    pub fn file(file: impl Fn() -> File + 'static) -> CommandPipe {
        CommandPipe::File(FileFn {
            inner: Box::new(file),
        })
    }

    fn apply(&self, command: &mut Command, pipe: StdPipe) {
        let stdio = match self {
            CommandPipe::Inherit => Stdio::inherit(),
            CommandPipe::Nowhere => Stdio::null(),
            CommandPipe::MakePipe => Stdio::piped(),
            CommandPipe::File(file) => Stdio::from((file.inner)()),
        };

        match pipe {
            StdPipe::Stdin => command.stdin(stdio),
            StdPipe::Stdout => command.stdout(stdio),
            StdPipe::Stderr => command.stderr(stdio),
        };
    }
}

impl Display for CommandPipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CommandPipe::Inherit => "inherit",
                CommandPipe::Nowhere => "nowhere",
                CommandPipe::MakePipe => "a new pipe",
                CommandPipe::File(_) => "file",
            }
        )
    }
}

#[derive(Debug)]
pub struct CommandPipes {
    stdin: CommandPipe,
    stdout: CommandPipe,
    stderr: CommandPipe,
}

impl Display for CommandPipes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.stdin, &self.stdout, &self.stderr) {
            (CommandPipe::Inherit, CommandPipe::Inherit, CommandPipe::Inherit) => {
                write!(f, "inherit")
            }
            (CommandPipe::Nowhere, CommandPipe::Nowhere, CommandPipe::Nowhere) => {
                write!(f, "nowhere")
            }
            (CommandPipe::MakePipe, CommandPipe::MakePipe, CommandPipe::MakePipe) => {
                write!(f, "a new pipe")
            }
            (stdin, stdout, stderr) => {
                write!(f, "stdin={}, stdout={}, stderr={}", stdin, stdout, stderr)
            }
        }
    }
}

impl CommandPipes {
    pub fn inherit() -> Self {
        Self {
            stdin: CommandPipe::Inherit,
            stdout: CommandPipe::Inherit,
            stderr: CommandPipe::Inherit,
        }
    }

    pub fn stdin(mut self, stdin: CommandPipe) -> CommandPipes {
        self.stdin = stdin;
        self
    }

    pub fn stdout(mut self, stdout: CommandPipe) -> CommandPipes {
        self.stdout = stdout;
        self
    }

    pub fn stderr(mut self, stderr: CommandPipe) -> CommandPipes {
        self.stderr = stderr;
        self
    }
}

impl ApplyToCommand for CommandPipes {
    fn apply_to_command(&self, command: &mut Command) {
        let Self {
            stdin,
            stdout,
            stderr,
        } = self;

        stdin.apply(command, StdPipe::Stdin);
        stdout.apply(command, StdPipe::Stdout);
        stderr.apply(command, StdPipe::Stderr);
    }
}
