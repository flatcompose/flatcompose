//! The `layer` command

use std::ffi::OsString;
use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::lower::{Command, Lower};

/// The `layer` key, which states one `fim-layer` command.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Layer {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub overlay: Option<Overlay>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub stack: Option<Stack>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mount: Option<Mount>,
}

impl Layer {
  /// The number of `fim-layer` commands the `layer` key states. A `layer` key must state exactly one.
  pub(crate) fn command_count(&self) -> usize {
    let overlay = self
      .overlay
      .as_ref()
      .map_or(0, |overlay| usize::from(overlay.save.is_some()));
    let stack = self.stack.as_ref().map_or(0, |stack| usize::from(stack.add.is_some()));
    let mount = self.mount.as_ref().map_or(0, |mount| {
      usize::from(mount.set.is_some()) + usize::from(mount.del.is_some()) + usize::from(mount.clear.is_some())
    });

    overlay + stack + mount
  }
}

impl Lower for Layer {
  /// Writes the command of the noun the key states.
  fn lower(&self) -> Result<Vec<Command>> {
    if let Some(overlay) = &self.overlay {
      return overlay.lower();
    }

    if let Some(stack) = &self.stack {
      return stack.lower();
    }

    if let Some(mount) = &self.mount {
      return mount.lower();
    }

    Ok(Vec::new())
  }
}

/// The `overlay` key, which states the `fim-layer overlay save` command.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Overlay {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub save: Option<OverlaySave>,
}

impl Lower for Overlay {
  /// Writes the command of the verb the key states.
  fn lower(&self) -> Result<Vec<Command>> {
    match &self.save {
      Some(save) => save.lower(),
      None => Ok(Vec::new()),
    }
  }
}

/// The `overlay.save` key, with the directory `fim-layer overlay save` compresses and the layer's destination.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OverlaySave {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub from: Option<PathBuf>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub to: Option<PathBuf>,
}

impl Lower for OverlaySave {
  /// Writes the source and destination the key states.
  fn lower(&self) -> Result<Vec<Command>> {
    let mut argv: Vec<OsString> = vec!["fim-layer".into(), "overlay".into(), "save".into()];

    if let Some(path_dir_from) = &self.from {
      argv.push("--from".into());
      argv.push(path_dir_from.clone().into_os_string());
    }

    if let Some(path_to) = &self.to {
      argv.push("--to".into());
      argv.push(path_to.clone().into_os_string());
    }

    Ok(vec![Command::new(argv)])
  }
}

/// The `stack` key, which states the `fim-layer stack add` command.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stack {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub add: Option<StackAdd>,
}

impl Lower for Stack {
  /// Writes the command of the verb the key states.
  fn lower(&self) -> Result<Vec<Command>> {
    match &self.add {
      Some(add) => add.lower(),
      None => Ok(Vec::new()),
    }
  }
}

/// The `stack.add` key, with the layer file `fim-layer stack add` appends and the path where the command grafts the layer.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StackAdd {
  pub path: PathBuf,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mount: Option<PathBuf>,
}

impl Lower for StackAdd {
  /// Writes the mountpoint the key states before the layer file it appends.
  fn lower(&self) -> Result<Vec<Command>> {
    let mut argv: Vec<OsString> = vec!["fim-layer".into(), "stack".into(), "add".into()];

    if let Some(path_mount) = &self.mount {
      argv.push("--mount".into());
      argv.push(path_mount.clone().into_os_string());
    }

    argv.push(self.path.clone().into_os_string());

    Ok(vec![Command::new(argv)])
  }
}

/// The `mount` key, which manages the mountpoints recorded in the FlatImage.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Mount {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub set: Option<MountSet>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub del: Option<MountDel>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub clear: Option<MountClear>,
}

impl Lower for Mount {
  /// Writes the command of the verb the key states.
  fn lower(&self) -> Result<Vec<Command>> {
    if let Some(set) = &self.set {
      return set.lower();
    }

    if let Some(del) = &self.del {
      return del.lower();
    }

    if let Some(clear) = &self.clear {
      return clear.lower();
    }

    Ok(Vec::new())
  }
}

/// The `mount.set` key, with the layer hash and the path where `fim-layer mount set` grafts the layer.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MountSet {
  pub hash: String,
  pub path: PathBuf,
}

impl Lower for MountSet {
  /// Writes the hash and the path the key states.
  fn lower(&self) -> Result<Vec<Command>> {
    let argv: Vec<OsString> = vec![
      "fim-layer".into(),
      "mount".into(),
      "set".into(),
      self.hash.clone().into(),
      self.path.clone().into_os_string(),
    ];

    Ok(vec![Command::new(argv)])
  }
}

/// The `mount.del` key, with the hash of the layer `fim-layer mount del` unmounts.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MountDel {
  pub hash: String,
}

impl Lower for MountDel {
  /// Writes the hash the key states.
  fn lower(&self) -> Result<Vec<Command>> {
    let argv: Vec<OsString> = vec![
      "fim-layer".into(),
      "mount".into(),
      "del".into(),
      self.hash.clone().into(),
    ];

    Ok(vec![Command::new(argv)])
  }
}

/// The `mount.clear` key, which removes every mountpoint.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MountClear {}

impl Lower for MountClear {
  /// Writes the command that removes every mountpoint.
  fn lower(&self) -> Result<Vec<Command>> {
    let argv: Vec<OsString> = vec!["fim-layer".into(), "mount".into(), "clear".into()];

    Ok(vec![Command::new(argv)])
  }
}

#[cfg(test)]
mod tests {
  use std::path::PathBuf;

  use crate::lower::{Command, Lower};

