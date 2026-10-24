//! The `flatcompose` process entry point.

use std::process::ExitCode;

use flatcompose::cli;

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
