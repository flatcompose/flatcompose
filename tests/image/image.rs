//! `FR-IMAGE`: the `image` block, and what the run writes.

use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

use flatcompose::compose::image::{Arch, From};
use flatcompose::internal::hash;
use indoc::indoc;

use crate::common::{FlatImage, Harness, Outcome, digest, path_file_digest};

/// Runs the binary with `args` from `path_dir`.
fn run_at(path_dir: &Path, args: &[&str]) -> Outcome {
  let output = Command::new(env!("CARGO_BIN_EXE_flatcompose"))
    .args(args)
    .current_dir(path_dir)
    .stdin(Stdio::null())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .output()
    .expect("the binary runs");

  Outcome {
    code: output.status.code(),
    stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
    stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
  }
}

/// Writes `text` as the case's FlatCompose file, for the cases about text the
/// `Compose` types cannot hold.
fn write_text(harness: &Harness, text: &str) {
  fs::write(harness.path_file_self(), text).expect("the FlatCompose file is writable");
}

// covers: FR-IMAGE-001
#[test]
fn a_file_carrying_only_the_image_block_builds() {
  let harness = Harness::new("image-basic");

  harness.write();
  let outcome = harness.run();

  assert!(outcome.succeeded(), "{outcome:?}");
  assert!(outcome.silent(), "{outcome:?}");
  assert!(harness.path_file_output().is_file());
  assert!(harness.verifies());
}

// covers: FR-IMAGE-003
#[test]
fn the_output_file_verifies_with_sha256sum() {
  let harness = Harness::new("image-verify");

  harness.write();
  let outcome = harness.run();

  assert!(outcome.succeeded(), "{outcome:?}");
  assert!(harness.verifies(), "sha256sum -c refused the pair the run wrote");
}

// covers: FR-IMAGE-003
#[test]
fn the_digest_line_names_the_digest_and_the_file() {
  let harness = Harness::new("image-digest-line");

  harness.write();
  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");

  let path_file_digest = harness.path_file_self().with_file_name("app.flatimage.sha256sum");
  let text = fs::read_to_string(&path_file_digest).expect("the digest file is readable");
  let line = text.lines().next().expect("the digest file states one line");
  let (digest, file_name) = line
    .split_once("  ")
    .expect("the line states the separator `sha256sum` writes between the parts");

  assert_eq!(digest.len(), 64, "{line:?}");
  assert!(digest.chars().all(|character| character.is_ascii_hexdigit()), "{line:?}");
  assert_eq!(file_name, "app.flatimage", "{line:?}");
}

// covers: FR-IMAGE-003
#[test]
fn the_digest_matches_the_output_bytes() {
  let harness = Harness::new("image-digest-bytes");

  harness.write();
  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");

  let printed = digest(&harness.path_file_output()).expect("the digest beside the output is readable");
  let computed = hash::sha256(&harness.path_file_output()).expect("the output is hashable");

  assert_eq!(printed, computed);
}

// covers: FR-IMAGE-002
#[test]
fn the_file_from_names_stays_as_the_build_found_it() {
  let harness = Harness::new("image-from-untouched");
  let path_file_from = harness.path_file_from();
  let before = hash::sha256(&path_file_from).expect("the base image is hashable");

  harness.write();
  let outcome = harness.run();
  assert!(outcome.succeeded(), "{outcome:?}");

  let after = hash::sha256(&path_file_from).expect("the base image is hashable");
  assert_eq!(before, after);
}

// covers: FR-IMAGE-001
#[test]
fn the_default_file_name_serves_a_build_without_the_flag() {
  let harness = Harness::new("image-default-name");
  harness.write();

  let outcome = run_at(harness.path_dir_output(), &["build"]);

  assert!(outcome.succeeded(), "{outcome:?}");
  assert!(harness.path_file_output().is_file());
}

