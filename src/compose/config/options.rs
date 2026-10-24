//! Runtime options

use std::ffi::OsString;

use serde::Deserialize;

use crate::lower::{Command, Lower};
use crate::violation::{Entry, Violation};

/// The `options` key. Each entry gives one runtime toggle stored in the FlatImage.
#[derive(Deserialize)]
#[serde(transparent)]
pub struct Options {
  pub entries: Vec<String>,
}

impl Lower for Options {
  /// Writes one `fim-options set`, one argument per name.
  fn lower(&self) -> Vec<Command> {
    let mut argv: Vec<OsString> = vec!["fim-options".into(), "set".into()];
    argv.extend(self.entries.iter().map(OsString::from));

    vec![Command::new(argv)]
  }
}

impl Violation for Options {
  fn violations(&self) -> Vec<Entry> {
    Vec::new()
  }
}
