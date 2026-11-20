//! `FR-EXEC`: the `exec` command of a step.

use std::fs;

use flatcompose::compose::config::Unshare;
use flatcompose::compose::steps::{Builder, Exec, Install, Step, Steps};
use indoc::indoc;

use crate::common::{Harness, path_file_digest};

// covers: FR-STEPS-004, FR-EXEC-001, FR-EXEC-003
#[test]
fn an_exec_step_runs_its_script_in_the_root_it_names() {
  let mut harness = Harness::new("exec-root");
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
        builder: Some(Builder {
          from: "debian:bullseye".into(),
          arch: None,
          http_retries: None,
          install: Install {
            patterns: vec!["zip".into()],
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
          from: "debian:bullseye".into(),
          uid: None,
          gid: None,
          script: "test -x /usr/bin/zip".into(),
          position: 0,
        }),
        layer: None,
      },
    ],
  };
  harness.write();

  let outcome = harness.run();

  assert!(outcome.succeeded(), "{outcome:?}");
  assert!(path_file_digest(&harness.path_file_output()).exists());
}

// covers: FR-EXEC-005, FR-EXEC-006
#[test]
fn an_exec_step_runs_its_script_as_the_uid_and_gid_it_names() {
  let mut harness = Harness::new("exec-uid-gid");

  // Switching uid/gid needs the user namespace. The image contains that setting first.
  harness.compose().config.unshare = Some(Unshare {
    entries: vec!["user".into()],
  });
  harness.write();
  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");

  let script = indoc! {r#"
    test "$EUID" = "1000"
    test "${GROUPS[0]}" = "4242"
  "#};
  harness.compose().image.inplace = true;
  harness.compose().image.from = None;
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
          uid: Some(1000),
          gid: Some(4242),
          script: script.into(),
          position: 0,
        }),
        layer: None,
      },
    ],
  };
  harness.write();

  let outcome = harness.run();

  assert!(outcome.succeeded(), "{outcome:?}");
  assert!(path_file_digest(&harness.path_file_output()).exists());
}

// covers: FR-STEPS-003, FR-EXEC-004
#[test]
fn an_exec_step_runs_every_script_in_the_order_the_file_writes_them() {
  let mut harness = Harness::new("exec-order");
  let marker = harness
    .path_dir_output()
    .file_name()
    .expect("the output directory has a name")
    .to_string_lossy()
    .into_owned();
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
          script: format!("# {marker}\ntouch /marker\n"),
          position: 0,
        }),
        layer: None,
      },
      Step {
        builder: None,
        exec: Some(Exec {
          from: "debian:bookworm".into(),
          uid: None,
          gid: None,
          script: "test -f /marker".into(),
          position: 0,
        }),
        layer: None,
      },
    ],
  };
  harness.write();

  let outcome = harness.run();

  assert!(outcome.succeeded(), "{outcome:?}");
  let script_left = fs::read_dir("/tmp/fim/flatcompose").is_ok_and(|entries| {
    entries
      .filter_map(Result::ok)
      .filter_map(|entry| fs::read_dir(entry.path()).ok())
      .flatten()
      .filter_map(Result::ok)
      .any(|entry| fs::read_to_string(entry.path()).is_ok_and(|text| text.contains(marker.as_str())))
  });
  assert!(script_left, "the build removed the script of a step");
}

// covers: FR-EXEC-004
#[test]
fn an_exec_step_runs_its_script_text_verbatim() {
  let mut harness = Harness::new("exec-quoting");
  let script = indoc! {r#"
    read -r line <<'EOF'
    a b "c" $HOME `echo d`
    EOF
    test "$line" = 'a b "c" $HOME `echo d`'
  "#};
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
          script: script.into(),
          position: 0,
        }),
        layer: None,
      },
    ],
  };
  harness.write();

  let outcome = harness.run();

  assert!(outcome.succeeded(), "{outcome:?}");
  assert!(path_file_digest(&harness.path_file_output()).exists());
}

// covers: FR-EXEC-004
#[test]
fn an_exec_step_names_its_script_by_the_step_position() {
  let mut harness = Harness::new("exec-failed");
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
          script: "if true; then\n".into(),
          position: 0,
        }),
        layer: None,
      },
    ],
  };
  harness.write();

  let outcome = harness.run();

  assert!(outcome.refused_with("script-1.sh"), "{outcome:?}");
  assert!(!path_file_digest(&harness.path_file_output()).exists());
}

