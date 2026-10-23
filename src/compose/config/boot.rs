//! The startup command

use std::ffi::OsString;
use std::path::PathBuf;

use serde::Deserialize;

use crate::lower::{Command, Lower};
use crate::violation::{Entry, Violation};

/// The `boot` key, naming the program and its arguments.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Boot {
  pub program: PathBuf,
  #[serde(default)]
  pub args: Vec<String>,
}

impl Lower for Boot {
  /// Writes one `fim-boot set`
  fn lower(&self) -> Vec<Command> {
    let mut argv: Vec<OsString> = vec!["fim-boot".into(), "set".into()];
    argv.push(self.program.clone().into_os_string());
    argv.extend(self.args.iter().map(OsString::from));

    vec![Command::new(argv)]
  }
}

impl Violation for Boot {
  fn violations(&self) -> Vec<Entry> {
    Vec::new()
  }
}
