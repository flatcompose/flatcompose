//! The test harness

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU32, Ordering};

use anyhow::{Context, Result};
use flatcompose::compose::Compose;
use flatcompose::compose::config::Config;
use flatcompose::compose::image::{From, Image};
use flatcompose::compose::steps::Steps;

pub use crate::flatimage::FlatImage;

/// The directory every test works under, relative to the home directory.
const DIR_TESTS: &str = "flatcompose/tests";

/// The name the harness gives the FlatCompose file it writes.
const NAME_FILE_COMPOSE: &str = "flatcompose.yml";

/// The name the harness gives the image the build writes.
const NAME_FILE_OUTPUT: &str = "app.flatimage";

/// The name the base image takes when `FLATIMAGE` holds a URL.
const NAME_FILE_BASE: &str = "core-x86_64.flatimage";

static SERIAL: AtomicU32 = AtomicU32::new(0);

/// A name no other test in this process holds.
fn unique(label: &str) -> String {
  let serial = SERIAL.fetch_add(1, Ordering::Relaxed);
  format!("{label}-{}-{serial}", std::process::id())
}

/// The suite writes under the home directory, so it runs inside a container only.
fn require_container() {
  assert!(Path::new("/.dockerenv").exists(), "the suite must run inside a container. See tests/AGENTS.md");
}

/// One test's work: the directory it owns and the FlatCompose file it builds.
///
/// Every file the case creates stands inside that directory, and nothing the
/// suite creates stands anywhere else.
pub struct Harness {
  path_dir: PathBuf,
  compose: Compose,
}

impl Harness {
  /// The harness for one test, with the directory created and the FlatCompose
  /// file holding `image.from` and `image.output` and nothing else.
  ///
  /// `image.from` comes from `FLATIMAGE`: a path is used as given,
  /// a URL is fetched into the directory first, and a missing value fails the
  /// test instead of skipping it.
  pub fn new(label: &str) -> Harness {
    require_container();

    let path_dir_home = std::env::var("HOME").expect("HOME names the home directory");
    let path_dir = PathBuf::from(path_dir_home).join(DIR_TESTS).join(unique(label));
    fs::create_dir_all(&path_dir).unwrap_or_else(|error| panic!("creating {}: {error}", path_dir.display()));

    let path_file_from = path_file_base(&path_dir);
    let compose = Compose {
      path_file_compose: path_dir.join(NAME_FILE_COMPOSE),
      image: Image {
        from: Some(From::Local(path_file_from)),
        output: path_dir.join(NAME_FILE_OUTPUT),
        inplace: false,
        arch: Vec::new(),
      },
      steps: Steps::default(),
      config: Config::default(),
    };

    Harness { path_dir, compose }
  }

  /// The `Compose` a case sets its keys on.
  pub fn compose(&mut self) -> &mut Compose {
    &mut self.compose
  }

  /// The directory holding `image.output`, which is the directory of one test.
  pub fn path_dir_output(&self) -> &Path {
    &self.path_dir
  }

  /// The FlatCompose file the case writes.
  pub fn path_file_self(&self) -> PathBuf {
    self.compose.path_file_compose.clone()
  }

  /// Writes the FlatCompose file to `path_file_self` and returns that path.
  pub fn write(&self) -> PathBuf {
    let text = serde_yaml::to_string(&self.compose).expect("the FlatCompose file serializes");
    fs::write(&self.compose.path_file_compose, text)
      .unwrap_or_else(|error| panic!("writing {}: {error}", self.compose.path_file_compose.display()));
    self.compose.path_file_compose.clone()
  }

  /// Runs `flatcompose -f <dir>/flatcompose.yml build` in the directory, on
  /// the file `write` left there.
  pub fn run(&self) -> Outcome {
    let path_file_compose = self.path_file_self();
    let output = Command::new(env!("CARGO_BIN_EXE_flatcompose"))
      .arg("-f")
      .arg(&path_file_compose)
      .arg("build")
      .current_dir(&self.path_dir)
      .stdin(Stdio::null())
      .stdout(Stdio::piped())
      .stderr(Stdio::piped())
      .output()
      .unwrap_or_else(|error| panic!("running {}: {error}", env!("CARGO_BIN_EXE_flatcompose")));

    Outcome {
      code: output.status.code(),
      stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
      stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    }
  }

