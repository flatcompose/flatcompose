//! The argv of one `fim-*` command

use std::ffi::OsString;

/// One `fim-*` command as argv
pub struct Command(Vec<OsString>);

impl Command {
  pub fn new(argv: Vec<OsString>) -> Command {
    Command(argv)
  }

  pub fn argv(&self) -> &[OsString] {
    &self.0
  }
}
