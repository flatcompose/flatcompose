//! The parser arguments

use std::path::PathBuf;

use anyhow::{Result, anyhow};

/// The file path `-f` names, and the verb to run on it.
pub struct Args {
  pub path_file_compose: PathBuf,
  pub command: Command,
}

/// The verb the argv names.
pub enum Command {
  Build,
}

impl Args {
  /// Parses flatcompose.yml into Args
  pub fn parse() -> Result<Args> {
    let mut path_file_compose = PathBuf::from("flatcompose.yml");
    let mut command = None;
    let mut words = std::env::args().skip(1);

    while let Some(word) = words.next() {
      match word.as_str() {
        "-f" | "--file" => {
          let value = words.next().ok_or_else(|| anyhow!("`{word}` expects a file path"))?;
          path_file_compose = PathBuf::from(value);
        }
        "build" => command = Some(Command::Build),
        other => return Err(anyhow!("`{other}` names no verb of flatcompose")),
      }
    }

    let command = command.ok_or_else(|| anyhow!("no verb given; flatcompose runs `build`"))?;

    Ok(Args {
      path_file_compose,
      command,
    })
  }
}
