//! The SHA-256 of a file.

use std::fs::File;
use std::io::Read;
use std::path::Path;

use anyhow::{Context, Result};
use sha2::{Digest, Sha256};

/// The file's SHA-256, as lowercase hex.
pub fn sha256(path_file: &Path) -> Result<String> {
  let mut file = File::open(path_file).with_context(|| format!("opening `{}`", path_file.display()))?;
  let mut hasher = Sha256::new();
  let mut buffer = [0u8; 64 * 1024];

  loop {
    let read = file
      .read(&mut buffer)
      .with_context(|| format!("reading `{}`", path_file.display()))?;

    if read == 0 {
      break;
    }

    hasher.update(&buffer[..read]);
  }

  Ok(format!("{:x}", hasher.finalize()))
}
