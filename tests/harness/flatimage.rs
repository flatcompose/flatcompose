//! flatimage, driven as a subprocess.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use anyhow::{Context, Result, ensure};

/// A collection a container holds.
pub trait Collection {
  /// One item of the collection.
  type Item;

  /// The items the container holds, in the order flatimage listed them.
  fn items(self) -> Result<Vec<Self::Item>>;
}

/// flatimage, the image file at `path_file`.
pub struct FlatImage {
  path_file: PathBuf,
}

impl FlatImage {
  /// flatimage on the image at `path_file`.
  pub fn of(path_file: &Path) -> FlatImage {
    FlatImage {
      path_file: path_file.to_path_buf(),
    }
  }

  /// The image file this flatimage runs from.
  pub fn path_file_self(&self) -> &Path {
    &self.path_file
  }

  /// The `fim-root` command.
  pub fn root(&self) -> root::Root {
    root::Root::of(&self.path_file)
  }

  /// The `fim-perms` command.
  pub fn perms(&self) -> perms::Perms {
    perms::Perms::of(&self.path_file)
  }

  /// The `fim-options` command.
  pub fn options(&self) -> options::Options {
    options::Options::of(&self.path_file)
  }

  /// The `fim-unshare` command.
  pub fn unshare(&self) -> unshare::Unshare {
    unshare::Unshare::of(&self.path_file)
  }

  /// The `fim-capsule` command.
  pub fn capsule(&self) -> capsule::Capsule {
    capsule::Capsule::of(&self.path_file)
  }

  /// The `fim-env` command.
  pub fn env(&self) -> env::Env {
    env::Env::of(&self.path_file)
  }

  /// The `fim-bind` command.
  pub fn bind(&self) -> bind::Bind {
    bind::Bind::of(&self.path_file)
  }

  /// The `fim-overlay` command.
  pub fn overlay(&self) -> overlay::Overlay {
    overlay::Overlay::of(&self.path_file)
  }

  /// The `fim-sandbox` command.
  pub fn sandbox(&self) -> sandbox::Sandbox {
    sandbox::Sandbox::of(&self.path_file)
  }

  /// The `fim-boot` command.
  pub fn boot(&self) -> boot::Boot {
    boot::Boot::of(&self.path_file)
  }

  /// The `fim-desktop` command.
  pub fn desktop(&self) -> desktop::Desktop {
    desktop::Desktop::of(&self.path_file)
  }
}

/// Runs `command`, refusing when it cannot start.
pub(super) fn execute(command: &mut Command) -> Result<Output> {
  command
    .output()
    .with_context(|| format!("running {}", command.get_program().to_string_lossy()))
}

/// The value `command` prints as JSON, which refuses when flatimage answers non-zero.
pub(super) fn read(command: &mut Command) -> Result<serde_json::Value> {
  let output = execute(command.arg("--format").arg("json"))?;
  ensure!(output.status.success(), "flatimage refused: {}", String::from_utf8_lossy(&output.stderr).trim());
  let printed = String::from_utf8(output.stdout).context("reading what flatimage printed")?;
  serde_json::from_str(&printed).context("parsing what flatimage printed")
}

/// The bytes `command` prints, which refuses when flatimage answers non-zero.
pub(super) fn read_bytes(command: &mut Command) -> Result<Vec<u8>> {
  let output = execute(command)?;
  ensure!(output.status.success(), "flatimage refused: {}", String::from_utf8_lossy(&output.stderr).trim());
  Ok(output.stdout)
}

/// The `fim-root` command: the roots the container carries.
pub mod root {
  use std::path::Path;
  use std::process::Command;

  use anyhow::Result;

  use super::Collection;

  /// The `fim-root` command.
  pub struct Root {
    command: Command,
  }

  impl Root {
    /// The command, opening the invocation on flatimage at `path_file`.
    pub(super) fn of(path_file: &Path) -> Root {
      let mut command = Command::new(path_file);
      command.arg("fim-root");
      Root { command }
    }

    /// The `fim-root system` command: the roots the container carries.
    pub fn system(self) -> system::System {
      system::System::of(self.command)
    }
  }

  /// The `fim-root system` command, and the roots it lists.
  pub mod system {
    use std::process::Command;

    use anyhow::{Context, Result, ensure};
    use serde::Deserialize;

    use super::super::{Collection, execute};

    /// One root the container carries: the name it is addressed by, and whether it is active.
    #[derive(Deserialize)]
    pub struct Root {
      pub name: String,
      pub active: bool,
    }

    /// The `fim-root system` command.
    pub struct System {
      command: Command,
    }

    impl System {
      /// Composes `fim-root system`.
      pub(super) fn of(mut command: Command) -> System {
        command.arg("system");
        System { command }
      }
    }

