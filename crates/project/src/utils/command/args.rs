use super::ApplyToCommand;

use amplify_derive::Display;

#[derive(Debug, Default, Display)]
#[display(Args::display)]
pub struct Args {
    inner: Vec<String>,
}

impl<T, R> From<T> for Args
where
    T: IntoIterator<Item = R>,
    R: Into<String>,
{
    fn from(iterable: T) -> Self {
        Args {
            inner: iterable.into_iter().map(|item| item.into()).collect(),
        }
    }
}

impl ApplyToCommand for Args {
    fn apply_to_command(&self, command: &mut std::process::Command) {
        command.args(&self.inner);
    }
}

impl Args {
    pub fn add(mut self, arg: String) -> Args {
        self.inner.push(arg);
        self
    }

    fn display(&self) -> String {
        itertools::join(&self.inner, " ")
    }

    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.inner.iter().map(|item| item.as_ref())
    }
}
