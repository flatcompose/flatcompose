//! The root the FlatImage executes in.

use std::ffi::OsString;

use serde::{Deserialize, Serialize};

use crate::lower::{Command, Lower};
use crate::violation::{Entry, Violation};

/// The `root` key, naming the root the FlatImage executes in.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Root {
  pub system: String,
}

impl Lower for Root {
  /// Writes `fim-root system use <root>`.
  fn lower(&self) -> Vec<Command> {
    let mut argv: Vec<OsString> = vec!["fim-root".into(), "system".into(), "use".into()];
    argv.push(self.system.clone().into());

    vec![Command::new(argv)]
  }
}

impl Violation for Root {
  fn violations(&self) -> Vec<Entry> {
    Vec::new()
  }
}
