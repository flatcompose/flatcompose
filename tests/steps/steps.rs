//! `FR-STEPS`: the commands the block declares.

use std::fs;

use flatcompose::compose::steps::{Builder, Install, Step, Steps};
use indoc::indoc;

use crate::common::{FlatImage, Harness, path_file_digest};
use crate::flatimage::Collection;

// covers: FR-STEPS-002
#[test]
fn a_builder_step_builds_the_root_it_names() {
  let mut harness = Harness::new("steps-root");
  harness.compose().steps = Steps {
    entries: vec![Step {
      builder: Some(Builder {
        from: "debian:bookworm".into(),
        arch: None,
        http_retries: None,
        install: Install {
          patterns: vec!["hello".into()],
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
    }],
  };
  harness.write();

  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");

  let roots = FlatImage::of(&harness.path_file_output())
    .root()
    .system()
    .items()
    .expect("fim-root system list answers");
  let names: Vec<&str> = roots.iter().map(|root| root.name.as_str()).collect();
  assert_eq!(names, ["debian-bookworm"]);
}

// covers: FR-STEPS-001
#[test]
fn a_step_stating_a_command_other_than_builder_is_refused() {
  let harness = Harness::new("steps-command-other");
  let text = format!(
    indoc! {r"
      image:
        from: {}
        output: {}
      steps:
        - system:
            use: debian-bookworm
    "},
    harness.path_file_from().display(),
    harness.path_file_output().display()
  );
  fs::write(harness.path_file_self(), text).expect("the FlatCompose file is writable");

  let outcome = harness.run();

  assert!(outcome.refused_with("builder"), "{outcome:?}");
  assert!(!path_file_digest(&harness.path_file_output()).exists());
}

// covers: FR-STEPS-002
#[test]
fn a_builder_step_runs_a_command_against_the_flatimage() {
  let mut harness = Harness::new("steps-run");
  harness.compose().steps = Steps {
    entries: vec![Step {
      builder: Some(Builder {
        from: "nosuchdistro:1".into(),
        arch: None,
        http_retries: None,
        install: Install {
          patterns: vec!["firefox-esr".into()],
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
    }],
  };
  harness.write();

  let outcome = harness.run();

  assert!(outcome.refused_with("`fim-root` failed"), "{outcome:?}");
  assert!(!path_file_digest(&harness.path_file_output()).exists());
}
