//! `FR-CAPSULE`: the host driver stacks the image links.

use flatcompose::compose::config::Capsule;

use crate::common::{FlatImage, Harness};

// covers: FR-CAPSULE-001
#[test]
fn capsule_reaches_the_output_file() {
  let mut harness = Harness::new("capsule-list");
  harness.compose().config.capsule = Some(Capsule {
    entries: vec!["gpu".into(), "audio".into()],
  });
  harness.write();

  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");

  let mut entries = FlatImage::of(&harness.path_file_output())
    .capsule()
    .list()
    .expect("fim-capsule list answers");
  entries.sort();
  assert_eq!(entries, ["audio", "gpu"]);
}

// covers: FR-CAPSULE-001
#[test]
fn a_single_capsule_reaches_the_output_file() {
  let mut harness = Harness::new("capsule-short");
  harness.compose().config.capsule = Some(Capsule {
    entries: vec!["gpu".into()],
  });
  harness.write();

  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");

  let entries = FlatImage::of(&harness.path_file_output())
    .capsule()
    .list()
    .expect("fim-capsule list answers");
  assert_eq!(entries, ["gpu"]);
}