// covers: FR-IMAGE-002
#[test]
fn a_from_path_resolves_beside_the_flatcompose_file() {
  let mut harness = Harness::new("image-relative-from");
  fs::copy(harness.path_file_from(), harness.path_dir_output().join("core.flatimage"))
    .expect("the base image is copyable into the directory");
  harness.compose().image.from = Some(From::Local("core.flatimage".into()));
  harness.write();

  let path_dir_parent = harness
    .path_dir_output()
    .parent()
    .expect("the directory has a parent")
    .to_path_buf();
  let outcome = run_at(
    &path_dir_parent,
    &[
      "-f",
      harness.path_file_self().to_str().expect("the path is unicode"),
      "build",
    ],
  );

  assert!(outcome.succeeded(), "{outcome:?}");
  assert!(harness.path_file_output().is_file());
}

// covers: FR-IMAGE-002
#[test]
fn without_from_the_build_starts_from_the_file_at_output() {
  use flatcompose::compose::config::Perms;

  let mut harness = Harness::new("image-from-omitted");
  harness.compose().config.perms = Some(Perms {
    entries: vec!["home".into()],
  });
  harness.write();
  let first = harness.run();
  assert!(first.succeeded(), "{first:?}");

  harness.compose().image.from = None;
  harness.compose().config = flatcompose::compose::config::Config::default();
  harness.compose().config.env = Some(flatcompose::compose::config::Env {
    entries: vec![flatcompose::compose::config::Variable {
      name: "LANG".into(),
      value: "C.UTF-8".into(),
    }],
  });
  harness.write();
  let second = harness.run();
  assert!(second.succeeded(), "{second:?}");

  let flatimage = FlatImage::of(&harness.path_file_output());
  assert_eq!(
    flatimage.perms().list().expect("fim-perms list answers"),
    ["home"],
    "the permission of the first run survives"
  );
  assert_eq!(
    flatimage.env().list().expect("fim-env list answers").len(),
    1,
    "the variable of the second run reached the file"
  );
}

// covers: FR-IMAGE-002
#[test]
fn a_url_from_gets_fetched_once() {
  let value = std::env::var("FLATIMAGE").expect("FLATIMAGE names the base image");
  assert!(
    value.starts_with("http://") || value.starts_with("https://"),
    "this case needs FLATIMAGE to hold a URL, and it states {value:?}"
  );

  let mut harness = Harness::new("image-url-from");
  harness.compose().image.from = Some(From::Remote(value));
  harness.write();
  let outcome = harness.run();

  assert!(outcome.succeeded(), "{outcome:?}");
  assert!(harness.path_file_output().is_file());
}

// covers: FR-IMAGE-004
#[test]
fn inplace_false_replaces_what_the_output_held() {
  use flatcompose::compose::config::{Config, Env, Perms, Variable};

  let mut harness = Harness::new("image-inplace-false");
  harness.compose().config.perms = Some(Perms {
    entries: vec!["home".into()],
  });
  harness.write();
  assert!(harness.run().succeeded());

  harness.compose().config = Config {
    env: Some(Env {
      entries: vec![Variable {
        name: "LANG".into(),
        value: "C.UTF-8".into(),
      }],
    }),
    ..Config::default()
  };
  harness.write();
  let second = harness.run();
  assert!(second.succeeded(), "{second:?}");

  let flatimage = FlatImage::of(&harness.path_file_output());
  assert_eq!(flatimage.perms().list().expect("fim-perms list answers"), Vec::<String>::new());
  assert_eq!(flatimage.env().list().expect("fim-env list answers").len(), 1);
}

// covers: FR-IMAGE-004
#[test]
fn inplace_true_keeps_what_the_output_held() {
  use flatcompose::compose::config::{Config, Env, Perms, Variable};

  let mut harness = Harness::new("image-inplace-true");
  harness.compose().config.perms = Some(Perms {
    entries: vec!["home".into()],
  });
  harness.write();
  assert!(harness.run().succeeded());

  harness.compose().image.inplace = true;
  harness.compose().config = Config {
    env: Some(Env {
      entries: vec![Variable {
        name: "LANG".into(),
        value: "C.UTF-8".into(),
      }],
    }),
    ..Config::default()
  };
  harness.write();
  let second = harness.run();
  assert!(second.succeeded(), "{second:?}");

  let flatimage = FlatImage::of(&harness.path_file_output());
  assert_eq!(flatimage.perms().list().expect("fim-perms list answers"), ["home"]);
  assert_eq!(flatimage.env().list().expect("fim-env list answers").len(), 1);
}