  use super::{Layer, Mount, MountClear, MountDel, MountSet, Overlay, OverlaySave, Stack, StackAdd};

  fn argv(command: &Command) -> Vec<String> {
    command
      .argv()
      .iter()
      .map(|arg| arg.to_string_lossy().into_owned())
      .collect()
  }

  fn commands(layer: &Layer) -> Vec<Vec<String>> {
    layer.lower().expect("the command lowers").iter().map(argv).collect()
  }

  fn layer(overlay: Option<Overlay>, stack: Option<Stack>, mount: Option<Mount>) -> Layer {
    Layer { overlay, stack, mount }
  }

  fn overlay(from: Option<&str>, to: Option<&str>) -> Layer {
    layer(
      Some(Overlay {
        save: Some(OverlaySave {
          from: from.map(PathBuf::from),
          to: to.map(PathBuf::from),
        }),
      }),
      None,
      None,
    )
  }

  fn stack(path: &str, mount: Option<&str>) -> Layer {
    layer(
      None,
      Some(Stack {
        add: Some(StackAdd {
          path: PathBuf::from(path),
          mount: mount.map(PathBuf::from),
        }),
      }),
      None,
    )
  }

  fn mount_set(hash: &str, path: &str) -> Layer {
    layer(
      None,
      None,
      Some(Mount {
        set: Some(MountSet {
          hash: hash.into(),
          path: PathBuf::from(path),
        }),
        del: None,
        clear: None,
      }),
    )
  }

  fn mount_del(hash: &str) -> Layer {
    layer(
      None,
      None,
      Some(Mount {
        set: None,
        del: Some(MountDel { hash: hash.into() }),
        clear: None,
      }),
    )
  }

  fn mount_clear() -> Layer {
    layer(
      None,
      None,
      Some(Mount {
        set: None,
        del: None,
        clear: Some(MountClear {}),
      }),
    )
  }

  // covers: FR-LAYER-001
  #[test]
  fn lower_writes_the_overlay_save_command() {
    assert_eq!(commands(&overlay(None, None)), vec![vec!["fim-layer", "overlay", "save"]]);
  }

  // covers: FR-LAYER-002, FR-LAYER-003
  #[test]
  fn lower_writes_the_overlay_source_and_destination() {
    assert_eq!(
      commands(&overlay(Some("./game-assets"), Some("./game.layer"))),
      vec![vec![
        "fim-layer",
        "overlay",
        "save",
        "--from",
        "./game-assets",
        "--to",
        "./game.layer"
      ]]
    );
  }

  // covers: FR-LAYER-002, FR-LAYER-003
  #[test]
  fn lower_omits_the_overlay_flags_the_key_does_not_name() {
    assert_eq!(
      commands(&overlay(Some("./game-assets"), None)),
      vec![vec!["fim-layer", "overlay", "save", "--from", "./game-assets"]]
    );

    assert_eq!(
      commands(&overlay(None, Some("./game.layer"))),
      vec![vec!["fim-layer", "overlay", "save", "--to", "./game.layer"]]
    );
  }

  // covers: FR-LAYER-004, FR-LAYER-005
  #[test]
  fn lower_writes_the_stack_add_command() {
    assert_eq!(commands(&stack("./game.layer", None)), vec![vec!["fim-layer", "stack", "add", "./game.layer"]]);
  }

  // covers: FR-LAYER-006
  #[test]
  fn lower_writes_the_mount_before_the_layer_file() {
    assert_eq!(
      commands(&stack("./game.layer", Some("/opt/game"))),
      vec![vec![
        "fim-layer",
        "stack",
        "add",
        "--mount",
        "/opt/game",
        "./game.layer"
      ]]
    );
  }

  // covers: FR-LAYER-007, FR-LAYER-008, FR-LAYER-009
  #[test]
  fn lower_writes_the_mount_set_command() {
    assert_eq!(commands(&mount_set("9f2b", "/opt/game")), vec![vec!["fim-layer", "mount", "set", "9f2b", "/opt/game"]]);
  }

  // covers: FR-LAYER-010, FR-LAYER-011
  #[test]
  fn lower_writes_the_mount_del_command() {
    assert_eq!(commands(&mount_del("9f2b")), vec![vec!["fim-layer", "mount", "del", "9f2b"]]);
  }

  // covers: FR-LAYER-012
  #[test]
  fn lower_writes_the_mount_clear_command() {
    assert_eq!(commands(&mount_clear()), vec![vec!["fim-layer", "mount", "clear"]]);
  }

  // covers: FR-LAYER-013
  #[test]
  fn command_count_counts_every_layer_command() {
    assert_eq!(layer(None, None, None).command_count(), 0);
    assert_eq!(overlay(None, None).command_count(), 1);
    assert_eq!(stack("./game.layer", None).command_count(), 1);
    assert_eq!(mount_set("9f2b", "/opt/game").command_count(), 1);
    assert_eq!(mount_del("9f2b").command_count(), 1);
    assert_eq!(mount_clear().command_count(), 1);

    let two_nouns = layer(
      Some(Overlay {
        save: Some(OverlaySave { from: None, to: None }),
      }),
      Some(Stack {
        add: Some(StackAdd {
          path: PathBuf::from("./game.layer"),
          mount: None,
        }),
      }),
      None,
    );
    assert_eq!(two_nouns.command_count(), 2);

    let two_verbs = layer(
      None,
      None,
      Some(Mount {
        set: Some(MountSet {
          hash: "9f2b".into(),
          path: PathBuf::from("/opt/game"),
        }),
        del: Some(MountDel { hash: "9f2b".into() }),
        clear: None,
      }),
    );
    assert_eq!(two_verbs.command_count(), 2);
  }
}
