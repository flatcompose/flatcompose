//! Host bindings

use std::ffi::OsString;
use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::lower::{Command, Lower};
use crate::violation::{Entry, Violation};

/// The `bind` key, with one binding per list entry.
#[derive(Deserialize, Serialize)]
#[serde(transparent)]
pub struct Bind {
  pub entries: Vec<Binding>,
}

impl Lower for Bind {
  /// Writes one `fim-bind set`, one `src:dst:mode` argument per binding.
  fn lower(&self) -> Result<Vec<Command>> {
    let mut argv: Vec<OsString> = vec!["fim-bind".into(), "set".into()];

    for binding in &self.entries {
      let argument = format!("{}:{}:{}", binding.src.display(), binding.dst.display(), binding.mode.token());
      argv.push(argument.into());
    }

    Ok(vec![Command::new(argv)])
  }
}

impl Violation for Bind {
  fn violations(&self) -> Vec<Entry> {
    Vec::new()
  }
}

/// One host path binding, with its source, destination and mode.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Binding {
  pub src: PathBuf,
  pub dst: PathBuf,
  pub mode: Mode,
}

/// The mode one binding grants.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
  Ro,
  Rw,
  Dev,
}

impl Mode {
  /// The token that fills the mode slot of a `src:dst:mode` argument.
  pub fn token(&self) -> &'static str {
    match self {
      Mode::Ro => "ro",
      Mode::Rw => "rw",
      Mode::Dev => "dev",
    }
  }
}
