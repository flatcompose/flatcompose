//! Sandbox permissions

use std::ffi::OsString;

use serde::Deserialize;

use crate::lower::{Command, Lower};
use crate::violation::{Entry, Violation};

/// The `perms` key, naming the host resources the sandbox grants the FlatImage.
#[derive(Deserialize)]
#[serde(transparent)]
pub struct Perms {
  pub entries: Vec<String>,
}

impl Lower for Perms {
  /// Writes one `fim-perms set`, one argument per name.
  fn lower(&self) -> Vec<Command> {
    let mut argv: Vec<OsString> = vec!["fim-perms".into(), "set".into()];
    argv.extend(self.entries.iter().map(OsString::from));

    vec![Command::new(argv)]
  }
}

impl Violation for Perms {
  fn violations(&self) -> Vec<Entry> {
    Vec::new()
  }
}
