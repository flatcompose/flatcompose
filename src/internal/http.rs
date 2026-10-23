//! The fetch of a URL into a file.

use std::fs::File;
use std::io;
use std::path::Path;

use anyhow::{Context, Result};

/// Fetches the URL's body into the file, replacing whatever stood there.
pub fn fetch(url: &str, path_file: &Path) -> Result<()> {
  let response = ureq::get(url).call().with_context(|| format!("fetching `{url}`"))?;
  let mut reader = response.into_reader();
  let mut file = File::create(path_file).with_context(|| format!("creating `{}`", path_file.display()))?;

  io::copy(&mut reader, &mut file).with_context(|| format!("writing `{}`", path_file.display()))?;

  Ok(())
}
