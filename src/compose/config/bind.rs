//! Host bindings

use std::ffi::OsString;
use std::path::PathBuf;

use serde::Deserialize;

use crate::lower::{Command, Lower};
use crate::violation::{Entry, Violation};

/// The `bind` key, holding one binding per list entry.
#[derive(Deserialize)]
#[serde(transparent)]
pub struct Bind {
  pub entries: Vec<Binding>,
}

impl Lower for Bind {
  /// Writes one `fim-bind set`, one `src:dst:mode` argument per binding.
  fn lower(&self) -> Vec<Command> {
    let mut argv: Vec<OsString> = vec!["fim-bind".into(), "set".into()];

    for binding in &self.entries {
      let argument = format!("{}:{}:{}", binding.src.display(), binding.dst.display(), binding.mode.token());
      argv.push(argument.into());
    }

    vec![Command::new(argv)]
  }
}

impl Violation for Bind {
  fn violations(&self) -> Vec<Entry> {
    Vec::new()
  }
}

/// One host path binding, with its source, destination and mode.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Binding {
  pub src: PathBuf,
  pub dst: PathBuf,
  pub mode: Mode,
}

/// The mode one binding grants.
#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
  Ro,
  Rw,
  Dev,
}

impl Mode {
  /// The token the `src:dst:mode` argument carries.
  pub fn token(&self) -> &'static str {
    match self {
      Mode::Ro => "ro",
      Mode::Rw => "rw",
      Mode::Dev => "dev",
    }
  }
}
