//! Environment variables

use std::ffi::OsString;

use serde::Deserialize;
use serde::de::{Deserializer, Error as _};

use crate::lower::{Command, Lower};
use crate::violation::{Entry, Violation};

/// The `env` key, holding one variable per mapping pair.
pub struct Env {
  pub entries: Vec<Variable>,
}

impl<'de> Deserialize<'de> for Env {
  /// Reads the mapping one entry per pair, keeping the order the file writes.
  fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Env, D::Error> {
    let mapping = serde_yaml::Mapping::deserialize(deserializer)?;
    let mut entries = Vec::with_capacity(mapping.len());

    for (key, value) in mapping {
      let name = match key {
        serde_yaml::Value::String(name) => name,
        _ => return Err(D::Error::custom("`env` expects variable names as strings")),
      };

      let value = match value {
        serde_yaml::Value::String(value) => value,
        _ => return Err(D::Error::custom(format!("variable `{name}` expects a string value"))),
      };

      entries.push(Variable { name, value });
    }

    Ok(Env { entries })
  }
}

impl Lower for Env {
  /// Writes one `fim-env set`, one `NAME=VALUE` argument per variable.
  fn lower(&self) -> Vec<Command> {
    let mut argv: Vec<OsString> = vec!["fim-env".into(), "set".into()];

    for variable in &self.entries {
      argv.push(format!("{}={}", variable.name, variable.value).into());
    }

    vec![Command::new(argv)]
  }
}

impl Violation for Env {
  fn violations(&self) -> Vec<Entry> {
    Vec::new()
  }
}

/// One `NAME=VALUE` pair the FlatImage carries.
pub struct Variable {
  pub name: String,
  pub value: String,
}
