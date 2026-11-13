//! `FR-BUILDER`: the `builder` command of a step.

use std::fs;

use flatcompose::compose::steps::{Builder, Install, Step, Steps};
use indoc::indoc;

use crate::common::{Harness, path_file_digest};

// covers: FR-BUILDER-002
#[test]
fn a_step_that_names_no_source_is_refused() {
  let harness = Harness::new("steps-source-absent");
  let text = format!(
    indoc! {r"
      image:
        from: {}
        output: {}
      steps:
        - builder:
            install:
              patterns:
                - firefox-esr
    "},
    harness.path_file_from().display(),
    harness.path_file_output().display()
  );
  fs::write(harness.path_file_self(), text).expect("the FlatCompose file is writable");

  let outcome = harness.run();

  assert!(outcome.refused_with("missing field `from`"), "{outcome:?}");
  assert!(!path_file_digest(&harness.path_file_output()).exists());
}

// covers: FR-BUILDER-003
#[test]
fn a_step_that_names_no_install_request_is_refused() {
  let harness = Harness::new("steps-install-absent");
  let text = format!(
    indoc! {r"
      image:
        from: {}
        output: {}
      steps:
        - builder:
            from: debian:bookworm
    "},
    harness.path_file_from().display(),
    harness.path_file_output().display()
  );
  fs::write(harness.path_file_self(), text).expect("the FlatCompose file is writable");

  let outcome = harness.run();

  assert!(outcome.refused_with("missing field `install`"), "{outcome:?}");
  assert!(!path_file_digest(&harness.path_file_output()).exists());
}

// covers: FR-BUILDER-006
#[test]
fn a_builder_whose_install_names_no_pattern_is_refused() {
  let mut harness = Harness::new("steps-pattern-absent");
  harness.compose().steps = Steps {
    entries: vec![Step {
      builder: Some(Builder {
        from: "debian:bookworm".into(),
        arch: None,
        http_retries: None,
        install: Install {
          patterns: Vec::new(),
          pattern_type: None,
          pattern_match: None,
          with: None,
          exclude: None,
          postinstall: None,
          parallel: None,
          no_deps: false,
        },
      }),
      exec: None,
      layer: None,
    }],
  };
  harness.write();

  let outcome = harness.run();

  assert!(outcome.refused_with("steps[0].builder.install.patterns"), "{outcome:?}");
  assert!(!path_file_digest(&harness.path_file_output()).exists());
}
