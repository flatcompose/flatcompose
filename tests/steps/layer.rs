//! `FR-LAYER`: the `layer` command of a step.

use std::fs;
use std::path::PathBuf;

use flatcompose::compose::steps::{
  Builder, Exec, Install, Layer, Mount, MountClear, MountDel, MountSet, Overlay, OverlaySave, Stack, StackAdd, Step,
  Steps,
};
use indoc::indoc;

use crate::common::{Harness, path_file_digest};
use crate::flatimage::{Collection, FlatImage};

/// The shell script every layer test writes.
const SCRIPT: &str = "#!/bin/sh\necho payload-ok\n";

// covers: FR-STEPS-005
#[test]
fn a_layer_step_builds() {
  let mut harness = Harness::new("layer-builds");
  harness.place_script("payload/bin/payload.sh", SCRIPT);
  harness.compose().steps = Steps {
    entries: vec![Step {
      builder: None,
      exec: None,
      layer: Some(Layer {
        overlay: Some(Overlay {
          save: Some(OverlaySave {
            from: Some(PathBuf::from("payload")),
            to: Some(PathBuf::from("payload.layer")),
          }),
        }),
        stack: None,
        mount: None,
      }),
    }],
  };
  harness.write();

  let outcome = harness.run();

  assert!(outcome.succeeded(), "{outcome:?}");
  assert!(path_file_digest(&harness.path_file_output()).exists(), "the build wrote no digest");
  assert!(harness.path_dir_output().join("payload.layer").exists(), "the build wrote no layer file");
}

// covers: FR-LAYER-002, FR-LAYER-003
#[test]
fn a_saved_directory_layer_appends_to_the_image() {
  let mut harness = Harness::new("layer-source-only");
  harness.place_script("payload/bin/payload.sh", SCRIPT);
  harness.compose().steps = Steps {
    entries: vec![Step {
      builder: None,
      exec: None,
      layer: Some(Layer {
        overlay: Some(Overlay {
          save: Some(OverlaySave {
            from: Some(PathBuf::from("payload")),
            to: None,
          }),
        }),
        stack: None,
        mount: None,
      }),
    }],
  };
  harness.write();

  let outcome = harness.run();

  assert!(outcome.succeeded(), "{outcome:?}");

  let entries = FlatImage::of(&harness.path_file_output())
    .layer()
    .stack()
    .items()
    .expect("fim-layer stack list answers");

  assert!(
    entries.iter().any(|entry| entry.kind == "committed"),
    "the save did not append a committed layer: {entries:?}"
  );
}

// covers: FR-LAYER-002, FR-LAYER-003
#[test]
fn saving_the_live_overlay_to_a_file_writes_that_file() {
  let mut harness = Harness::new("layer-destination-only");
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
        exec: None,
        layer: Some(Layer {
          overlay: Some(Overlay {
            save: Some(OverlaySave {
              from: None,
              to: Some(PathBuf::from("payload.layer")),
            }),
          }),
          stack: None,
          mount: None,
        }),
      },
    ],
  };
  harness.write();

  let outcome = harness.run();

  assert!(outcome.succeeded(), "{outcome:?}");
  assert!(harness.path_dir_output().join("payload.layer").exists(), "the build wrote no layer file");

  let entries = FlatImage::of(&harness.path_file_output())
    .layer()
    .stack()
    .items()
    .expect("fim-layer stack list answers");

  assert!(
    !entries.iter().any(|entry| entry.kind == "committed"),
    "the save appended a layer the step sent to a file: {entries:?}"
  );
}