// covers: FR-EXEC-002
#[test]
fn an_exec_step_that_names_no_source_is_refused() {
  let harness = Harness::new("exec-source-absent");
  let text = format!(
    indoc! {r"
      image:
        from: {}
        output: {}
      steps:
        - exec:
            script: |
              true
    "},
    harness.path_file_from().display(),
    harness.path_file_output().display()
  );
  fs::write(harness.path_file_self(), text).expect("the FlatCompose file is writable");

  let outcome = harness.run();

  assert!(outcome.refused_with("missing field `from`"), "{outcome:?}");
  assert!(!path_file_digest(&harness.path_file_output()).exists());
}

// covers: FR-EXEC-004
#[test]
fn an_exec_step_whose_middle_command_fails_ends_the_build() {
  let mut harness = Harness::new("exec-middle-failure");
  let script = indoc! {r#"
    false
    echo ok
  "#};
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
          script: script.into(),
          position: 0,
        }),
        layer: None,
      },
    ],
  };
  harness.write();

  let outcome = harness.run();

  assert!(outcome.refused_with("`fim-exec` failed"), "{outcome:?}");
  assert!(!path_file_digest(&harness.path_file_output()).exists());
}

// covers: FR-EXEC-004
#[test]
fn an_exec_step_can_disable_errexit() {
  let mut harness = Harness::new("exec-errexit-off");
  let script = indoc! {r#"
    set +e
    false
    echo ok
  "#};
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
          script: script.into(),
          position: 0,
        }),
        layer: None,
      },
    ],
  };
  harness.write();

  let outcome = harness.run();

  assert!(outcome.succeeded(), "{outcome:?}");
  assert!(path_file_digest(&harness.path_file_output()).exists());
}

// covers: FR-EXEC-004
#[test]
fn an_exec_step_whose_pipeline_stage_fails_ends_the_build() {
  let mut harness = Harness::new("exec-pipeline-failure");
  let script = indoc! {r#"
    false | true
  "#};
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
          script: script.into(),
          position: 0,
        }),
        layer: None,
      },
    ],
  };
  harness.write();

  let outcome = harness.run();

  assert!(outcome.refused_with("`fim-exec` failed"), "{outcome:?}");
  assert!(!path_file_digest(&harness.path_file_output()).exists());
}

// covers: FR-EXEC-004
#[test]
fn an_exec_step_that_names_no_script_is_refused() {
  let harness = Harness::new("exec-script-absent");
  let text = format!(
    indoc! {r"
      image:
        from: {}
        output: {}
      steps:
        - exec:
            from: debian:bookworm
    "},
    harness.path_file_from().display(),
    harness.path_file_output().display()
  );
  fs::write(harness.path_file_self(), text).expect("the FlatCompose file is writable");

  let outcome = harness.run();

  assert!(outcome.refused_with("missing field `script`"), "{outcome:?}");
  assert!(!path_file_digest(&harness.path_file_output()).exists());
}

// covers: FR-EXEC-004
#[test]
fn an_exec_step_whose_script_is_empty_is_refused() {
  let mut harness = Harness::new("exec-script-empty");
  harness.compose().steps = Steps {
    entries: vec![Step {
      builder: None,
      exec: Some(Exec {
        from: "debian:bookworm".into(),
        uid: None,
        gid: None,
        script: "".into(),
        position: 0,
      }),
      layer: None,
    }],
  };
  harness.write();

  let outcome = harness.run();

  assert!(outcome.refused_with("steps[0].exec.script"), "{outcome:?}");
  assert!(!path_file_digest(&harness.path_file_output()).exists());
}

// covers: FR-STEPS-003, FR-EXEC-003
#[test]
fn an_exec_step_before_the_step_that_builds_its_root_is_refused() {
  let mut harness = Harness::new("exec-before-builder");
  harness.compose().steps = Steps {
    entries: vec![
      Step {
        builder: None,
        exec: Some(Exec {
          from: "debian:bookworm".into(),
          uid: None,
          gid: None,
          script: "true".into(),
          position: 0,
        }),
        layer: None,
      },
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
    ],
  };
  harness.write();

  let outcome = harness.run();

  assert!(outcome.refused_with("does not exist"), "{outcome:?}");
  assert!(!path_file_digest(&harness.path_file_output()).exists());
}
