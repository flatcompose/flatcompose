//! The build command

use std::path::Path;
use std::process::ExitCode;

use anyhow::Result;

use crate::compose::Compose;
use crate::internal::environment::Environment;
use crate::violation::{Violation, Violations};

/// Reports a broken file or a refused one, then builds.
pub fn run(path_file_compose: &Path) -> Result<ExitCode> {
  let mut environment = Environment::from_process();
  let compose = match Compose::parse(path_file_compose, &mut environment) {
    Ok(compose) => compose,
    Err(error) => {
      eprintln!("error: {error}");
      return Ok(ExitCode::FAILURE);
    }
  };

  let mut violations = Violations::none();

  for entry in compose.violations() {
    violations.push(entry);
  }

  if !violations.is_empty() {
    eprintln!("{violations}");
    return Ok(ExitCode::FAILURE);
  }

  compose.build()?;

  Ok(ExitCode::SUCCESS)
}