// covers: FR-LAYER-001, FR-LAYER-002, FR-LAYER-003
#[test]
fn the_saved_overlay_becomes_a_committed_layer() {
  let mut harness = Harness::new("layer-committed");
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
        exec: None,
        layer: Some(Layer {
          overlay: Some(Overlay {
            save: Some(OverlaySave { from: None, to: None }),
          }),
          stack: None,
          mount: None,
        }),
      },
      Step {
        builder: None,
        exec: Some(Exec {
          from: "debian:bookworm".into(),
          uid: None,
          gid: None,
          script: "hello".into(),
          position: 0,
        }),
        layer: None,
      },
    ],
  };
  harness.write();

  let outcome = harness.run();

  assert!(outcome.succeeded(), "{outcome:?}");
  assert!(path_file_digest(&harness.path_file_output()).exists(), "the build wrote no digest");

  let entries = FlatImage::of(&harness.path_file_output())
    .layer()
    .stack()
    .items()
    .expect("fim-layer stack list answers");

  assert!(
    entries.iter().any(|entry| entry.kind == "committed" && entry.size > 0),
    "the save left no committed layer: {entries:?}"
  );
}

// covers: FR-LAYER-004, FR-LAYER-005
#[test]
fn a_layer_file_is_added_to_the_stack() {
  let mut harness = Harness::new("layer-add");
  harness.place_script("payload/bin/payload.sh", SCRIPT);
  harness.compose().steps = Steps {
    entries: vec![
      Step {
        builder: None,
        exec: None,
        layer: Some(Layer {
          overlay: Some(Overlay {
            save: Some(OverlaySave {
              from: Some(PathBuf::from("payload")),
              to: Some(PathBuf::from("payload.layer")),
            }),
          }),
          stack: None,
          mount: None,
        }),
      },
      Step {
        builder: None,
        exec: None,
        layer: Some(Layer {
          overlay: None,
          stack: Some(Stack {
            add: Some(StackAdd {
              path: PathBuf::from("payload.layer"),
              mount: None,
            }),
          }),
          mount: None,
        }),
      },
    ],
  };
  harness.write();

  let outcome = harness.run();

  assert!(outcome.succeeded(), "{outcome:?}");

  let entries = FlatImage::of(&harness.path_file_output())
    .layer()
    .stack()
    .items()
    .expect("fim-layer stack list answers");

  assert!(
    entries
      .iter()
      .any(|entry| entry.kind == "included" && entry.dst.is_empty()),
    "the stack does not hold the layer the step added: {entries:?}"
  );
}

// covers: FR-LAYER-006
#[test]
fn a_layer_file_grafts_at_its_mount() {
  let mut harness = Harness::new("layer-mount");
  harness.place_script("payload/bin/payload.sh", SCRIPT);
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
        exec: None,
        layer: Some(Layer {
          overlay: Some(Overlay {
            save: Some(OverlaySave {
              from: Some(PathBuf::from("payload")),
              to: Some(PathBuf::from("payload.layer")),
            }),
          }),
          stack: None,
          mount: None,
        }),
      },
      Step {
        builder: None,
        exec: None,
        layer: Some(Layer {
          overlay: None,
          stack: Some(Stack {
            add: Some(StackAdd {
              path: PathBuf::from("payload.layer"),
              mount: Some(PathBuf::from("/opt/payload")),
            }),
          }),
          mount: None,
        }),
      },
      Step {
        builder: None,
        exec: Some(Exec {
          from: "debian:bookworm".into(),
          uid: None,
          gid: None,
          script: "test \"$(/opt/payload/bin/payload.sh)\" = payload-ok".into(),
          position: 0,
        }),
        layer: None,
      },
    ],
  };
  harness.write();

  let outcome = harness.run();

  assert!(outcome.succeeded(), "{outcome:?}");

  let entries = FlatImage::of(&harness.path_file_output())
    .layer()
    .stack()
    .items()
    .expect("fim-layer stack list answers");

  assert!(
    entries
      .iter()
      .any(|entry| entry.kind == "included" && entry.dst == "/opt/payload"),
    "the stack does not record the mountpoint the step named: {entries:?}"
  );
}

