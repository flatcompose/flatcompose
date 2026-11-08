//! The trait that lowers the YAML into a FlatImage command

use anyhow::Result;

use crate::lower::Command;

pub trait Lower {
  fn lower(&self) -> Result<Vec<Command>>;
}
