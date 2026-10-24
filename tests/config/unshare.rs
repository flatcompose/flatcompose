//! `FR-UNSHARE`: the namespaces the sandbox detaches.

use flatcompose::compose::config::Unshare;

use crate::common::{FlatImage, Harness};

// covers: FR-UNSHARE-001
#[test]
fn unshare_reaches_the_output_file() {
  let mut harness = Harness::new("unshare-list");
  harness.compose().config.unshare = Some(Unshare {
    entries: vec!["pid".into(), "net".into()],
  });
  harness.write();

  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");

  let mut entries = FlatImage::of(&harness.path_file_output())
    .unshare()
    .list()
    .expect("fim-unshare list answers");
  entries.sort();
  assert_eq!(entries, ["net", "pid"]);
}

// covers: FR-UNSHARE-001
#[test]
fn a_single_namespace_reaches_the_output_file() {
  let mut harness = Harness::new("unshare-short");
  harness.compose().config.unshare = Some(Unshare {
    entries: vec!["pid".into()],
  });
  harness.write();

  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");

  let entries = FlatImage::of(&harness.path_file_output())
    .unshare()
    .list()
    .expect("fim-unshare list answers");
  assert_eq!(entries, ["pid"]);
}
