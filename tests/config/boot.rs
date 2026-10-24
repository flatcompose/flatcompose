//! `FR-BOOT`: the command the image starts.

use std::path::PathBuf;

use flatcompose::compose::config::Boot;

use crate::common::{FlatImage, Harness};

// covers: FR-BOOT-001
#[test]
fn boot_reaches_the_output_file() {
  let mut harness = Harness::new("boot-args");
  harness.compose().config.boot = Some(Boot {
    program: PathBuf::from("/usr/bin/true"),
    args: vec!["one".into(), "two".into()],
  });
  harness.write();

  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");

  let boot = FlatImage::of(&harness.path_file_output())
    .boot()
    .show()
    .expect("fim-boot show answers");
  assert_eq!(boot.program, "/usr/bin/true");
  assert_eq!(boot.args, ["one", "two"]);
}

// covers: FR-BOOT-001
#[test]
fn a_program_without_args_reaches_the_output_file() {
  let mut harness = Harness::new("boot-no-args");
  harness.compose().config.boot = Some(Boot {
    program: PathBuf::from("/usr/bin/true"),
    args: vec![],
  });
  harness.write();

  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");

  let boot = FlatImage::of(&harness.path_file_output())
    .boot()
    .show()
    .expect("fim-boot show answers");
  assert_eq!(boot.program, "/usr/bin/true");
  assert!(boot.args.is_empty());
}
