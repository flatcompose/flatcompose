//! The mode the FlatImage executes in.

use std::ffi::OsString;

use serde::Deserialize;

use crate::lower::{Command, Lower};
use crate::violation::{Entry, Violation};

/// The `sandbox` key, naming the mode the FlatImage executes in.
#[derive(Deserialize)]
#[serde(transparent)]
pub struct Sandbox {
  pub mode: String,
}

impl Lower for Sandbox {
  /// Writes `fim-sandbox set <mode>`.
  fn lower(&self) -> Vec<Command> {
    let argv: Vec<OsString> = vec!["fim-sandbox".into(), "set".into(), self.mode.clone().into()];

    vec![Command::new(argv)]
  }
}

impl Violation for Sandbox {
  fn violations(&self) -> Vec<Entry> {
    Vec::new()
  }
}
