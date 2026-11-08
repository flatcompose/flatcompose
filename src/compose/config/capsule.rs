//! Host driver capsules

use std::ffi::OsString;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::lower::{Command, Lower};
use crate::violation::{Entry, Violation};

/// The `capsule` key, naming the host driver stacks the container links into itself.
#[derive(Deserialize, Serialize)]
#[serde(transparent)]
pub struct Capsule {
  pub entries: Vec<String>,
}

impl Lower for Capsule {
  /// Writes one `fim-capsule set`, one argument per name.
  fn lower(&self) -> Result<Vec<Command>> {
    let mut argv: Vec<OsString> = vec!["fim-capsule".into(), "set".into()];
    argv.extend(self.entries.iter().map(OsString::from));

    Ok(vec![Command::new(argv)])
  }
}

impl Violation for Capsule {
  fn violations(&self) -> Vec<Entry> {
    Vec::new()
  }
}