// covers: FR-LAYER-007, FR-LAYER-008, FR-LAYER-009
#[test]
fn an_external_layer_grafts_at_its_recorded_path() {
  let mut harness = Harness::new("layer-external");
  let path_dir_payload = harness.path_dir_output().join("payload");
  harness.place_script("payload/bin/payload.sh", SCRIPT);
  let path_file_layer = harness.path_dir_output().join("payload.layer");
  let hash = harness.place_layer(&path_dir_payload, &path_file_layer);

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
        exec: None,
        layer: Some(Layer {
          overlay: None,
          stack: None,
          mount: Some(Mount {
            set: Some(MountSet {
              hash: hash.clone(),
              path: PathBuf::from("/opt/payload"),
            }),
            del: None,
            clear: None,
          }),
        }),
      },
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
    ],
  };
  harness.write();

  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");

  let mounts = FlatImage::of(&harness.path_file_output())
    .layer()
    .mount()
    .items()
    .expect("fim-layer mount list answers");
  assert!(
    mounts
      .iter()
      .any(|entry| entry.hash == hash && entry.dst == "/opt/payload"),
    "the image does not record the mountpoint the step named: {mounts:?}"
  );

  let printed = FlatImage::of(&harness.path_file_output())
    .exec_with_layer(&path_file_layer, &["/opt/payload/bin/payload.sh"])
    .expect("fim-exec answers");
  assert_eq!(printed.trim(), "payload-ok", "the external layer did not graft at its path");
}

// covers: FR-LAYER-010, FR-LAYER-011
#[test]
fn a_deleted_mountpoint_is_gone() {
  let mut harness = Harness::new("layer-del");
  let path_dir_payload = harness.path_dir_output().join("payload");
  harness.place_script("payload/bin/payload.sh", SCRIPT);
  let path_file_layer = harness.path_dir_output().join("payload.layer");
  let hash = harness.place_layer(&path_dir_payload, &path_file_layer);

  harness.compose().steps = Steps {
    entries: vec![Step {
      builder: None,
      exec: None,
      layer: Some(Layer {
        overlay: None,
        stack: None,
        mount: Some(Mount {
          set: Some(MountSet {
            hash: hash.clone(),
            path: PathBuf::from("/opt/payload"),
          }),
          del: None,
          clear: None,
        }),
      }),
    }],
  };
  harness.write();

  let first = harness.run();
  assert!(first.succeeded(), "{first:?}");

  let mounts = FlatImage::of(&harness.path_file_output())
    .layer()
    .mount()
    .items()
    .expect("fim-layer mount list answers");
  assert!(mounts.iter().any(|entry| entry.hash == hash), "the first build recorded no mountpoint: {mounts:?}");

  harness.compose().image.inplace = true;
  harness.compose().image.from = None;
  harness.compose().steps = Steps {
    entries: vec![Step {
      builder: None,
      exec: None,
      layer: Some(Layer {
        overlay: None,
        stack: None,
        mount: Some(Mount {
          set: None,
          del: Some(MountDel { hash: hash.clone() }),
          clear: None,
        }),
      }),
    }],
  };
  harness.write();

  let second = harness.run();
  assert!(second.succeeded(), "{second:?}");

  let mounts = FlatImage::of(&harness.path_file_output())
    .layer()
    .mount()
    .items()
    .expect("fim-layer mount list answers");
  assert!(mounts.is_empty(), "the deleted mountpoint survived: {mounts:?}");
}

