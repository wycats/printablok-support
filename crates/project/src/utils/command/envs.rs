use std::process::Command;

use indexmap::{map::Iter, IndexMap};

use super::ApplyToCommand;

#[derive(Debug, Default)]
pub struct Envs {
    map: IndexMap<String, String>,
}

impl Envs {
    pub fn env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.map.insert(key.into(), value.into());
        self
    }
}

impl ApplyToCommand for Envs {
    fn apply_to_command(&self, command: &mut Command) {
        command.envs(self);
    }
}

impl<'a> IntoIterator for &'a Envs {
    type Item = (&'a String, &'a String);

    type IntoIter = Iter<'a, String, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.iter()
    }
}

impl std::fmt::Display for Envs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let last = self.map.len() - 1;

        for (i, (key, value)) in self.map.iter().enumerate() {
            write!(f, "{}={}", key, value)?;

            if i != last {
                write!(f, " ")?;
            }
        }

        Ok(())
    }
}
