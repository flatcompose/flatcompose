//! `FR-SANDBOX`: the sandbox mode the image stores.

use flatcompose::compose::config::Sandbox;

use crate::common::{FlatImage, Harness};

// covers: FR-SANDBOX-001
#[test]
fn sandbox_reaches_the_output_file() {
  let mut harness = Harness::new("sandbox-bwrap");
  harness.compose().config.sandbox = Some(Sandbox { mode: "bwrap".into() });
  harness.write();

  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");

  let mode = FlatImage::of(&harness.path_file_output())
    .sandbox()
    .show()
    .expect("fim-sandbox show answers");
  assert_eq!(mode, "bwrap");
}

// covers: FR-SANDBOX-001
#[test]
fn the_other_mode_reaches_the_output_file() {
  let mut harness = Harness::new("sandbox-none");
  harness.compose().config.sandbox = Some(Sandbox { mode: "none".into() });
  harness.write();

  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");

  let mode = FlatImage::of(&harness.path_file_output())
    .sandbox()
    .show()
    .expect("fim-sandbox show answers");
  assert_eq!(mode, "none");
}
