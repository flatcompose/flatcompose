//! The `config` block

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::compose::config::{Bind, Boot, Capsule, Desktop, Env, Options, Overlay, Perms, Root, Sandbox, Unshare};
use crate::lower::{Command, Lower};
use crate::violation::{Entry, Violation};

/// The `config` block, carrying the entities of the keys the file writes.
#[derive(Default, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub root: Option<Root>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub perms: Option<Perms>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub options: Option<Options>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub unshare: Option<Unshare>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub capsule: Option<Capsule>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub overlay: Option<Overlay>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sandbox: Option<Sandbox>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub env: Option<Env>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub boot: Option<Boot>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bind: Option<Bind>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub desktop: Option<Desktop>,
}

impl Config {
  /// Whether the block states no key, which a written file leaves out.
  pub fn is_empty(&self) -> bool {
    self.root.is_none()
      && self.perms.is_none()
      && self.options.is_none()
      && self.unshare.is_none()
      && self.capsule.is_none()
      && self.overlay.is_none()
      && self.sandbox.is_none()
      && self.env.is_none()
      && self.boot.is_none()
      && self.bind.is_none()
      && self.desktop.is_none()
  }
}

impl Lower for Config {
  fn lower(&self) -> Result<Vec<Command>> {
    let mut commands = Vec::new();

    if let Some(perms) = &self.perms {
      commands.extend(perms.lower()?);
    }

    if let Some(capsule) = &self.capsule {
      commands.extend(capsule.lower()?);
    }

    if let Some(unshare) = &self.unshare {
      commands.extend(unshare.lower()?);
    }

    if let Some(options) = &self.options {
      commands.extend(options.lower()?);
    }

    if let Some(overlay) = &self.overlay {
      commands.extend(overlay.lower()?);
    }

    if let Some(sandbox) = &self.sandbox {
      commands.extend(sandbox.lower()?);
    }

    if let Some(env) = &self.env {
      commands.extend(env.lower()?);
    }

    if let Some(boot) = &self.boot {
      commands.extend(boot.lower()?);
    }

    if let Some(bind) = &self.bind {
      commands.extend(bind.lower()?);
    }

    if let Some(desktop) = &self.desktop {
      commands.extend(desktop.lower()?);
    }

    if let Some(root) = &self.root {
      commands.extend(root.lower()?);
    }

    Ok(commands)
  }
}

impl Violation for Config {
  fn violations(&self) -> Vec<Entry> {
    let mut entries = Vec::new();

    if let Some(root) = &self.root {
      entries.extend(root.violations());
    }

    if let Some(perms) = &self.perms {
      entries.extend(perms.violations());
    }

    if let Some(options) = &self.options {
      entries.extend(options.violations());
    }

    if let Some(unshare) = &self.unshare {
      entries.extend(unshare.violations());
    }

    if let Some(capsule) = &self.capsule {
      entries.extend(capsule.violations());
    }

    if let Some(overlay) = &self.overlay {
      entries.extend(overlay.violations());
    }

    if let Some(sandbox) = &self.sandbox {
      entries.extend(sandbox.violations());
    }

    if let Some(env) = &self.env {
      entries.extend(env.violations());
    }

    if let Some(boot) = &self.boot {
      entries.extend(boot.violations());
    }

    if let Some(bind) = &self.bind {
      entries.extend(bind.violations());
    }

    if let Some(desktop) = &self.desktop {
      entries.extend(desktop.violations());
    }

    entries
  }
}
