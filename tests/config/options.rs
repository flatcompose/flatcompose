//! `FR-OPTIONS`: the flatimage toggles the image enables.

use flatcompose::compose::config::Options;

use crate::common::{FlatImage, Harness};

// covers: FR-OPTIONS-001
#[test]
fn options_reach_the_output_file() {
  let mut harness = Harness::new("options-short");
  harness.compose().config.options = Some(Options {
    entries: vec!["casefold".into()],
  });
  harness.write();

  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");

  let mut entries = FlatImage::of(&harness.path_file_output())
    .options()
    .list()
    .expect("fim-options list answers");
  entries.sort();
  assert_eq!(entries, ["casefold"]);
}

// covers: FR-OPTIONS-001
#[test]
fn two_options_reach_the_output_file() {
  let mut harness = Harness::new("options-list");
  harness.compose().config.options = Some(Options {
    entries: vec!["casefold".into(), "notify".into()],
  });
  harness.write();

  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");

  let mut entries = FlatImage::of(&harness.path_file_output())
    .options()
    .list()
    .expect("fim-options list answers");
  entries.sort();
  assert_eq!(entries, ["casefold", "notify"]);
}
