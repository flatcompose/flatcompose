//! `FR-ENV`: the variables the image stores.

use flatcompose::compose::config::{Env, Variable};

use crate::common::{FlatImage, Harness};

// covers: FR-ENV-001
#[test]
fn env_reaches_the_output_file() {
  let mut harness = Harness::new("env-short");
  harness.compose().config.env = Some(Env {
    entries: vec![Variable {
      name: "LANG".into(),
      value: "C.UTF-8".into(),
    }],
  });
  harness.write();

  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");

  let entries = FlatImage::of(&harness.path_file_output())
    .env()
    .list()
    .expect("fim-env list answers");
  assert_eq!(entries.len(), 1);
  assert_eq!(entries[0].name, "LANG");
  assert_eq!(entries[0].value, "C.UTF-8");
}

// covers: FR-ENV-001
#[test]
fn two_variables_reach_the_output_file() {
  let mut harness = Harness::new("env-map");
  harness.compose().config.env = Some(Env {
    entries: vec![
      Variable {
        name: "LANG".into(),
        value: "C.UTF-8".into(),
      },
      Variable {
        name: "EDITOR".into(),
        value: "vi".into(),
      },
    ],
  });
  harness.write();

  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");

  let mut entries = FlatImage::of(&harness.path_file_output())
    .env()
    .list()
    .expect("fim-env list answers");
  entries.sort_by(|left, right| left.name.cmp(&right.name));
  assert_eq!(entries.len(), 2);
  assert_eq!(entries[0].name, "EDITOR");
  assert_eq!(entries[1].name, "LANG");
}

// covers: FR-ENV-001
#[test]
fn a_value_carrying_an_equals_sign_keeps_its_tail() {
  let mut harness = Harness::new("env-equals-tail");
  harness.compose().config.env = Some(Env {
    entries: vec![Variable {
      name: "ARGS".into(),
      value: "a=b".into(),
    }],
  });
  harness.write();

  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");

  let entries = FlatImage::of(&harness.path_file_output())
    .env()
    .list()
    .expect("fim-env list answers");
  assert_eq!(entries.len(), 1);
  assert_eq!(entries[0].name, "ARGS");
  assert_eq!(entries[0].value, "a=b");
}
