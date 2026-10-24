//! `FR-DESKTOP`: the manifest, the integrations and the desktop files.

use std::fs;
use std::path::PathBuf;

use flatcompose::compose::config::Desktop;

use crate::common::{FlatImage, Harness, path_file_digest};
use crate::mock::desktop;

// covers: FR-DESKTOP-001
#[test]
fn a_manifest_reaches_the_output_file() {
  let mut harness = Harness::new("desktop-manifest");
  let path_file_manifest = desktop::manifest(harness.path_dir_output());
  harness.compose().config.desktop = Some(Desktop {
    manifest: Some(path_file_manifest.clone()),
    ..Desktop::default()
  });
  harness.write();

  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");

  let mut manifest = FlatImage::of(&harness.path_file_output())
    .desktop()
    .manifest()
    .expect("fim-desktop manifest show answers");
  let mut stated: serde_json::Value =
    serde_json::from_str(&fs::read_to_string(&path_file_manifest).expect("the manifest is readable"))
      .expect("the manifest parses");
  assert_eq!(manifest["name"], stated["name"]);
  manifest["categories"]
    .as_array_mut()
    .expect("the categories answer as a list")
    .sort_by(|left, right| left.as_str().cmp(&right.as_str()));
  stated["categories"]
    .as_array_mut()
    .expect("the categories answer as a list")
    .sort_by(|left, right| left.as_str().cmp(&right.as_str()));
  assert_eq!(manifest["categories"], stated["categories"]);
}

// covers: FR-DESKTOP-001
#[test]
fn an_icon_reaches_the_output_file() {
  let mut harness = Harness::new("desktop-icon");
  let path_file_icon = desktop::icon(harness.path_dir_output());
  let stored = fs::read(&path_file_icon).expect("the icon is readable");
  harness.compose().config.desktop = Some(Desktop {
    icon: Some(path_file_icon),
    ..Desktop::default()
  });
  harness.write();

  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");

  let icon = FlatImage::of(&harness.path_file_output())
    .desktop()
    .icon()
    .expect("fim-desktop icon show answers");
  assert_eq!(icon, stored);
}

// covers: FR-DESKTOP-001
#[test]
fn an_entry_reaches_the_output_file() {
  let mut harness = Harness::new("desktop-entry");
  let path_file_entry = desktop::entry(harness.path_dir_output());
  let stored = fs::read_to_string(&path_file_entry).expect("the entry is readable");
  harness.compose().config.desktop = Some(Desktop {
    entry: Some(path_file_entry),
    ..Desktop::default()
  });
  harness.write();

  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");

  let entry = FlatImage::of(&harness.path_file_output())
    .desktop()
    .entry()
    .expect("fim-desktop entry show answers");
  assert_eq!(entry, stored);
}

// covers: FR-DESKTOP-001
#[test]
fn a_mimetype_reaches_the_output_file() {
  let mut harness = Harness::new("desktop-mimetype");
  let path_file_mimetype = desktop::mimetype(harness.path_dir_output());
  let stored = fs::read_to_string(&path_file_mimetype).expect("the mimetype is readable");
  harness.compose().config.desktop = Some(Desktop {
    mimetype: Some(path_file_mimetype),
    ..Desktop::default()
  });
  harness.write();

  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");

  let mimetype = FlatImage::of(&harness.path_file_output())
    .desktop()
    .mimetype()
    .expect("fim-desktop mimetype show answers");
  assert_eq!(mimetype, stored);
}

// covers: FR-DESKTOP-001
#[test]
fn integrations_reach_the_output_file() {
  let mut harness = Harness::new("desktop-integrations");
  let path_dir_output = harness.path_dir_output().to_path_buf();
  harness.compose().config.desktop = Some(Desktop {
    manifest: Some(desktop::manifest(&path_dir_output)),
    entry: Some(desktop::entry(&path_dir_output)),
    icon: Some(desktop::icon(&path_dir_output)),
    integrations: vec!["entry".into(), "icon".into()],
    ..Desktop::default()
  });
  harness.write();

  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");

  let mut items = FlatImage::of(&harness.path_file_output())
    .desktop()
    .integrations()
    .expect("fim-desktop integrations show answers");
  items.sort();
  assert_eq!(items, ["entry", "icon"]);
}

// covers: FR-DESKTOP-001
#[test]
fn every_desktop_field_reaches_the_output_file() {
  let mut harness = Harness::new("desktop-every-field");
  let path_dir_output = harness.path_dir_output().to_path_buf();
  harness.compose().config.desktop = Some(Desktop {
    manifest: Some(desktop::manifest(&path_dir_output)),
    icon: Some(desktop::icon(&path_dir_output)),
    entry: Some(desktop::entry(&path_dir_output)),
    mimetype: Some(desktop::mimetype(&path_dir_output)),
    integrations: vec!["entry".into(), "icon".into()],
  });
  harness.write();

  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");
  assert!(outcome.silent(), "{outcome:?}");
}

// covers: FR-DESKTOP-001
#[test]
fn a_desktop_path_that_is_not_there_ends_the_build() {
  let mut harness = Harness::new("desktop-absent-path");
  harness.compose().config.desktop = Some(Desktop {
    icon: Some(PathBuf::from("/nonexistent/icon.png")),
    ..Desktop::default()
  });
  harness.write();

  let outcome = harness.run();

  assert!(!outcome.succeeded(), "{outcome:?}");
  assert!(!outcome.stderr.is_empty(), "{outcome:?}");
  assert!(!path_file_digest(&harness.path_file_output()).exists(), "{outcome:?}");
}
