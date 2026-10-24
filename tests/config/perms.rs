//! `FR-PERMS`: the permissions the sandbox grants.

use flatcompose::compose::config::Perms;

use crate::common::{FlatImage, Harness};

// covers: FR-PERMS-001
#[test]
fn perms_reach_the_output_file() {
  let mut harness = Harness::new("perms-list");
  harness.compose().config.perms = Some(Perms {
    entries: vec!["network".into(), "home".into(), "gpu".into()],
  });
  harness.write();

  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");

  let mut entries = FlatImage::of(&harness.path_file_output())
    .perms()
    .list()
    .expect("fim-perms list answers");
  entries.sort();
  assert_eq!(entries, ["gpu", "home", "network"]);
}

// covers: FR-PERMS-001
#[test]
fn a_single_permission_reaches_the_output_file() {
  let mut harness = Harness::new("perms-short");
  harness.compose().config.perms = Some(Perms {
    entries: vec!["network".into()],
  });
  harness.write();

  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");

  let entries = FlatImage::of(&harness.path_file_output())
    .perms()
    .list()
    .expect("fim-perms list answers");
  assert_eq!(entries, ["network"]);
}
