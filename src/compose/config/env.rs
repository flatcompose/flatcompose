//! Environment variables

use std::ffi::OsString;

use serde::de::{Deserializer, Error as _};
use serde::ser::{SerializeMap, Serializer};
use serde::{Deserialize, Serialize};

use crate::lower::{Command, Lower};
use crate::violation::{Entry, Violation};

/// The `env` key, with one variable per mapping pair.
pub struct Env {
  pub entries: Vec<Variable>,
}

impl<'de> Deserialize<'de> for Env {
  /// Reads the mapping one entry per pair, in the order the file writes.
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

impl Serialize for Env {
  /// Writes the mapping one entry per variable, in the order `entries` lists them.
  fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    let mut map = serializer.serialize_map(Some(self.entries.len()))?;

    for variable in &self.entries {
      map.serialize_entry(&variable.name, &variable.value)?;
    }

    map.end()
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

/// One `NAME=VALUE` pair stored in the FlatImage.
#[derive(Deserialize, Serialize)]
pub struct Variable {
  pub name: String,
  pub value: String,
}
