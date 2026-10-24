//! The files `config.desktop` names.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

use super::asset;

/// The artifact the manifest names as its icon.
const NAME_FILE_ICON: &str = "icon.png";

/// Copies `tests/assets/icon.png` into `<path_dir_output>` and answers the
/// path of the copy.
pub fn icon(path_dir_output: &Path) -> PathBuf {
  asset(NAME_FILE_ICON, path_dir_output)
}

/// Copies `tests/assets/entry.desktop` into `<path_dir_output>` and answers
/// the path of the copy.
pub fn entry(path_dir_output: &Path) -> PathBuf {
  asset("entry.desktop", path_dir_output)
}

/// Copies `tests/assets/mimetype.xml` into `<path_dir_output>` and answers
/// the path of the copy.
pub fn mimetype(path_dir_output: &Path) -> PathBuf {
  asset("mimetype.xml", path_dir_output)
}

/// Copies `tests/assets/manifest.json` into `<path_dir_output>` with its
/// `icon` naming the copied `icon.png`, then answers the path of that file.
pub fn manifest(path_dir_output: &Path) -> PathBuf {
  let path_file_icon = icon(path_dir_output);
  let path_file_manifest = asset("manifest.json", path_dir_output);
  let text = fs::read_to_string(&path_file_manifest).expect("the manifest artifact is readable");
  let mut manifest: Value = serde_json::from_str(&text).expect("the manifest artifact parses");
  manifest["icon"] = Value::String(path_file_icon.display().to_string());
  fs::write(&path_file_manifest, serde_json::to_string_pretty(&manifest).expect("the manifest serializes"))
    .expect("the manifest is writable");
  path_file_manifest
}
