//! `FR-ENV-VAR`: the environment a FlatCompose file takes values from

use std::fs;
use std::path::PathBuf;

use flatcompose::compose::config::{Env, Perms, Variable};
use flatcompose::compose::steps::{Builder, Exec, Install, Step, Steps};
use indoc::indoc;

use crate::common::{FlatImage, Harness, path_file_digest};

// covers: FR-ENV-VAR-001
#[test]
fn a_braced_reference_takes_the_environment_value() {
  let mut harness = Harness::new("env-braced");
  harness.compose().config.env = Some(Env {
    entries: vec![Variable {
      name: "EDITOR".into(),
      value: "${EDITOR}".into(),
    }],
  });
  harness.write();

  let outcome = harness.run_with_env(&[("EDITOR", "vim")]);
  assert!(outcome.succeeded(), "{outcome:?}");

  let entries = FlatImage::of(&harness.path_file_output())
    .env()
    .list()
    .expect("fim-env list answers");
  assert_eq!(entries.len(), 1);
  assert_eq!(entries[0].name, "EDITOR");
  assert_eq!(entries[0].value, "vim");
}

// covers: FR-ENV-VAR-001
#[test]
fn a_bare_reference_takes_the_environment_value() {
  let mut harness = Harness::new("env-bare");
  harness.compose().config.env = Some(Env {
    entries: vec![Variable {
      name: "EDITOR".into(),
      value: "$EDITOR".into(),
    }],
  });
  harness.write();

  let outcome = harness.run_with_env(&[("EDITOR", "vim")]);
  assert!(outcome.succeeded(), "{outcome:?}");

  let entries = FlatImage::of(&harness.path_file_output())
    .env()
    .list()
    .expect("fim-env list answers");
  assert_eq!(entries.len(), 1);
  assert_eq!(entries[0].value, "vim");
}

// covers: FR-ENV-VAR-001
#[test]
fn a_reference_builds_the_published_file() {
  let mut harness = Harness::new("env-output");
  let path_file_output = harness.path_file_output();
  harness.compose().image.output = PathBuf::from("${APP}");
  harness.write();
  let path_app = path_file_output.display().to_string();

  let outcome = harness.run_with_env(&[("APP", path_app.as_str())]);
  assert!(outcome.succeeded(), "{outcome:?}");
  assert!(path_file_output.exists(), "the build wrote no file at the resolved path");
  assert!(path_file_digest(&path_file_output).exists(), "the build wrote no digest");
}

// covers: FR-ENV-VAR-001
#[test]
fn a_reference_in_a_list_item_reaches_the_image() {
  let mut harness = Harness::new("env-list");
  harness.compose().config.perms = Some(Perms {
    entries: vec!["${PERM}".into(), "network".into()],
  });
  harness.write();

  let outcome = harness.run_with_env(&[("PERM", "home")]);
  assert!(outcome.succeeded(), "{outcome:?}");

  let perms = FlatImage::of(&harness.path_file_output())
    .perms()
    .list()
    .expect("fim-perms list answers");
  assert_eq!(perms, ["home", "network"]);
}

// covers: FR-ENV-VAR-001
#[test]
fn a_reference_in_a_mapping_key_reaches_the_image() {
  let mut harness = Harness::new("env-key");
  harness.compose().config.env = Some(Env {
    entries: vec![Variable {
      name: "${NAME}".into(),
      value: "${VALUE}".into(),
    }],
  });
  harness.write();

  let outcome = harness.run_with_env(&[("NAME", "LANG"), ("VALUE", "C.UTF-8")]);
  assert!(outcome.succeeded(), "{outcome:?}");

  let entries = FlatImage::of(&harness.path_file_output())
    .env()
    .list()
    .expect("fim-env list answers");
  assert_eq!(entries.len(), 1);
  assert_eq!(entries[0].name, "LANG");
  assert_eq!(entries[0].value, "C.UTF-8");
}

// covers: FR-ENV-VAR-002
#[test]
fn a_default_fills_an_unset_variable() {
  let mut harness = Harness::new("env-default-unset");
  harness.compose().config.env = Some(Env {
    entries: vec![Variable {
      name: "GREETING".into(),
      value: "${GREETING:-hello}".into(),
    }],
  });
  harness.write();

  let outcome = harness.run_with_env(&[]);
  assert!(outcome.succeeded(), "{outcome:?}");

  let entries = FlatImage::of(&harness.path_file_output())
    .env()
    .list()
    .expect("fim-env list answers");
  assert_eq!(entries[0].value, "hello");
}

