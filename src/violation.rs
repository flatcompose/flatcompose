//! Violations of semantic rules

use std::fmt::{self, Display, Formatter};

/// The rule an entity applies to its own value
pub trait Violation {
  /// Every refusal this entity finds.
  fn violations(&self) -> Vec<Entry>;
}

/// One refusal
pub struct Entry {
  path: String,
  message: String,
}

impl Entry {
  pub fn of(path: impl Into<String>, message: impl Into<String>) -> Entry {
    Entry {
      path: path.into(),
      message: message.into(),
    }
  }
}

impl Display for Entry {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
    write!(formatter, "error: `{}` {}", self.path, self.message)
  }
}

/// Every violation of one file, in the order the calls met them.
#[derive(Default)]
pub struct Violations(Vec<Entry>);

impl Violations {
  pub fn none() -> Violations {
    Violations(Vec::new())
  }

  pub fn push(&mut self, entry: Entry) {
    self.0.push(entry);
  }

  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }
}

impl Display for Violations {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
    for (index, entry) in self.0.iter().enumerate() {
      if index > 0 {
        writeln!(formatter)?;
      }

      write!(formatter, "{entry}")?;
    }

    Ok(())
  }
}
