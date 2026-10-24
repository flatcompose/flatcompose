//! `FR-OVERLAY`: the overlay backend the image stores.

use flatcompose::compose::config::Overlay;

use crate::common::{FlatImage, Harness, path_file_digest};

// covers: FR-OVERLAY-001
#[test]
fn overlay_reaches_the_output_file() {
  let mut harness = Harness::new("overlay-backend");
  harness.compose().config.overlay = Some(Overlay {
    backend: "overlayfs".into(),
  });
  harness.write();

  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");

  let backend = FlatImage::of(&harness.path_file_output())
    .overlay()
    .show()
    .expect("fim-overlay show answers");
  assert_eq!(backend, "overlayfs");
}

// covers: FR-OVERLAY-001
#[test]
fn a_backend_the_flatimage_does_not_know_ends_the_build() {
  let mut harness = Harness::new("overlay-unknown");
  harness.compose().config.overlay = Some(Overlay {
    backend: "somethingelse".into(),
  });
  harness.write();

  let outcome = harness.run();

  assert!(!outcome.succeeded(), "{outcome:?}");
  assert!(!outcome.stderr.is_empty(), "{outcome:?}");
  assert!(!path_file_digest(&harness.path_file_output()).exists(), "{outcome:?}");
}
