//! The `builder` command

use std::ffi::OsString;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::compose::image::Arch;
use crate::lower::{Command, Lower};

/// The `builder` key, which states the `fim-root builder` command that installs a distribution root.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Builder {
  pub from: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub arch: Option<Arch>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub http_retries: Option<u32>,
  pub install: Install,
}

impl Lower for Builder {
  /// Writes one `fim-root builder`, with the builder options first and the install request last.
  fn lower(&self) -> Result<Vec<Command>> {
    let mut argv: Vec<OsString> = vec!["fim-root".into(), "builder".into()];

    argv.push("--from".into());
    argv.push(self.from.clone().into());

    if let Some(arch) = &self.arch {
      argv.push("--arch".into());
      argv.push(arch.token().into());
    }

    if let Some(http_retries) = self.http_retries {
      argv.push("--http-retries".into());
      argv.push(http_retries.to_string().into());
    }

    argv.extend(self.install.arguments());

    Ok(vec![Command::new(argv)])
  }
}

/// The `install` key, with the patterns the builder installs and the options of the request.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Install {
  pub patterns: Vec<String>,
  #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
  pub pattern_type: Option<String>,
  #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
  pub pattern_match: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub with: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub exclude: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub postinstall: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub parallel: Option<u32>,
  #[serde(default, skip_serializing_if = "Install::is_false")]
  pub no_deps: bool,
}

impl Install {
  /// Writes the install request, the options the file writes first and the patterns closing it.
  fn arguments(&self) -> Vec<OsString> {
    let mut argv: Vec<OsString> = vec!["install".into()];

    if let Some(pattern_type) = &self.pattern_type {
      argv.push("--type".into());
      argv.push(pattern_type.clone().into());
    }

    if let Some(pattern_match) = &self.pattern_match {
      argv.push("--match".into());
      argv.push(pattern_match.clone().into());
    }

    if let Some(kinds) = &self.with {
      argv.push("--with".into());
      argv.push(kinds.join(",").into());
    }

    if let Some(packages) = &self.exclude {
      argv.push("--exclude".into());
      argv.push(packages.join(",").into());
    }

    if let Some(phases) = &self.postinstall {
      argv.push("--postinstall".into());
      argv.push(phases.join(",").into());
    }

    if let Some(parallel) = self.parallel {
      argv.push("--parallel".into());
      argv.push(parallel.to_string().into());
    }

    if self.no_deps {
      argv.push("--no-deps".into());
    }

    argv.extend(self.patterns.iter().map(OsString::from));

    argv
  }

  /// Whether `value` equals `false`
  fn is_false(value: &bool) -> bool {
    !*value
  }
}
