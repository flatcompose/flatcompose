//! The `exec` command

use std::ffi::OsString;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::lower::{Command, Lower};

/// The `exec` key, which states the `fim-exec` command that runs a shell script in a root filesystem.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Exec {
  pub from: String,
  #[serde(
    default,
    deserialize_with = "crate::internal::scalar::optional_u32",
    skip_serializing_if = "Option::is_none"
  )]
  pub uid: Option<u32>,
  #[serde(
    default,
    deserialize_with = "crate::internal::scalar::optional_u32",
    skip_serializing_if = "Option::is_none"
  )]
  pub gid: Option<u32>,
  pub script: String,
  /// The entry's position among the steps. It sets the script file name.
  #[serde(skip)]
  pub position: usize,
}

impl Lower for Exec {
  /// Writes the script where the step position names it, then states the commands that run it.
  ///
  /// The builder derives the root name from the source by replacing `:` and `@` with `-`, as
  /// flatimage's `name_derive` does.
  fn lower(&self) -> Result<Vec<Command>> {
    let path_file_script = path_file_script(self.position);
    let path_dir_script = path_file_script
      .parent()
      .context("the script path names its directory")?;

    std::fs::create_dir_all(path_dir_script).with_context(|| format!("creating `{}`", path_dir_script.display()))?;

    for path_dir in [
      Path::new("/tmp/fim"),
      Path::new("/tmp/fim/flatcompose"),
      path_dir_script,
    ] {
      std::fs::set_permissions(path_dir, std::fs::Permissions::from_mode(0o755))
        .with_context(|| format!("marking `{}` readable", path_dir.display()))?;
    }

    std::fs::write(&path_file_script, &self.script)
      .with_context(|| format!("writing `{}`", path_file_script.display()))?;

    std::fs::set_permissions(&path_file_script, std::fs::Permissions::from_mode(0o644))
      .with_context(|| format!("marking `{}` readable", path_file_script.display()))?;

    let name_root = self.from.replace([':', '@'], "-");
    let mut argv: Vec<OsString> = vec!["fim-exec".into()];

    if let Some(uid) = self.uid {
      argv.push("--uid".into());
      argv.push(uid.to_string().into());
    }

    if let Some(gid) = self.gid {
      argv.push("--gid".into());
      argv.push(gid.to_string().into());
    }

    argv.push("bash".into());
    argv.push("-e".into());
    argv.push("-o".into());
    argv.push("pipefail".into());
    argv.push(path_file_script.into_os_string());

    Ok(vec![
      Command::new(vec!["fim-root".into(), "system".into(), "use".into(), name_root.into()]),
      Command::new(argv),
    ])
  }
}

/// The path the script of the step at `position` takes: `/tmp/fim/flatcompose/<build-pid>/script-<n>.sh`.
fn path_file_script(position: usize) -> PathBuf {
  PathBuf::from(format!("/tmp/fim/flatcompose/{}/script-{}.sh", std::process::id(), position))
}

#[cfg(test)]
mod tests {
  use crate::lower::{Command, Lower};

  use super::{Exec, path_file_script};

  fn exec() -> Exec {
    Exec {
      from: "debian:bookworm".into(),
      uid: None,
      gid: None,
      script: "firefox --version\n".into(),
      position: 0,
    }
  }

  fn argv(command: &Command) -> Vec<String> {
    command
      .argv()
      .iter()
      .map(|arg| arg.to_string_lossy().into_owned())
      .collect()
  }

  // covers: FR-EXEC-003
  #[test]
  fn lower_selects_the_root_before_running_the_script() {
    let commands = exec().lower().expect("the commands lower");

    assert_eq!(argv(&commands[0]), vec!["fim-root", "system", "use", "debian-bookworm"]);
    assert_eq!(argv(&commands[1])[0], "fim-exec");
  }

  // covers: FR-EXEC-003
  #[test]
  fn lower_names_the_root_the_source_derives() {
    let mut exec = exec();
    exec.from = "ubuntu:noble@2024-06-15".into();

    let commands = exec.lower().expect("the commands lower");

    assert_eq!(argv(&commands[0]), vec!["fim-root", "system", "use", "ubuntu-noble-2024-06-15"]);
  }

  // covers: FR-EXEC-004
  #[test]
  fn lower_runs_bash_with_the_script_path() {
    let commands = exec().lower().expect("the commands lower");
    let path_file_script = path_file_script(0).to_string_lossy().into_owned();

    assert_eq!(argv(&commands[1]), vec!["fim-exec", "bash", "-e", "-o", "pipefail", &path_file_script]);
  }

  // covers: FR-EXEC-004
  #[test]
  fn lower_names_the_script_by_the_step_position() {
    let first = path_file_script(0);
    let second = path_file_script(1);
    let pid = std::process::id().to_string();

    assert_eq!(first.parent().unwrap().file_name().unwrap().to_string_lossy(), pid);
    assert_eq!(first.file_name().unwrap().to_string_lossy(), "script-0.sh");
    assert_eq!(second.file_name().unwrap().to_string_lossy(), "script-1.sh");
  }

  // covers: FR-EXEC-005, FR-EXEC-006
  #[test]
  fn lower_omits_the_flags_the_file_does_not_name() {
    let commands = exec().lower().expect("the commands lower");
    let path_file_script = path_file_script(0).to_string_lossy().into_owned();

    assert_eq!(argv(&commands[1]), vec!["fim-exec", "bash", "-e", "-o", "pipefail", &path_file_script]);
  }

  // covers: FR-EXEC-006
  #[test]
  fn lower_writes_the_uid_before_the_gid() {
    let mut exec = exec();
    exec.uid = Some(1000);
    exec.gid = Some(4242);

    let commands = exec.lower().expect("the commands lower");
    let path_file_script = path_file_script(0).to_string_lossy().into_owned();

    assert_eq!(
      argv(&commands[1]),
      vec![
        "fim-exec",
        "--uid",
        "1000",
        "--gid",
        "4242",
        "bash",
        "-e",
        "-o",
        "pipefail",
        &path_file_script
      ]
    );
  }

  // covers: FR-EXEC-005, FR-EXEC-006
  #[test]
  fn lower_keeps_an_explicit_zero_uid_and_gid() {
    let mut exec = exec();
    exec.uid = Some(0);
    exec.gid = Some(0);

    let commands = exec.lower().expect("the commands lower");
    let path_file_script = path_file_script(0).to_string_lossy().into_owned();

    assert_eq!(
      argv(&commands[1]),
      vec![
        "fim-exec",
        "--uid",
        "0",
        "--gid",
        "0",
        "bash",
        "-e",
        "-o",
        "pipefail",
        &path_file_script
      ]
    );
  }
}