// covers: FR-IMAGE-004
#[test]
fn inplace_true_creates_the_output_from_from() {
  let mut harness = Harness::new("image-inplace-absorbs");
  harness.compose().image.inplace = true;
  harness.write();

  let outcome = harness.run();

  assert!(outcome.succeeded(), "{outcome:?}");
  assert!(harness.path_file_output().is_file());
  assert!(harness.verifies());
}

// covers: US1-2
#[test]
fn a_second_build_of_one_file_reports_the_same_settings() {
  use flatcompose::compose::config::Perms;

  let mut harness = Harness::new("image-second-run");
  harness.compose().config.perms = Some(Perms {
    entries: vec!["network".into(), "home".into()],
  });
  harness.write();
  assert!(harness.run().succeeded());

  let first = FlatImage::of(&harness.path_file_output())
    .perms()
    .list()
    .expect("fim-perms list answers");

  harness.write();
  let second = harness.run();
  assert!(second.succeeded(), "{second:?}");

  let again = FlatImage::of(&harness.path_file_output())
    .perms()
    .list()
    .expect("fim-perms list answers");

  assert_eq!(first, again);
  assert!(harness.verifies());
}

// covers: FR-IMAGE-005
#[test]
fn every_arch_token_is_accepted() {
  let mut harness = Harness::new("image-arch-all");
  harness.compose().image.arch = vec![Arch::X86_64, Arch::I686, Arch::AArch64, Arch::Armv7l, Arch::Riscv64];
  harness.write();

  let outcome = harness.run();

  assert!(outcome.succeeded(), "{outcome:?}");
  assert!(harness.path_file_output().is_file());
}