// covers: FR-ENV-VAR-002
#[test]
fn a_default_fills_an_empty_variable() {
  let mut harness = Harness::new("env-default-empty");
  harness.compose().config.env = Some(Env {
    entries: vec![Variable {
      name: "GREETING".into(),
      value: "${GREETING:-hello}".into(),
    }],
  });
  harness.write();

  let outcome = harness.run_with_env(&[("GREETING", "")]);
  assert!(outcome.succeeded(), "{outcome:?}");

  let entries = FlatImage::of(&harness.path_file_output())
    .env()
    .list()
    .expect("fim-env list answers");
  assert_eq!(entries[0].value, "hello");
}

// covers: FR-ENV-VAR-003
#[test]
fn a_default_without_a_colon_fills_an_unset_variable() {
  let mut harness = Harness::new("env-short-default-unset");
  harness.compose().config.env = Some(Env {
    entries: vec![Variable {
      name: "GREETING".into(),
      value: "${GREETING-hello}".into(),
    }],
  });
  harness.write();

  let outcome = harness.run_with_env(&[]);
  assert!(outcome.succeeded(), "{outcome:?}");

  let entries = FlatImage::of(&harness.path_file_output())
    .env()
    .list()
    .expect("fim-env list answers");
  assert_eq!(entries[0].value, "hello");
}

// covers: FR-ENV-VAR-003
#[test]
fn a_default_without_a_colon_keeps_an_empty_value() {
  let mut harness = Harness::new("env-short-default-empty");
  harness.compose().config.env = Some(Env {
    entries: vec![Variable {
      name: "GREETING".into(),
      value: "${GREETING-hello}".into(),
    }],
  });
  harness.write();

  let outcome = harness.run_with_env(&[("GREETING", "")]);
  assert!(outcome.succeeded(), "{outcome:?}");

  let entries = FlatImage::of(&harness.path_file_output())
    .env()
    .list()
    .expect("fim-env list answers");
  assert_eq!(entries[0].value, "");
}

// covers: FR-ENV-VAR-001, FR-ENV-VAR-004
#[test]
fn an_empty_value_is_not_unset() {
  let mut harness = Harness::new("env-empty");
  harness.compose().config.env = Some(Env {
    entries: vec![Variable {
      name: "EDITOR".into(),
      value: "${EDITOR}".into(),
    }],
  });
  harness.write();

  let outcome = harness.run_with_env(&[("EDITOR", "")]);
  assert!(outcome.silent(), "{outcome:?}");

  let entries = FlatImage::of(&harness.path_file_output())
    .env()
    .list()
    .expect("fim-env list answers");
  assert_eq!(entries[0].value, "");
}

// covers: FR-ENV-VAR-005
#[test]
fn a_double_dollar_writes_one_dollar() {
  let mut harness = Harness::new("env-double-dollar");
  harness.compose().config.env = Some(Env {
    entries: vec![Variable {
      name: "PATH".into(),
      value: "/opt/local/bin:$$PATH".into(),
    }],
  });
  harness.write();
  let path_reader = std::env::var("PATH").expect("the reader names its PATH");

  let outcome = harness.run_with_env(&[("PATH", "/build/bin")]);
  assert!(outcome.succeeded(), "{outcome:?}");

  let entries = FlatImage::of(&harness.path_file_output())
    .env()
    .list()
    .expect("fim-env list answers");
  let value = &entries[0].value;
  assert!(value.starts_with("/opt/local/bin:"), "{value}");
  assert!(value.contains(&path_reader), "{value}");
  assert!(!value.contains("/build/bin"), "{value}");
}

// covers: FR-ENV-VAR-005
#[test]
fn a_double_dollar_writes_a_literal_dollar_before_a_digit() {
  let mut harness = Harness::new("env-double-dollar-digit");
  harness.compose().config.env = Some(Env {
    entries: vec![Variable {
      name: "PRICE".into(),
      value: "$$5".into(),
    }],
  });
  harness.write();

  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");

  let entries = FlatImage::of(&harness.path_file_output())
    .env()
    .list()
    .expect("fim-env list answers");
  assert_eq!(entries[0].value, "");
}

