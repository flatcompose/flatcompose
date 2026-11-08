//! The `steps` block

use anyhow::Result;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};

use crate::compose::steps::{Builder, Exec};
use crate::lower::{Command, Lower};
use crate::violation::{Entry, Violation};

/// The `steps` block, one command per entry, run in the order the file writes them.
#[derive(Default, Serialize)]
#[serde(transparent)]
pub struct Steps {
  pub entries: Vec<Step>,
}

impl<'de> Deserialize<'de> for Steps {
  /// Reads the entries and numbers them, so each `exec` names its script by its step position.
  fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Steps, D::Error> {
    let mut entries = Vec::<Step>::deserialize(deserializer)?;

    for (position, step) in entries.iter_mut().enumerate() {
      if let Some(exec) = &mut step.exec {
        exec.position = position;
      }
    }

    Ok(Steps { entries })
  }
}

impl Steps {
  /// Whether the block states no entry, which a written file leaves out.
  pub fn is_empty(&self) -> bool {
    self.entries.is_empty()
  }
}

impl Lower for Steps {
  /// Writes the commands of every entry, in the order the file writes them.
  fn lower(&self) -> Result<Vec<Command>> {
    let mut commands = Vec::new();

    for step in &self.entries {
      commands.extend(step.lower()?);
    }

    Ok(commands)
  }
}

impl Violation for Steps {
  /// Refuses a builder with an empty pattern list and an exec with an empty script.
  fn violations(&self) -> Vec<Entry> {
    let mut entries = Vec::new();

    for (index, step) in self.entries.iter().enumerate() {
      if let Some(builder) = &step.builder {
        if builder.install.patterns.is_empty() {
          entries.push(Entry::of(
            format!("steps[{index}].builder.install.patterns"),
            "is empty, so the builder installs nothing",
          ));
        }
      }

      if let Some(exec) = &step.exec {
        if exec.script.is_empty() {
          entries.push(Entry::of(format!("steps[{index}].exec.script"), "is empty, so the step runs nothing"));
        }
      }
    }

    entries
  }
}

/// One entry of `steps`, naming the command the build runs with the key `builder` or `exec`.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Step {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub builder: Option<Builder>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub exec: Option<Exec>,
}

impl Lower for Step {
  /// Writes the commands the entry states, the builder's before the exec's.
  fn lower(&self) -> Result<Vec<Command>> {
    let mut commands = Vec::new();

    if let Some(builder) = &self.builder {
      commands.extend(builder.lower()?);
    }

    if let Some(exec) = &self.exec {
      commands.extend(exec.lower()?);
    }

    Ok(commands)
  }
}
