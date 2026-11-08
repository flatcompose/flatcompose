//! The mode the FlatImage executes in.

use std::ffi::OsString;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::lower::{Command, Lower};
use crate::violation::{Entry, Violation};

/// The `sandbox` key, naming the mode the FlatImage executes in.
#[derive(Deserialize, Serialize)]
#[serde(transparent)]
pub struct Sandbox {
  pub mode: String,
}

impl Lower for Sandbox {
  /// Writes `fim-sandbox set <mode>`.
  fn lower(&self) -> Result<Vec<Command>> {
    let argv: Vec<OsString> = vec!["fim-sandbox".into(), "set".into(), self.mode.clone().into()];

    Ok(vec![Command::new(argv)])
  }
}

impl Violation for Sandbox {
  fn violations(&self) -> Vec<Entry> {
    Vec::new()
  }
}
