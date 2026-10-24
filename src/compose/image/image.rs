//! The `image` block

use std::path::PathBuf;

use serde::Deserialize;

use crate::violation::{Entry, Violation};

/// The `image` block, with the four keys that set the build.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Image {
  pub from: Option<From>,
  pub output: PathBuf,
  #[serde(default)]
  pub inplace: bool,
  #[serde(default)]
  pub arch: Vec<Arch>,
}

impl Violation for Image {
  /// Refuses a build with nothing to work on
  fn violations(&self) -> Vec<Entry> {
    if self.from.is_some() || self.output.exists() {
      return Vec::new();
    }

    vec![Entry::of(
      "image.from",
      format!("is absent and no file sits at `{}`", self.output.display()),
    )]
  }
}

/// The file the build starts from, as a path beside the FlatCompose file or an `http(s)://` URL.
#[derive(Deserialize)]
#[serde(try_from = "String")]
pub enum From {
  Local(PathBuf),
  Remote(String),
}

impl TryFrom<String> for From {
  type Error = std::convert::Infallible;

  fn try_from(token: String) -> Result<From, Self::Error> {
    let remote = token.starts_with("http://") || token.starts_with("https://");

    Ok(match remote {
      true => From::Remote(token),
      false => From::Local(PathBuf::from(token)),
    })
  }
}

/// One architecture the image's install steps build for.
#[derive(Deserialize)]
#[serde(try_from = "String")]
pub enum Arch {
  X86_64,
  I686,
  AArch64,
  Armv7l,
  Riscv64,
}

impl TryFrom<String> for Arch {
  type Error = String;

  fn try_from(token: String) -> Result<Arch, Self::Error> {
    match token.as_str() {
      "x86_64" => Ok(Arch::X86_64),
      "i686" => Ok(Arch::I686),
      "aarch64" => Ok(Arch::AArch64),
      "armv7l" => Ok(Arch::Armv7l),
      "riscv64" => Ok(Arch::Riscv64),
      _ => Err(format!("names `{token}`, which is not one of x86_64, i686, aarch64, armv7l, riscv64")),
    }
  }
}
