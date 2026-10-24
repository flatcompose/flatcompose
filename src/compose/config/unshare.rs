//! The namespaces the sandbox detaches

use std::ffi::OsString;

use serde::{Deserialize, Serialize};

use crate::lower::{Command, Lower};
use crate::violation::{Entry, Violation};

/// The `unshare` key, naming the kernel namespaces the sandbox detaches.
#[derive(Deserialize, Serialize)]
#[serde(transparent)]
pub struct Unshare {
  pub entries: Vec<String>,
}

impl Lower for Unshare {
  /// Writes one `fim-unshare set`, one argument per name.
  fn lower(&self) -> Vec<Command> {
    let mut argv: Vec<OsString> = vec!["fim-unshare".into(), "set".into()];
    argv.extend(self.entries.iter().map(OsString::from));

    vec![Command::new(argv)]
  }
}

impl Violation for Unshare {
  fn violations(&self) -> Vec<Entry> {
    Vec::new()
  }
}
