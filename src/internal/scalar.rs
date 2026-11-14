//! Scalars that arrive as text after a reference resolves

use std::fmt;

use serde::de::{self, Deserializer, Visitor};

/// Reads a boolean a reference resolved to text.
pub fn boolean<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
  D: Deserializer<'de>,
{
  struct Boolean;

  impl<'de> Visitor<'de> for Boolean {
    type Value = bool;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      formatter.write_str("a boolean or a string holding one")
    }

    fn visit_bool<E: de::Error>(self, value: bool) -> Result<bool, E> {
      Ok(value)
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<bool, E> {
      value.parse().map_err(E::custom)
    }
  }

  deserializer.deserialize_any(Boolean)
}

/// Reads a count a reference resolved to text, or no count.
pub fn optional_u32<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
  D: Deserializer<'de>,
{
  struct Optional;

  impl<'de> Visitor<'de> for Optional {
    type Value = Option<u32>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      formatter.write_str("a count or a string holding one")
    }

    fn visit_none<E: de::Error>(self) -> Result<Option<u32>, E> {
      Ok(None)
    }

    fn visit_unit<E: de::Error>(self) -> Result<Option<u32>, E> {
      Ok(None)
    }

    fn visit_some<D: Deserializer<'de>>(self, deserializer: D) -> Result<Option<u32>, D::Error> {
      deserializer.deserialize_any(Count).map(Some)
    }
  }

  struct Count;

  impl<'de> Visitor<'de> for Count {
    type Value = u32;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      formatter.write_str("a count or a string holding one")
    }

    fn visit_u64<E: de::Error>(self, value: u64) -> Result<u32, E> {
      u32::try_from(value).map_err(E::custom)
    }

    fn visit_i64<E: de::Error>(self, value: i64) -> Result<u32, E> {
      u32::try_from(value).map_err(E::custom)
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<u32, E> {
      value.parse().map_err(E::custom)
    }
  }

  deserializer.deserialize_option(Optional)
}
