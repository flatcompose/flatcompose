//! The `config` block

use serde::Deserialize;

use crate::compose::config::{Bind, Boot, Capsule, Desktop, Env, Options, Overlay, Perms, Root, Sandbox, Unshare};
use crate::lower::{Command, Lower};
use crate::violation::{Entry, Violation};

/// The `config` block, carrying the entities of the keys the file writes.
#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
  pub root: Option<Root>,
  pub perms: Option<Perms>,
  pub options: Option<Options>,
  pub unshare: Option<Unshare>,
  pub capsule: Option<Capsule>,
  pub overlay: Option<Overlay>,
  pub sandbox: Option<Sandbox>,
  pub env: Option<Env>,
  pub boot: Option<Boot>,
  pub bind: Option<Bind>,
  pub desktop: Option<Desktop>,
}

impl Lower for Config {
  fn lower(&self) -> Vec<Command> {
    let mut commands = Vec::new();

    if let Some(perms) = &self.perms {
      commands.extend(perms.lower());
    }

    if let Some(capsule) = &self.capsule {
      commands.extend(capsule.lower());
    }

    if let Some(unshare) = &self.unshare {
      commands.extend(unshare.lower());
    }

    if let Some(options) = &self.options {
      commands.extend(options.lower());
    }

    if let Some(overlay) = &self.overlay {
      commands.extend(overlay.lower());
    }

    if let Some(sandbox) = &self.sandbox {
      commands.extend(sandbox.lower());
    }

    if let Some(env) = &self.env {
      commands.extend(env.lower());
    }

    if let Some(boot) = &self.boot {
      commands.extend(boot.lower());
    }

    if let Some(bind) = &self.bind {
      commands.extend(bind.lower());
    }

    if let Some(desktop) = &self.desktop {
      commands.extend(desktop.lower());
    }

    if let Some(root) = &self.root {
      commands.extend(root.lower());
    }

    commands
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
