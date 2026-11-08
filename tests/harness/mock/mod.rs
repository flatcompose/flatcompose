//! The mocking system for the tests

use std::fs;
use std::path::{Path, PathBuf};

pub mod desktop;

/// The directory holding the artifacts flatimage provides.
const DIR_ASSETS: &str = "tests/assets";

/// Copies the artifact at `tests/assets/<path_file_asset>` into
/// `<path_dir_output>/<path_file_asset>` and answers the path of the copy.
pub(crate) fn asset(path_file_asset: &str, path_dir_output: &Path) -> PathBuf {
  let path_file_from = Path::new(env!("CARGO_MANIFEST_DIR"))
    .join(DIR_ASSETS)
    .join(path_file_asset);
  let path_file_to = path_dir_output.join(path_file_asset);
  fs::copy(&path_file_from, &path_file_to)
    .unwrap_or_else(|error| panic!("copying {} to {}: {error}", path_file_from.display(), path_file_to.display()));
  path_file_to
}
