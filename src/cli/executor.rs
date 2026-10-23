//! Orchestrates the execution of the parsed arguments

use std::process::ExitCode;

use anyhow::Result;

use crate::cli::{Args, Command, build};

pub struct Executor;

impl Executor {
  pub fn execute(&self, args: Args) -> Result<ExitCode> {
    match args.command {
      Command::Build => build::run(&args.path_file_compose),
    }
  }
}
