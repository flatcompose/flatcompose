//! The trait that lowers the YAML into a FlatImage command

use crate::lower::Command;

pub trait Lower {
  fn lower(&self) -> Vec<Command>;
}
