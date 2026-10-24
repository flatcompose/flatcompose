//! `FR-BIND`: the host paths the image binds.

use std::fs;
use std::path::PathBuf;

use flatcompose::compose::config::{Bind, Binding, Mode};
use indoc::indoc;

use crate::common::{FlatImage, Harness};

// covers: FR-BIND-001
#[test]
fn bind_reaches_the_output_file() {
  let mut harness = Harness::new("bind-short");
  harness.compose().config.bind = Some(Bind {
    entries: vec![Binding {
      src: PathBuf::from("/tmp/in"),
      dst: PathBuf::from("/in"),
      mode: Mode::Ro,
    }],
  });
  harness.write();

  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");

  let entries = FlatImage::of(&harness.path_file_output())
    .bind()
    .list()
    .expect("fim-bind list answers");
  assert_eq!(entries.len(), 1);
  assert_eq!(entries[0].src, "/tmp/in");
  assert_eq!(entries[0].dst, "/in");
  assert_eq!(entries[0].mode, "ro");
}

// covers: FR-BIND-001
#[test]
fn two_bindings_reach_the_output_file() {
  let mut harness = Harness::new("bind-list");
  harness.compose().config.bind = Some(Bind {
    entries: vec![
      Binding {
        src: PathBuf::from("/tmp/in"),
        dst: PathBuf::from("/in"),
        mode: Mode::Ro,
      },
      Binding {
        src: PathBuf::from("/tmp/out"),
        dst: PathBuf::from("/out"),
        mode: Mode::Rw,
      },
    ],
  });
  harness.write();

  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");

  let entries = FlatImage::of(&harness.path_file_output())
    .bind()
    .list()
    .expect("fim-bind list answers");
  assert_eq!(entries.len(), 2);
  assert!(
    entries
      .iter()
      .any(|binding| binding.dst == "/in" && binding.mode == "ro")
  );
  assert!(
    entries
      .iter()
      .any(|binding| binding.dst == "/out" && binding.mode == "rw")
  );
}

// covers: FR-BIND-001
#[test]
fn a_mode_outside_the_valid_values_is_refused() {
  let harness = Harness::new("bind-bad-mode");
  let text = format!(
    indoc! {r"
      image:
        from: {}
        output: {}
      config:
        bind:
          - src: /tmp/in
            dst: /in
            mode: rwx
    "},
    harness.path_file_from().display(),
    harness.path_file_output().display()
  );
  fs::write(harness.path_file_self(), text).expect("the FlatCompose file is writable");

  let outcome = harness.run();

  assert!(outcome.refused_with("unknown variant `rwx`"), "{outcome:?}");
  assert!(outcome.stderr.contains("config.bind[0].mode"), "{outcome:?}");
}

// covers: FR-BIND-001
#[test]
fn a_binding_missing_a_field_is_refused() {
  let harness_mode = Harness::new("bind-missing-mode");
  let text_mode = format!(
    indoc! {r"
      image:
        from: {}
        output: {}
      config:
        bind:
          - src: /tmp/in
            dst: /in
    "},
    harness_mode.path_file_from().display(),
    harness_mode.path_file_output().display()
  );
  fs::write(harness_mode.path_file_self(), text_mode).expect("the FlatCompose file is writable");

  let outcome_mode = harness_mode.run();
  assert!(!outcome_mode.succeeded(), "{outcome_mode:?}");
  assert!(outcome_mode.stderr.contains("mode"), "{outcome_mode:?}");

  let harness_dst = Harness::new("bind-missing-dst");
  let text_dst = format!(
    indoc! {r"
      image:
        from: {}
        output: {}
      config:
        bind:
          - src: /tmp/in
            mode: ro
    "},
    harness_dst.path_file_from().display(),
    harness_dst.path_file_output().display()
  );
  fs::write(harness_dst.path_file_self(), text_dst).expect("the FlatCompose file is writable");

  let outcome_dst = harness_dst.run();
  assert!(!outcome_dst.succeeded(), "{outcome_dst:?}");
  assert!(outcome_dst.stderr.contains("dst"), "{outcome_dst:?}");
}