    impl Collection for System {
      type Item = Root;

      /// The roots the container carries, in the order flatimage lists them.
      fn items(mut self) -> Result<Vec<Root>> {
        let output = execute(self.command.arg("list").arg("--format").arg("json"))?;
        ensure!(output.status.success(), "flatimage refused: {}", String::from_utf8_lossy(&output.stderr).trim());
        let printed = String::from_utf8(output.stdout).context("reading what list printed")?;
        serde_json::from_str(&printed).context("parsing what list printed")
      }
    }
  }
}

/// The `fim-perms` command.
pub mod perms {
  use std::path::Path;
  use std::process::Command;

  use anyhow::Result;

  use super::read;

  /// The `fim-perms` command.
  pub struct Perms {
    command: Command,
  }

  impl Perms {
    /// The command, opening the invocation on flatimage at `path_file`.
    pub(super) fn of(path_file: &Path) -> Perms {
      let mut command = Command::new(path_file);
      command.arg("fim-perms");
      Perms { command }
    }

    /// `fim-perms list`: the host resources the sandbox grants.
    pub fn list(mut self) -> Result<Vec<String>> {
      let value = read(self.command.arg("list"))?;
      Ok(serde_json::from_value(value)?)
    }
  }
}

/// The `fim-options` command.
pub mod options {
  use std::path::Path;
  use std::process::Command;

  use anyhow::Result;

  use super::read;

  /// The `fim-options` command.
  pub struct Options {
    command: Command,
  }

  impl Options {
    /// The command, opening the invocation on flatimage at `path_file`.
    pub(super) fn of(path_file: &Path) -> Options {
      let mut command = Command::new(path_file);
      command.arg("fim-options");
      Options { command }
    }

    /// `fim-options list`: the flatimage toggles the image enables.
    pub fn list(mut self) -> Result<Vec<String>> {
      let value = read(self.command.arg("list"))?;
      Ok(serde_json::from_value(value)?)
    }
  }
}

/// The `fim-unshare` command.
pub mod unshare {
  use std::path::Path;
  use std::process::Command;

  use anyhow::Result;

  use super::read;

  /// The `fim-unshare` command.
  pub struct Unshare {
    command: Command,
  }

  impl Unshare {
    /// The command, opening the invocation on flatimage at `path_file`.
    pub(super) fn of(path_file: &Path) -> Unshare {
      let mut command = Command::new(path_file);
      command.arg("fim-unshare");
      Unshare { command }
    }

    /// `fim-unshare list`: the namespaces the sandbox detaches.
    pub fn list(mut self) -> Result<Vec<String>> {
      let value = read(self.command.arg("list"))?;
      Ok(serde_json::from_value(value)?)
    }
  }
}

/// The `fim-capsule` command.
pub mod capsule {
  use std::path::Path;
  use std::process::Command;

  use anyhow::Result;

  use super::read;

  /// The `fim-capsule` command.
  pub struct Capsule {
    command: Command,
  }

  impl Capsule {
    /// The command, opening the invocation on flatimage at `path_file`.
    pub(super) fn of(path_file: &Path) -> Capsule {
      let mut command = Command::new(path_file);
      command.arg("fim-capsule");
      Capsule { command }
    }

    /// `fim-capsule list`: the host driver stacks the container links into itself.
    pub fn list(mut self) -> Result<Vec<String>> {
      let value = read(self.command.arg("list"))?;
      Ok(serde_json::from_value(value)?)
    }
  }
}

/// The `fim-env` command.
pub mod env {
  use std::path::Path;
  use std::process::Command;

  use anyhow::Result;
  use serde::Deserialize;

  use super::read;

  /// One variable the image stores.
  #[derive(Deserialize)]
  pub struct Variable {
    pub name: String,
    pub value: String,
  }

  /// The `fim-env` command.
  pub struct Env {
    command: Command,
  }

  impl Env {
    /// The command, opening the invocation on flatimage at `path_file`.
    pub(super) fn of(path_file: &Path) -> Env {
      let mut command = Command::new(path_file);
      command.arg("fim-env");
      Env { command }
    }

    /// `fim-env list`: every variable the image stores.
    pub fn list(mut self) -> Result<Vec<Variable>> {
      let value = read(self.command.arg("list"))?;
      Ok(serde_json::from_value(value)?)
    }
  }
}

/// The `fim-bind` command.
pub mod bind {
  use std::path::Path;
  use std::process::Command;

  use anyhow::Result;
  use serde::Deserialize;

  use super::read;

  /// One host path the image binds.
  #[derive(Deserialize)]
  pub struct Binding {
    pub src: String,
    pub dst: String,
    #[serde(rename = "type")]
    pub mode: String,
  }