// covers: FR-LAYER-012
#[test]
fn clearing_the_mountpoints_leaves_none() {
  let mut harness = Harness::new("layer-clear");
  let path_dir_payload_a = harness.path_dir_output().join("payload-a");
  let path_dir_payload_b = harness.path_dir_output().join("payload-b");
  harness.place_script("payload-a/bin/payload.sh", "#!/bin/sh\necho payload-a\n");
  harness.place_script("payload-b/bin/payload.sh", "#!/bin/sh\necho payload-b\n");
  let path_file_layer_a = harness.path_dir_output().join("payload-a.layer");
  let path_file_layer_b = harness.path_dir_output().join("payload-b.layer");
  let hash_a = harness.place_layer(&path_dir_payload_a, &path_file_layer_a);
  let hash_b = harness.place_layer(&path_dir_payload_b, &path_file_layer_b);

  harness.compose().steps = Steps {
    entries: vec![
      Step {
        builder: None,
        exec: None,
        layer: Some(Layer {
          overlay: None,
          stack: None,
          mount: Some(Mount {
            set: Some(MountSet {
              hash: hash_a.clone(),
              path: PathBuf::from("/opt/a"),
            }),
            del: None,
            clear: None,
          }),
        }),
      },
      Step {
        builder: None,
        exec: None,
        layer: Some(Layer {
          overlay: None,
          stack: None,
          mount: Some(Mount {
            set: Some(MountSet {
              hash: hash_b.clone(),
              path: PathBuf::from("/opt/b"),
            }),
            del: None,
            clear: None,
          }),
        }),
      },
    ],
  };
  harness.write();

  let first = harness.run();
  assert!(first.succeeded(), "{first:?}");

  let mounts = FlatImage::of(&harness.path_file_output())
    .layer()
    .mount()
    .items()
    .expect("fim-layer mount list answers");
  assert_eq!(mounts.len(), 2, "the first build did not record both mountpoints: {mounts:?}");

  harness.compose().image.inplace = true;
  harness.compose().image.from = None;
  harness.compose().steps = Steps {
    entries: vec![Step {
      builder: None,
      exec: None,
      layer: Some(Layer {
        overlay: None,
        stack: None,
        mount: Some(Mount {
          set: None,
          del: None,
          clear: Some(MountClear {}),
        }),
      }),
    }],
  };
  harness.write();

  let second = harness.run();
  assert!(second.succeeded(), "{second:?}");

  let mounts = FlatImage::of(&harness.path_file_output())
    .layer()
    .mount()
    .items()
    .expect("fim-layer mount list answers");
  assert!(mounts.is_empty(), "the cleared image still records a mountpoint: {mounts:?}");
}

// covers: FR-LAYER-013
#[test]
fn a_layer_step_stating_no_command_is_refused() {
  let mut harness = Harness::new("layer-no-command");
  harness.compose().steps = Steps {
    entries: vec![Step {
      builder: None,
      exec: None,
      layer: Some(Layer {
        overlay: None,
        stack: None,
        mount: None,
      }),
    }],
  };
  harness.write();

  let outcome = harness.run();

  assert!(outcome.refused_with("states no layer command"), "{outcome:?}");
  assert!(!path_file_digest(&harness.path_file_output()).exists());
}

// covers: FR-LAYER-013
#[test]
fn a_layer_step_stating_two_nouns_is_refused() {
  let mut harness = Harness::new("layer-two-nouns");
  harness.compose().steps = Steps {
    entries: vec![Step {
      builder: None,
      exec: None,
      layer: Some(Layer {
        overlay: Some(Overlay {
          save: Some(OverlaySave { from: None, to: None }),
        }),
        stack: Some(Stack {
          add: Some(StackAdd {
            path: PathBuf::from("payload.layer"),
            mount: None,
          }),
        }),
        mount: None,
      }),
    }],
  };
  harness.write();

  let outcome = harness.run();

  assert!(outcome.refused_with("states more than one layer command"), "{outcome:?}");
  assert!(!path_file_digest(&harness.path_file_output()).exists());
}

// covers: FR-LAYER-013
#[test]
fn a_layer_step_stating_two_verbs_is_refused() {
  let mut harness = Harness::new("layer-two-verbs");
  harness.compose().steps = Steps {
    entries: vec![Step {
      builder: None,
      exec: None,
      layer: Some(Layer {
        overlay: None,
        stack: None,
        mount: Some(Mount {
          set: Some(MountSet {
            hash: "9f2b".into(),
            path: PathBuf::from("/opt/payload"),
          }),
          del: Some(MountDel { hash: "9f2b".into() }),
          clear: None,
        }),
      }),
    }],
  };
  harness.write();

  let outcome = harness.run();

  assert!(outcome.refused_with("states more than one layer command"), "{outcome:?}");
  assert!(!path_file_digest(&harness.path_file_output()).exists());
}