  /// The file `image.output` states.
  pub fn path_file_output(&self) -> PathBuf {
    self.compose.image.output.clone()
  }

  /// The file `image.from` states.
  pub fn path_file_from(&self) -> PathBuf {
    match &self.compose.image.from {
      Some(From::Local(path_file)) => path_file.clone(),
      Some(From::Remote(url)) => panic!("`new` leaves a local path in `image.from`, and this one states {url}"),
      None => panic!("`new` fills `image.from`, and the FlatCompose file states none"),
    }
  }

  /// Runs `sha256sum -c app.flatimage.sha256sum` from the directory, the check
  /// `FR-IMAGE-003` fixes for the file `image.output` states.
  pub fn verifies(&self) -> bool {
    let name_file_digest = format!(
      "{}.sha256sum",
      self
        .path_file_output()
        .file_name()
        .expect("the output file has a name")
        .to_string_lossy()
    );

    Command::new("sha256sum")
      .arg("-c")
      .arg(name_file_digest)
      .current_dir(&self.path_dir)
      .stdin(Stdio::null())
      .stdout(Stdio::null())
      .stderr(Stdio::null())
      .status()
      .map(|status| status.success())
      .unwrap_or(false)
  }
}

impl Drop for Harness {
  fn drop(&mut self) {
    let _ = fs::remove_dir_all(&self.path_dir);
  }
}

/// The base image inside `path_dir`, fetched there when the variable holds a URL.
fn path_file_base(path_dir: &Path) -> PathBuf {
  let value = std::env::var("FLATIMAGE").expect("FLATIMAGE names the base image. See docker/.env.tests");

  if !value.starts_with("http://") && !value.starts_with("https://") {
    return PathBuf::from(value);
  }

  let path_file = path_dir.join(NAME_FILE_BASE);
  flatcompose::internal::http::fetch(&value, &path_file).unwrap_or_else(|error| panic!("fetching {value}: {error:#}"));
  path_file
}

/// One run of the binary: how it ended and what it printed.
pub struct Outcome {
  pub stdout: String,
  pub stderr: String,
  pub code: Option<i32>,
}

impl Outcome {
  /// Exits zero, which a build that wrote its file does.
  pub fn succeeded(&self) -> bool {
    self.code == Some(0)
  }

  /// Exits zero with stdout and stderr empty, the success shape `FR-IMAGE-003` fixes.
  pub fn silent(&self) -> bool {
    self.succeeded() && self.stdout.is_empty() && self.stderr.is_empty()
  }

  /// Exits non-zero, prints `needle` on stderr, and prints nothing on stdout.
  pub fn refused_with(&self, needle: &str) -> bool {
    !self.succeeded() && self.stdout.is_empty() && self.stderr.contains(needle)
  }
}

impl std::fmt::Debug for Outcome {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(formatter, "code {:?}, stdout {:?}, stderr {:?}", self.code, self.stdout, self.stderr)
  }
}

/// The digest `sha256sum -c` reads for the file at `path_file`, when one stands beside it.
pub fn digest(path_file: &Path) -> Result<String> {
  let path_file_digest = path_file_digest(path_file);
  let text =
    fs::read_to_string(&path_file_digest).with_context(|| format!("reading {}", path_file_digest.display()))?;
  let line = text.lines().next().unwrap_or_default();
  let digest = line
    .split_whitespace()
    .next()
    .ok_or_else(|| anyhow::anyhow!("{} states no digest", path_file_digest.display()))?;
  Ok(digest.to_string())
}

/// The digest file that stands beside the file at `path_file`, written or not.
pub fn path_file_digest(path_file: &Path) -> PathBuf {
  PathBuf::from(format!("{}.sha256sum", path_file.display()))
}
