//! The `flatcompose` process entry point.

mod cli;
mod compose;
mod internal;
mod lower;
mod violation;

use std::process::ExitCode;

fn main() -> ExitCode {
  match run() {
    Ok(code) => code,
    Err(error) => {
      eprintln!("error: {error:#}");
      ExitCode::FAILURE
    }
  }
}

fn run() -> anyhow::Result<ExitCode> {
  let args = cli::Args::parse()?;
  cli::Executor.execute(args)
}