  /// The `fim-bind` command.
  pub struct Bind {
    command: Command,
  }

  impl Bind {
    /// The command, opening the invocation on flatimage at `path_file`.
    pub(super) fn of(path_file: &Path) -> Bind {
      let mut command = Command::new(path_file);
      command.arg("fim-bind");
      Bind { command }
    }

    /// `fim-bind list`: every binding the image carries.
    pub fn list(mut self) -> Result<Vec<Binding>> {
      let value = read(self.command.arg("list"))?;
      Ok(serde_json::from_value(value)?)
    }
  }
}

/// The `fim-overlay` command.
pub mod overlay {
  use std::path::Path;
  use std::process::Command;

  use anyhow::Result;

  use super::read;

  /// The `fim-overlay` command.
  pub struct Overlay {
    command: Command,
  }

  impl Overlay {
    /// The command, opening the invocation on flatimage at `path_file`.
    pub(super) fn of(path_file: &Path) -> Overlay {
      let mut command = Command::new(path_file);
      command.arg("fim-overlay");
      Overlay { command }
    }

    /// `fim-overlay show`: the backend the image stores.
    pub fn show(mut self) -> Result<String> {
      let value = read(self.command.arg("show"))?;
      Ok(serde_json::from_value(value)?)
    }
  }
}

/// The `fim-sandbox` command.
pub mod sandbox {
  use std::path::Path;
  use std::process::Command;

  use anyhow::Result;

  use super::read;

  /// The `fim-sandbox` command.
  pub struct Sandbox {
    command: Command,
  }

  impl Sandbox {
    /// The command, opening the invocation on flatimage at `path_file`.
    pub(super) fn of(path_file: &Path) -> Sandbox {
      let mut command = Command::new(path_file);
      command.arg("fim-sandbox");
      Sandbox { command }
    }

    /// `fim-sandbox show`: the mode the image executes in.
    pub fn show(mut self) -> Result<String> {
      let value = read(self.command.arg("show"))?;
      Ok(serde_json::from_value(value)?)
    }
  }
}

/// The `fim-boot` command.
pub mod boot {
  use std::path::Path;
  use std::process::Command;

  use anyhow::Result;
  use serde::Deserialize;

  use super::read;

  /// The command the image starts: its program and its arguments.
  #[derive(Deserialize)]
  pub struct BootItem {
    pub program: String,
    #[serde(default)]
    pub args: Vec<String>,
  }

  /// The `fim-boot` command.
  pub struct Boot {
    command: Command,
  }

  impl Boot {
    /// The command, opening the invocation on flatimage at `path_file`.
    pub(super) fn of(path_file: &Path) -> Boot {
      let mut command = Command::new(path_file);
      command.arg("fim-boot");
      Boot { command }
    }

    /// `fim-boot show`: the program the image starts and its arguments.
    pub fn show(mut self) -> Result<BootItem> {
      let value = read(self.command.arg("show"))?;
      Ok(serde_json::from_value(value)?)
    }
  }
}

/// The `fim-desktop` command: the manifest, the integrations and the files.
pub mod desktop {
  use std::path::Path;
  use std::process::Command;

  use anyhow::Result;

  use super::{read, read_bytes};

  /// The `fim-desktop` command.
  pub struct Desktop {
    command: Command,
  }

  impl Desktop {
    /// The command, opening the invocation on flatimage at `path_file`.
    pub(super) fn of(path_file: &Path) -> Desktop {
      let mut command = Command::new(path_file);
      command.arg("fim-desktop");
      Desktop { command }
    }

    /// `fim-desktop manifest show`: the name, the categories and the integrations the image stores.
    pub fn manifest(mut self) -> Result<serde_json::Value> {
      read(self.command.arg("manifest").arg("show"))
    }

    /// `fim-desktop integrations show`: the integration items the image reports.
    pub fn integrations(mut self) -> Result<Vec<String>> {
      let value = read(self.command.arg("integrations").arg("show"))?;
      Ok(serde_json::from_value(value)?)
    }

    /// `fim-desktop entry show`: the `.desktop` text the image stores.
    pub fn entry(mut self) -> Result<String> {
      let bytes = read_bytes(self.command.arg("entry").arg("show"))?;
      Ok(String::from_utf8(bytes)?)
    }

    /// `fim-desktop mimetype show`: the XML text the image stores.
    pub fn mimetype(mut self) -> Result<String> {
      let bytes = read_bytes(self.command.arg("mimetype").arg("show"))?;
      Ok(String::from_utf8(bytes)?)
    }

    /// `fim-desktop icon show`: the image bytes the icon slot holds.
    pub fn icon(mut self) -> Result<Vec<u8>> {
      read_bytes(self.command.arg("icon").arg("show"))
    }
  }
}