// covers: FR-ENV-VAR-005
#[test]
fn a_double_dollar_before_a_braced_reference_keeps_it_literal() {
  let mut harness = Harness::new("env-double-dollar-brace");
  harness.compose().config.env = Some(Env {
    entries: vec![Variable {
      name: "GREETING".into(),
      value: "$${HOME}".into(),
    }],
  });
  harness.write();
  let expected = std::env::var("HOME").expect("the reader names its HOME");

  let outcome = harness.run_with_env(&[("HOME", "/build/home")]);
  assert!(outcome.succeeded(), "{outcome:?}");

  let entries = FlatImage::of(&harness.path_file_output())
    .env()
    .list()
    .expect("fim-env list answers");
  assert_eq!(entries[0].value, expected);
}

// covers: FR-ENV-VAR-006
#[test]
fn a_value_from_the_environment_resolves_once() {
  let mut harness = Harness::new("env-once");
  harness.compose().config.env = Some(Env {
    entries: vec![Variable {
      name: "GREETING".into(),
      value: "${VALUE}".into(),
    }],
  });
  harness.write();
  let expected = std::env::var("HOME").expect("the reader names its HOME");

  let outcome = harness.run_with_env(&[("VALUE", "${HOME}"), ("HOME", "/build/home")]);
  assert!(outcome.succeeded(), "{outcome:?}");

  let entries = FlatImage::of(&harness.path_file_output())
    .env()
    .list()
    .expect("fim-env list answers");
  assert_eq!(entries[0].value, expected);
}