// covers: FR-IMAGE-005
#[test]
fn an_arch_token_outside_the_valid_values_is_refused() {
  let harness = Harness::new("image-arch-unknown");
  write_text(
    &harness,
    &format!(
      indoc! {r"
        image:
          from: {}
          output: {}
          arch: [mips64]
      "},
      harness.path_file_from().display(),
      harness.path_file_output().display()
    ),
  );

  let outcome = harness.run();

  assert!(!outcome.succeeded(), "{outcome:?}");
  assert!(outcome.stdout.is_empty(), "{outcome:?}");
  assert!(outcome.stderr.contains("mips64"), "{outcome:?}");
}

// covers: FR-IMAGE-005
#[test]
fn an_absent_arch_list_builds() {
  let harness = Harness::new("image-arch-absent");

  harness.write();
  let outcome = harness.run();

  assert!(outcome.succeeded(), "{outcome:?}");
  assert!(harness.path_file_output().is_file());
}

// covers: US1-3
#[test]
fn an_unknown_top_level_key_is_refused_naming_it() {
  let harness = Harness::new("image-unknown-top");
  write_text(
    &harness,
    &format!(
      indoc! {r"
        artifacts: []
        image:
          from: {}
          output: {}
      "},
      harness.path_file_from().display(),
      harness.path_file_output().display()
    ),
  );

  let outcome = harness.run();

  assert!(!outcome.succeeded(), "{outcome:?}");
  assert!(outcome.stderr.contains("artifacts"), "{outcome:?}");
  assert!(!harness.path_file_output().is_file());
}

// covers: US1-3
#[test]
fn an_unknown_key_inside_a_block_is_refused_naming_it() {
  let harness_image = Harness::new("image-unknown-inside-image");
  write_text(
    &harness_image,
    &format!(
      indoc! {r"
        image:
          from: {}
          output: {}
          artifacts: []
      "},
      harness_image.path_file_from().display(),
      harness_image.path_file_output().display()
    ),
  );

  let outcome_image = harness_image.run();
  assert!(!outcome_image.succeeded(), "{outcome_image:?}");
  assert!(outcome_image.stderr.contains("artifacts"), "{outcome_image:?}");

  let harness_config = Harness::new("image-unknown-inside-config");
  write_text(
    &harness_config,
    &format!(
      indoc! {r"
        image:
          from: {}
          output: {}
        config:
          artifacts: []
      "},
      harness_config.path_file_from().display(),
      harness_config.path_file_output().display()
    ),
  );

  let outcome_config = harness_config.run();
  assert!(!outcome_config.succeeded(), "{outcome_config:?}");
  assert!(outcome_config.stderr.contains("artifacts"), "{outcome_config:?}");
}

// covers: FR-IMAGE-001
#[test]
fn a_key_the_schema_wants_stays_required() {
  let harness_output = Harness::new("image-missing-output");
  write_text(
    &harness_output,
    &format!(
      indoc! {r"
        image:
          from: {}
      "},
      harness_output.path_file_from().display()
    ),
  );

  let outcome_output = harness_output.run();
  assert!(!outcome_output.succeeded(), "{outcome_output:?}");
  assert!(outcome_output.stderr.contains("output"), "{outcome_output:?}");

  let harness_image = Harness::new("image-missing-image");
  write_text(&harness_image, "config: {}\n");

  let outcome_image = harness_image.run();
  assert!(!outcome_image.succeeded(), "{outcome_image:?}");
  assert!(outcome_image.stderr.contains("image"), "{outcome_image:?}");
}

// covers: FR-IMAGE-001
#[test]
fn a_file_that_does_not_parse_is_refused() {
  let harness = Harness::new("image-broken-yaml");
  write_text(&harness, "image: [this is not a mapping\n");

  let outcome = harness.run();

  assert!(!outcome.succeeded(), "{outcome:?}");
  assert!(outcome.stdout.is_empty(), "{outcome:?}");
}

// covers: FR-IMAGE-001
#[test]
fn a_file_that_is_not_there_is_refused() {
  let harness = Harness::new("image-file-absent");
  let path_file_absent = harness.path_dir_output().join("absent.yml");

  let outcome =
    run_at(harness.path_dir_output(), &["-f", path_file_absent.to_str().expect("the path is unicode"), "build"]);

  assert!(!outcome.succeeded(), "{outcome:?}");
  assert!(outcome.stderr.contains("absent.yml"), "{outcome:?}");
}

// covers: FR-IMAGE-006
#[test]
fn a_build_with_nothing_to_start_from_is_refused() {
  let mut harness = Harness::new("image-no-from");
  let path_file_output = harness.path_file_output();
  harness.compose().image.from = None;
  harness.write();

  let outcome = harness.run();

  let refusal = format!("`image.from` is absent and no file sits at `{}`", path_file_output.display());
  assert!(outcome.refused_with(&refusal), "{outcome:?}");
  assert!(!path_file_output.is_file());
}

// covers: FR-IMAGE-002
#[test]
fn a_from_that_is_not_there_is_refused() {
  let mut harness = Harness::new("image-from-absent");
  harness.compose().image.from = Some(From::Local(harness.path_dir_output().join("absent.flatimage")));
  harness.write();

  let outcome = harness.run();

  assert!(!outcome.succeeded(), "{outcome:?}");
  assert!(outcome.stderr.contains("copying"), "{outcome:?}");
  assert!(!path_file_digest(&harness.path_file_output()).is_file(), "{outcome:?}");
}

// covers: FR-IMAGE-001
#[test]
fn a_bad_value_inside_the_image_block_is_refused() {
  let harness_arch = Harness::new("image-bad-arch");
  write_text(
    &harness_arch,
    &format!(
      indoc! {r"
        image:
          from: {}
          output: {}
          arch: [x86_64, mips64]
      "},
      harness_arch.path_file_from().display(),
      harness_arch.path_file_output().display()
    ),
  );

  let outcome_arch = harness_arch.run();
  assert!(!outcome_arch.succeeded(), "{outcome_arch:?}");
  assert!(outcome_arch.stderr.contains("mips64"), "{outcome_arch:?}");

  let harness_shape = Harness::new("image-bad-shape");
  write_text(&harness_shape, "image: []\n");

  let outcome_shape = harness_shape.run();
  assert!(!outcome_shape.succeeded(), "{outcome_shape:?}");
  assert!(!outcome_shape.stderr.is_empty(), "{outcome_shape:?}");
}
