//! The `steps` block

use serde::{Deserialize, Serialize};

use crate::compose::steps::Builder;
use crate::lower::{Command, Lower};
use crate::violation::{Entry, Violation};

/// The `steps` block, one command per entry, run in the order the file writes them.
#[derive(Default, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Steps {
  pub entries: Vec<Step>,
}

impl Steps {
  /// Whether the block states no entry, which a written file leaves out.
  pub fn is_empty(&self) -> bool {
    self.entries.is_empty()
  }
}

impl Lower for Steps {
  /// Writes the command of every entry, in the order the file writes them.
  fn lower(&self) -> Vec<Command> {
    let mut commands = Vec::new();

    for step in &self.entries {
      commands.extend(step.lower());
    }

    commands
  }
}

impl Violation for Steps {
  /// Refuses a builder whose install has an empty pattern list.
  fn violations(&self) -> Vec<Entry> {
    let mut entries = Vec::new();

    for (index, step) in self.entries.iter().enumerate() {
      if step.builder.install.patterns.is_empty() {
        entries.push(Entry::of(
          format!("steps[{index}].builder.install.patterns"),
          "is empty, so the builder installs nothing",
        ));
      }
    }

    entries
  }
}

/// One entry of `steps`, naming the command the build runs with the key `builder`.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Step {
  pub builder: Builder,
}

impl Lower for Step {
  /// Writes the entry's one command.
  fn lower(&self) -> Vec<Command> {
    self.builder.lower()
  }
}