// covers: FR-ENV-VAR-007
#[test]
fn a_reference_resolves_a_boolean() {
  let harness = Harness::new("env-boolean");
  let text = format!(
    indoc! {r"
      image:
        from: {}
        output: {}
        inplace: ${{INPLACE}}
    "},
    harness.path_file_from().display(),
    harness.path_file_output().display()
  );
  fs::write(harness.path_file_self(), text).expect("the FlatCompose file is writable");

  let outcome = harness.run_with_env(&[("INPLACE", "true")]);
  assert!(outcome.succeeded(), "{outcome:?}");
  assert!(path_file_digest(&harness.path_file_output()).exists(), "the build wrote no digest");
}

// covers: FR-ENV-VAR-007
#[test]
fn a_builder_reads_its_options_from_the_environment() {
  let harness = Harness::new("env-builder");
  let text = format!(
    indoc! {r"
      image:
        from: {}
        output: {}
      steps:
        - builder:
            from: ${{SOURCE}}
            arch: ${{ARCH}}
            http_retries: ${{RETRIES}}
            install:
              patterns:
                - ${{PACKAGE}}
    "},
    harness.path_file_from().display(),
    harness.path_file_output().display()
  );
  fs::write(harness.path_file_self(), text).expect("the FlatCompose file is writable");

  let outcome = harness.run_with_env(&[
    ("SOURCE", "debian:bookworm"),
    ("ARCH", "x86_64"),
    ("RETRIES", "3"),
    ("PACKAGE", "hello"),
  ]);
  assert!(outcome.succeeded(), "{outcome:?}");
  assert!(path_file_digest(&harness.path_file_output()).exists(), "the build wrote no digest");
}

// covers: FR-ENV-VAR-004, FR-ENV-VAR-007
#[test]
fn the_exec_script_is_left_as_written() {
  let mut harness = Harness::new("env-script");
  let marker = harness
    .path_dir_output()
    .file_name()
    .expect("the output directory has a name")
    .to_string_lossy()
    .into_owned();
  let script = format!("# {marker}\necho $FLATCOMPOSE_SCRIPT_CHECK\n");
  harness.compose().steps = Steps {
    entries: vec![
      Step {
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
        layer: None,
      },
      Step {
        builder: None,
        exec: Some(Exec {
          from: "debian:bookworm".into(),
          uid: None,
          gid: None,
          script,
          position: 0,
        }),
        layer: None,
      },
    ],
  };
  harness.write();

  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");
  assert!(!outcome.stderr.contains("FLATCOMPOSE_SCRIPT_CHECK"), "{outcome:?}");

  let script_left = fs::read_dir("/tmp/fim/flatcompose").is_ok_and(|entries| {
    entries
      .filter_map(Result::ok)
      .filter_map(|entry| fs::read_dir(entry.path()).ok())
      .flatten()
      .filter_map(Result::ok)
      .filter_map(|entry| fs::read_to_string(entry.path()).ok())
      .any(|text| text.contains(&marker) && text.contains("$FLATCOMPOSE_SCRIPT_CHECK"))
  });
  assert!(script_left, "the build did not leave the script as written");
}

// covers: FR-ENV-VAR-005
#[test]
fn an_unescaped_dollar_is_refused() {
  let mut harness = Harness::new("env-dollar-refused");
  harness.compose().config.env = Some(Env {
    entries: vec![Variable {
      name: "PRICE".into(),
      value: "$!".into(),
    }],
  });
  harness.write();

  let outcome = harness.run();

  assert!(outcome.refused_with("resolving environment references"), "{outcome:?}");
  assert!(!path_file_digest(&harness.path_file_output()).exists());
}

// covers: FR-ENV-VAR-007
#[test]
fn a_count_reference_that_is_not_a_number_is_refused() {
  let harness = Harness::new("env-count-refused");
  let text = format!(
    indoc! {r"
      image:
        from: {}
        output: {}
      steps:
        - exec:
            from: debian:bookworm
            uid: ${{UID}}
            script: |
              true
    "},
    harness.path_file_from().display(),
    harness.path_file_output().display()
  );
  fs::write(harness.path_file_self(), text).expect("the FlatCompose file is writable");

  let outcome = harness.run_with_env(&[("UID", "abc")]);

  assert!(outcome.refused_with("invalid digit"), "{outcome:?}");
  assert!(!path_file_digest(&harness.path_file_output()).exists());
}

// covers: FR-ENV-VAR-007
#[test]
fn a_boolean_reference_that_is_not_a_boolean_is_refused() {
  let harness = Harness::new("env-boolean-refused");
  let text = format!(
    indoc! {r"
      image:
        from: {}
        output: {}
        inplace: ${{INPLACE}}
    "},
    harness.path_file_from().display(),
    harness.path_file_output().display()
  );
  fs::write(harness.path_file_self(), text).expect("the FlatCompose file is writable");

  let outcome = harness.run_with_env(&[("INPLACE", "maybe")]);

  assert!(outcome.refused_with("provided string was not"), "{outcome:?}");
  assert!(!path_file_digest(&harness.path_file_output()).exists());
}

// covers: FR-ENV-VAR-007
#[test]
fn an_architecture_reference_outside_the_five_is_refused() {
  let harness = Harness::new("env-arch-refused");
  let text = format!(
    indoc! {r"
      image:
        from: {}
        output: {}
        arch:
          - ${{ARCH}}
    "},
    harness.path_file_from().display(),
    harness.path_file_output().display()
  );
  fs::write(harness.path_file_self(), text).expect("the FlatCompose file is writable");

  let outcome = harness.run_with_env(&[("ARCH", "mips64")]);

  assert!(outcome.refused_with("mips64"), "{outcome:?}");
  assert!(!path_file_digest(&harness.path_file_output()).exists());
}

// covers: FR-ENV-VAR-004
#[test]
fn an_unset_variable_empties_and_warns() {
  let mut harness = Harness::new("env-unset-warn");
  harness.compose().config.env = Some(Env {
    entries: vec![Variable {
      name: "EDITOR".into(),
      value: "${EDITOR}".into(),
    }],
  });
  harness.write();

  let outcome = harness.run();

  assert!(outcome.succeeded(), "{outcome:?}");
  assert!(
    outcome
      .stderr
      .contains("warning: environment variable `EDITOR` is unset; substituting an empty value"),
    "{outcome:?}"
  );

  let entries = FlatImage::of(&harness.path_file_output())
    .env()
    .list()
    .expect("fim-env list answers");
  assert_eq!(entries[0].value, "");
}

// covers: FR-ENV-VAR-004
#[test]
fn an_unset_variable_warns_once() {
  let mut harness = Harness::new("env-unset-warn-once");
  harness.compose().config.env = Some(Env {
    entries: vec![
      Variable {
        name: "A".into(),
        value: "${EDITOR}".into(),
      },
      Variable {
        name: "B".into(),
        value: "${EDITOR}".into(),
      },
    ],
  });
  harness.write();

  let outcome = harness.run();

  assert!(outcome.succeeded(), "{outcome:?}");
  assert_eq!(
    outcome
      .stderr
      .matches("warning: environment variable `EDITOR` is unset; substituting an empty value")
      .count(),
    1,
    "{outcome:?}"
  );
}
