//! The overlay backend the FlatImage uses.

use std::ffi::OsString;

use serde::{Deserialize, Serialize};

use crate::lower::{Command, Lower};
use crate::violation::{Entry, Violation};

/// The `overlay` key, naming the overlay backend.
#[derive(Deserialize, Serialize)]
#[serde(transparent)]
pub struct Overlay {
  pub backend: String,
}

impl Lower for Overlay {
  /// Writes `fim-overlay set <backend>`.
  fn lower(&self) -> Vec<Command> {
    let argv: Vec<OsString> = vec!["fim-overlay".into(), "set".into(), self.backend.clone().into()];

    vec![Command::new(argv)]
  }
}

impl Violation for Overlay {
  fn violations(&self) -> Vec<Entry> {
    Vec::new()
  }
}
