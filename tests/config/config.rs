//! `FR-CONFIG`: the config block itself

use std::fs;
use std::path::PathBuf;

use flatcompose::compose::config::{
  Bind, Binding, Boot, Capsule, Desktop, Env, Mode, Options, Overlay, Perms, Sandbox, Unshare, Variable,
};
use indoc::indoc;

use crate::common::{FlatImage, Harness, path_file_digest};
use crate::mock::desktop;

// covers: FR-CONFIG-002
#[test]
fn every_key_of_the_config_block_reaches_the_output_file() {
  let mut harness = Harness::new("config-every-key");
  let path_dir_output = harness.path_dir_output().to_path_buf();

  let config = &mut harness.compose().config;
  config.perms = Some(Perms {
    entries: vec!["network".into(), "home".into()],
  });
  config.options = Some(Options {
    entries: vec!["casefold".into()],
  });
  config.unshare = Some(Unshare {
    entries: vec!["pid".into()],
  });
  config.capsule = Some(Capsule {
    entries: vec!["gpu".into()],
  });
  config.overlay = Some(Overlay {
    backend: "overlayfs".into(),
  });
  config.sandbox = Some(Sandbox { mode: "bwrap".into() });
  config.env = Some(Env {
    entries: vec![Variable {
      name: "LANG".into(),
      value: "C.UTF-8".into(),
    }],
  });
  config.boot = Some(Boot {
    program: PathBuf::from("/usr/bin/true"),
    args: vec![],
  });
  config.bind = Some(Bind {
    entries: vec![Binding {
      src: PathBuf::from("/tmp/in"),
      dst: PathBuf::from("/in"),
      mode: Mode::Ro,
    }],
  });
  config.desktop = Some(Desktop {
    manifest: Some(desktop::manifest(&path_dir_output)),
    icon: Some(desktop::icon(&path_dir_output)),
    entry: Some(desktop::entry(&path_dir_output)),
    mimetype: Some(desktop::mimetype(&path_dir_output)),
    integrations: vec!["entry".into(), "icon".into()],
  });

  harness.write();
  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");

  let flatimage = FlatImage::of(&harness.path_file_output());
  assert_eq!(flatimage.perms().list().expect("fim-perms list answers").len(), 2);
  assert_eq!(flatimage.options().list().expect("fim-options list answers"), ["casefold"]);
  assert_eq!(flatimage.unshare().list().expect("fim-unshare list answers"), ["pid"]);
  assert_eq!(flatimage.capsule().list().expect("fim-capsule list answers"), ["gpu"]);
  assert_eq!(flatimage.overlay().show().expect("fim-overlay show answers"), "overlayfs");
  assert_eq!(flatimage.sandbox().show().expect("fim-sandbox show answers"), "bwrap");
  assert_eq!(flatimage.env().list().expect("fim-env list answers").len(), 1);
  assert_eq!(flatimage.boot().show().expect("fim-boot show answers").program, "/usr/bin/true");
  assert_eq!(flatimage.bind().list().expect("fim-bind list answers").len(), 1);
  assert!(
    flatimage
      .desktop()
      .manifest()
      .expect("fim-desktop manifest show answers")["name"]
      .is_string()
  );
  assert!(harness.verifies());
}

// covers: FR-CONFIG-001
#[test]
fn a_file_stating_no_config_block_builds() {
  let harness = Harness::new("config-absent");

  harness.write();
  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");
  assert!(outcome.silent(), "{outcome:?}");

  let flatimage = FlatImage::of(&harness.path_file_output());
  assert_eq!(flatimage.perms().list().expect("fim-perms list answers"), Vec::<String>::new());
  assert_eq!(flatimage.options().list().expect("fim-options list answers"), Vec::<String>::new());
  assert_eq!(flatimage.unshare().list().expect("fim-unshare list answers"), Vec::<String>::new());
  assert_eq!(flatimage.capsule().list().expect("fim-capsule list answers"), Vec::<String>::new());
  assert_eq!(flatimage.env().list().expect("fim-env list answers").len(), 0);
  assert_eq!(flatimage.bind().list().expect("fim-bind list answers").len(), 0);
}

// covers: FR-CONFIG-001
#[test]
fn an_unknown_key_inside_the_config_block_is_refused() {
  let harness = Harness::new("config-unknown-key");
  let text = format!(
    indoc! {r"
      image:
        from: {}
        output: {}
      config:
        artifacts: []
    "},
    harness.path_file_from().display(),
    harness.path_file_output().display()
  );
  fs::write(harness.path_file_self(), text).expect("the FlatCompose file is writable");

  let outcome = harness.run();

  assert!(!outcome.succeeded(), "{outcome:?}");
  assert!(outcome.stderr.contains("artifacts"), "{outcome:?}");
}

// covers: FR-CONFIG-003
#[test]
fn a_value_carrying_spaces_and_equals_arrives_unchanged() {
  let mut harness = Harness::new("config-value-verbatim");
  harness.compose().config.env = Some(Env {
    entries: vec![Variable {
      name: "ARGS".into(),
      value: "a=b c".into(),
    }],
  });
  harness.write();

  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");

  let entries = FlatImage::of(&harness.path_file_output())
    .env()
    .list()
    .expect("fim-env list answers");
  assert_eq!(entries.len(), 1);
  assert_eq!(entries[0].value, "a=b c");
}

// covers: FR-CONFIG-003
#[test]
fn a_value_the_flatimage_refuses_ends_the_build() {
  let mut harness = Harness::new("config-value-refused");
  harness.compose().config.perms = Some(Perms {
    entries: vec!["nonsense".into()],
  });
  harness.write();

  let outcome = harness.run();

  assert!(!outcome.succeeded(), "{outcome:?}");
  assert!(!outcome.stderr.is_empty(), "{outcome:?}");
  assert!(!path_file_digest(&harness.path_file_output()).exists(), "{outcome:?}");
}
