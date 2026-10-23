//! The FlatCompose file

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::Deserialize;

use crate::compose::config::Config;
use crate::compose::image::{From, Image};
use crate::internal::{hash, http};
use crate::lower::Lower;
use crate::violation::{Entry, Violation};

/// One FlatCompose file
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Compose {
  #[serde(skip)]
  pub path_file_compose: PathBuf,
  pub image: Image,
  #[serde(default)]
  pub config: Config,
}

impl Compose {
  /// Reads the file into the shape serde fills
  pub fn parse(path_file_compose: &Path) -> Result<Compose, serde_yaml::Error> {
    let text = std::fs::read_to_string(path_file_compose).map_err(|error| {
      <serde_yaml::Error as serde::de::Error>::custom(format!(
        "could not read `{}`: {error}",
        path_file_compose.display()
      ))
    })?;

    let mut compose: Compose = serde_yaml::from_str(&text)?;
    compose.path_file_compose = path_file_compose.to_path_buf();

    Ok(compose)
  }

  /// Builds the published FlatImage
  pub fn build(&self) -> Result<()> {
    let path_file_image = self.prepare(self.path_dir_compose())?;

    for command in self.config.lower() {
      self.run(&path_file_image, &command)?;
    }

    self.digest(&path_file_image)
  }

  /// The directory holding the FlatCompose file, which a relative `image.from` resolves against.
  fn path_dir_compose(&self) -> &Path {
    self.path_file_compose.parent().unwrap_or_else(|| Path::new("."))
  }

  /// Puts `image.output` in place: filled from `image.from`, or left as it stands under `inplace`.
  fn prepare(&self, path_dir_compose: &Path) -> Result<PathBuf> {
    let path_file_image = self.image.output.clone();

    if path_file_image.exists() && self.image.inplace {
      return Ok(path_file_image);
    }

    match &self.image.from {
      Some(From::Local(path_file_from)) => {
        let path_file_from = match path_file_from.is_absolute() {
          true => path_file_from.clone(),
          false => path_dir_compose.join(path_file_from),
        };

        std::fs::copy(&path_file_from, &path_file_image)
          .with_context(|| format!("copying `{}` to `{}`", path_file_from.display(), path_file_image.display()))?;
      }
      Some(From::Remote(url)) => http::fetch(url, &path_file_image)?,
      None => {}
    }

    #[cfg(unix)]
    {
      use std::os::unix::fs::PermissionsExt;

      std::fs::set_permissions(&path_file_image, std::fs::Permissions::from_mode(0o755))
        .with_context(|| format!("marking `{}` executable", path_file_image.display()))?;
    }

    Ok(path_file_image)
  }

  /// Runs one lowered command by executing the FlatImage with that command's argv.
  fn run(&self, path_file_image: &Path, command: &crate::lower::Command) -> Result<()> {
    let path_file_image =
      std::path::absolute(path_file_image).with_context(|| format!("resolving `{}`", path_file_image.display()))?;

    let status = std::process::Command::new(&path_file_image)
      .args(command.argv())
      .status()
      .with_context(|| format!("running `{}`", path_file_image.display()))?;

    if !status.success() {
      let name = command
        .argv()
        .first()
        .map(|word| word.to_string_lossy())
        .unwrap_or_default();

      bail!("`{name}` failed with {status}");
    }

    Ok(())
  }

  /// Writes the digest beside the published FlatImage, in the format `sha256sum -c` reads.
  fn digest(&self, path_file_image: &Path) -> Result<()> {
    let digest = hash::sha256(path_file_image)?;
    let path_file_digest = PathBuf::from(format!("{}.sha256sum", path_file_image.display()));
    let file_name = path_file_image
      .file_name()
      .map(|name| name.to_string_lossy().into_owned())
      .unwrap_or_default();

    std::fs::write(&path_file_digest, format!("{digest}  {file_name}\n"))
      .with_context(|| format!("writing `{}`", path_file_digest.display()))?;

    Ok(())
  }
}

impl Violation for Compose {
  /// Calls `violations()` on the yaml blocks and returns the violations
  fn violations(&self) -> Vec<Entry> {
    let mut entries = self.image.violations();
    entries.extend(self.config.violations());

    entries
  }
}
