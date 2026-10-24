//! The desktop configuration

use std::ffi::OsString;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::lower::{Command, Lower};
use crate::violation::{Entry, Violation};

/// The `desktop` key, naming each asset and its integration items.
#[derive(Default, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct Desktop {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub manifest: Option<PathBuf>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub icon: Option<PathBuf>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub entry: Option<PathBuf>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mimetype: Option<PathBuf>,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub integrations: Vec<String>,
}

impl Lower for Desktop {
  fn lower(&self) -> Vec<Command> {
    let mut commands = Vec::new();
    let assets = [
      ("manifest", self.manifest.as_ref()),
      ("icon", self.icon.as_ref()),
      ("entry", self.entry.as_ref()),
      ("mimetype", self.mimetype.as_ref()),
    ];

    for (asset, path_file_asset) in assets {
      let Some(path_file_asset) = path_file_asset else {
        continue;
      };

      let argv: Vec<OsString> = vec![
        "fim-desktop".into(),
        asset.into(),
        "set".into(),
        path_file_asset.clone().into_os_string(),
      ];

      commands.push(Command::new(argv));
    }

    if !self.integrations.is_empty() {
      let mut argv: Vec<OsString> = vec!["fim-desktop".into(), "integrations".into(), "set".into()];
      argv.extend(self.integrations.iter().map(OsString::from));

      commands.push(Command::new(argv));
    }

    commands
  }
}

impl Violation for Desktop {
  fn violations(&self) -> Vec<Entry> {
    Vec::new()
  }
}