// covers: FR-LAYER-005
#[test]
fn a_stack_add_that_names_no_layer_file_is_refused() {
  let harness = Harness::new("layer-add-path-absent");
  let text = format!(
    indoc! {r"
      image:
        from: {}
        output: {}
      steps:
        - layer:
            stack:
              add:
                mount: /opt/payload
    "},
    harness.path_file_from().display(),
    harness.path_file_output().display()
  );
  fs::write(harness.path_file_self(), text).expect("the FlatCompose file is writable");

  let outcome = harness.run();

  assert!(outcome.refused_with("missing field `path`"), "{outcome:?}");
  assert!(!path_file_digest(&harness.path_file_output()).exists());
}

// covers: FR-LAYER-008
#[test]
fn a_mount_set_that_names_no_hash_is_refused() {
  let harness = Harness::new("layer-set-hash-absent");
  let text = format!(
    indoc! {r"
      image:
        from: {}
        output: {}
      steps:
        - layer:
            mount:
              set:
                path: /opt/payload
    "},
    harness.path_file_from().display(),
    harness.path_file_output().display()
  );
  fs::write(harness.path_file_self(), text).expect("the FlatCompose file is writable");

  let outcome = harness.run();

  assert!(outcome.refused_with("missing field `hash`"), "{outcome:?}");
  assert!(!path_file_digest(&harness.path_file_output()).exists());
}

// covers: FR-LAYER-009
#[test]
fn a_mount_set_that_names_no_path_is_refused() {
  let harness = Harness::new("layer-set-path-absent");
  let text = format!(
    indoc! {r"
      image:
        from: {}
        output: {}
      steps:
        - layer:
            mount:
              set:
                hash: 9f2b
    "},
    harness.path_file_from().display(),
    harness.path_file_output().display()
  );
  fs::write(harness.path_file_self(), text).expect("the FlatCompose file is writable");

  let outcome = harness.run();

  assert!(outcome.refused_with("missing field `path`"), "{outcome:?}");
  assert!(!path_file_digest(&harness.path_file_output()).exists());
}

// covers: FR-LAYER-011
#[test]
fn a_mount_del_that_names_no_hash_is_refused() {
  let harness = Harness::new("layer-del-hash-absent");
  let text = format!(
    indoc! {r"
      image:
        from: {}
        output: {}
      steps:
        - layer:
            mount:
              del: {{}}
    "},
    harness.path_file_from().display(),
    harness.path_file_output().display()
  );
  fs::write(harness.path_file_self(), text).expect("the FlatCompose file is writable");

  let outcome = harness.run();

  assert!(outcome.refused_with("missing field `hash`"), "{outcome:?}");
  assert!(!path_file_digest(&harness.path_file_output()).exists());
}

// covers: FR-LAYER-012
#[test]
fn a_mount_clear_that_carries_a_key_is_refused() {
  let harness = Harness::new("layer-clear-key");
  let text = format!(
    indoc! {r"
      image:
        from: {}
        output: {}
      steps:
        - layer:
            mount:
              clear:
                path: /opt/payload
    "},
    harness.path_file_from().display(),
    harness.path_file_output().display()
  );
  fs::write(harness.path_file_self(), text).expect("the FlatCompose file is writable");

  let outcome = harness.run();

  assert!(outcome.refused_with("unknown field `path`"), "{outcome:?}");
  assert!(!path_file_digest(&harness.path_file_output()).exists());
}

// covers: FR-LAYER-004
#[test]
fn a_layer_step_naming_an_unknown_verb_is_refused() {
  let harness = Harness::new("layer-unknown-verb");
  let text = format!(
    indoc! {r"
      image:
        from: {}
        output: {}
      steps:
        - layer:
            stack:
              remove: {{}}
    "},
    harness.path_file_from().display(),
    harness.path_file_output().display()
  );
  fs::write(harness.path_file_self(), text).expect("the FlatCompose file is writable");

  let outcome = harness.run();

  assert!(outcome.refused_with("unknown field `remove`"), "{outcome:?}");
  assert!(!path_file_digest(&harness.path_file_output()).exists());
}
