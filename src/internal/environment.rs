//! The build environment, and the references a document takes from it

use std::collections::{HashMap, HashSet};

use anyhow::{Result, anyhow};
use serde_yaml::{Mapping, Value};
use xpanda::Xpanda;

/// The build environment, one variable per entry.
pub struct Environment {
  variables: Vec<Variable>,
  lenient: Xpanda,
  warned: HashSet<String>,
}

/// One variable the build environment holds.
pub struct Variable {
  pub name: String,
  pub value: String,
}

impl Environment {
  /// The process environment, read once for one FlatCompose file.
  pub fn from_process() -> Environment {
    let variables = std::env::vars_os()
      .filter_map(|(name, value)| {
        Some(Variable {
          name: name.into_string().ok()?,
          value: value.into_string().ok()?,
        })
      })
      .collect();

    Environment::with_variables(variables)
  }

  /// The environment built from `variables`, which the tests supply.
  pub(crate) fn with_variables(variables: Vec<Variable>) -> Environment {
    let lenient = Xpanda::builder().with_named_vars(Environment::vars(&variables)).build();

    Environment {
      variables,
      lenient,
      warned: HashSet::new(),
    }
  }

  /// The variables as the expansion library reads them.
  fn vars(variables: &[Variable]) -> HashMap<String, String> {
    variables
      .iter()
      .map(|variable| (variable.name.clone(), variable.value.clone()))
      .collect()
  }

  /// Resolves the references in `text` once, warning about a variable the environment leaves unset.
  pub fn resolve(&mut self, text: &str) -> Result<String> {
    self.warn_unset(text);

    self
      .lenient
      .expand(text)
      .map_err(|error| anyhow!("resolving environment references: {error:?}"))
  }

  /// Warns once per variable the environment leaves unset, using the library's strict evaluation to name it.
  fn warn_unset(&mut self, text: &str) {
    let mut vars = Environment::vars(&self.variables);

    loop {
      let strict = Xpanda::builder().with_named_vars(vars.clone()).no_unset(true).build();

      let Err(error) = strict.expand(text) else {
        return;
      };

      let Some(name) = unset_name(&error.message) else {
        return;
      };

      if vars.contains_key(&name) {
        return;
      }

      self.warn_name(&name);
      vars.insert(name, String::new());
    }
  }

  /// Warns about `name` when the environment leaves it unset, once per file.
  fn warn_name(&mut self, name: &str) {
    if !self.variables.iter().any(|variable| variable.name == name) && self.warned.insert(name.to_owned()) {
      eprintln!("warning: environment variable `{name}` is unset; substituting an empty value");
    }
  }
}

/// The paths a walk leaves alone.
pub struct Exclusion {
  patterns: Vec<Vec<Pattern>>,
}

/// One step of an excluded path: a key, or any index.
enum Pattern {
  Key(String),
  Index,
}

impl Exclusion {
  /// The paths a walk leaves alone, one path per item. `*` matches any index.
  pub fn of(patterns: &[&[&str]]) -> Exclusion {
    let patterns = patterns
      .iter()
      .map(|pattern| {
        pattern
          .iter()
          .map(|step| match *step {
            "*" => Pattern::Index,
            key => Pattern::Key(key.to_owned()),
          })
          .collect()
      })
      .collect();

    Exclusion { patterns }
  }

  /// Whether `path` matches one of the excluded paths.
  fn matches(&self, path: &[Segment]) -> bool {
    self.patterns.iter().any(|pattern| {
      pattern.len() == path.len()
        && pattern.iter().zip(path).all(|(step, segment)| match (step, segment) {
          (Pattern::Key(key), Segment::Key(name)) => key == name,
          (Pattern::Index, Segment::Index(_)) => true,
          _ => false,
        })
    })
  }
}

/// One pass that resolves the references in a value tree.
pub struct Resolve<'a> {
  environment: &'a mut Environment,
  exclusion: &'a Exclusion,
  path: Vec<Segment>,
}

/// One step of the path the walk stands on.
#[derive(Clone)]
enum Segment {
  Key(String),
  Index(usize),
}

impl<'a> Resolve<'a> {
  /// The pass that resolves the references in a value tree, apart from the excluded paths.
  pub fn new(environment: &'a mut Environment, exclusion: &'a Exclusion) -> Resolve<'a> {
    Resolve {
      environment,
      exclusion,
      path: Vec::new(),
    }
  }

  /// Resolves the references in `value` once.
  pub fn value(&mut self, value: Value) -> Result<Value> {
    match value {
      Value::String(text) => {
        if self.exclusion.matches(&self.path) {
          Ok(Value::String(text))
        } else {
          Ok(Value::String(self.environment.resolve(&text)?))
        }
      }
      Value::Sequence(items) => {
        let mut resolved = Vec::with_capacity(items.len());

        for (index, item) in items.into_iter().enumerate() {
          self.path.push(Segment::Index(index));
          resolved.push(self.value(item)?);
          self.path.pop();
        }

        Ok(Value::Sequence(resolved))
      }
      Value::Mapping(mapping) => self.mapping(mapping).map(Value::Mapping),
      value => Ok(value),
    }
  }

  /// Resolves the keys and values of `mapping`.
  fn mapping(&mut self, mapping: Mapping) -> Result<Mapping> {
    let mut resolved = Mapping::new();

    for (key, value) in mapping {
      let key = match key {
        Value::String(text) => Value::String(self.environment.resolve(&text)?),
        key => key,
      };
      let segment = key.as_str().map(|name| Segment::Key(name.to_owned()));

      if let Some(segment) = &segment {
        self.path.push(segment.clone());
      }

      let value = self.value(value)?;

      if segment.is_some() {
        self.path.pop();
      }

      resolved.insert(key, value);
    }

    Ok(resolved)
  }
}

/// The variable name an `xpanda` message names, which reads `'NAME' is unset`.
fn unset_name(message: &str) -> Option<String> {
  let start = message.find('\'')? + 1;
  let end = message[start..].find('\'')? + start;
  let name = &message[start..end];

  if name.chars().next().is_some_and(|character| character.is_ascii_digit()) {
    return None;
  }

  Some(name.to_owned())
}

#[cfg(test)]
mod tests {
  use serde_yaml::Value;

  use super::{Environment, Exclusion, Resolve, Variable, unset_name};

  fn environment(vars: &[(&str, &str)]) -> Environment {
    let variables = vars
      .iter()
      .map(|(name, value)| Variable {
        name: (*name).to_owned(),
        value: (*value).to_owned(),
      })
      .collect();

    Environment::with_variables(variables)
  }

  #[test]
  fn a_reference_takes_the_environment_value() {
    let mut environment = environment(&[("EDITOR", "vim")]);

    assert_eq!(environment.resolve("$EDITOR").unwrap(), "vim");
    assert_eq!(environment.resolve("${EDITOR}").unwrap(), "vim");
  }

  #[test]
  fn a_default_fills_an_unset_or_empty_variable() {
    let mut unset = environment(&[]);
    let mut empty = environment(&[("GREETING", "")]);
    let mut set = environment(&[("GREETING", "bye")]);

    assert_eq!(unset.resolve("${GREETING:-hello}").unwrap(), "hello");
    assert_eq!(empty.resolve("${GREETING:-hello}").unwrap(), "hello");
    assert_eq!(set.resolve("${GREETING:-hello}").unwrap(), "bye");
  }

  #[test]
  fn a_default_without_a_colon_keeps_an_empty_value() {
    let mut unset = environment(&[]);
    let mut empty = environment(&[("GREETING", "")]);

    assert_eq!(unset.resolve("${GREETING-hello}").unwrap(), "hello");
    assert_eq!(empty.resolve("${GREETING-hello}").unwrap(), "");
  }

  #[test]
  fn an_unset_variable_with_no_default_empties() {
    let mut environment = environment(&[]);

    assert_eq!(environment.resolve("$MISSING").unwrap(), "");
    assert_eq!(environment.resolve("${MISSING}").unwrap(), "");
  }

  #[test]
  fn a_double_dollar_writes_one_dollar() {
    let mut environment = environment(&[("PATH", "should-not-appear")]);

    assert_eq!(environment.resolve("$$PATH").unwrap(), "$PATH");
  }

  #[test]
  fn a_value_from_the_environment_resolves_once() {
    let mut environment = environment(&[("VALUE", "${HOME}")]);

    assert_eq!(environment.resolve("$VALUE").unwrap(), "${HOME}");
  }

  #[test]
  fn an_unset_variable_warns_once() {
    let mut environment = environment(&[]);

    environment.resolve("$MISSING").unwrap();
    environment.resolve("${MISSING}").unwrap();

    assert!(environment.warned.contains("MISSING"));
    assert_eq!(environment.warned.len(), 1);
  }

  #[test]
  fn the_walk_resolves_values_and_keys() {
    let mut environment = environment(&[("APP", "app.flatimage"), ("NAME", "LANG"), ("VALUE", "C.UTF-8")]);
    let exclusion = Exclusion::of(&[]);
    let value: Value = serde_yaml::from_str("image:\n  output: $APP\nconfig:\n  env:\n    $NAME: $VALUE\n").unwrap();

    let value = Resolve::new(&mut environment, &exclusion).value(value).unwrap();

    assert_eq!(value["image"]["output"].as_str(), Some("app.flatimage"));
    assert_eq!(value["config"]["env"]["LANG"].as_str(), Some("C.UTF-8"));
  }

  #[test]
  fn an_excluded_path_is_left_as_written() {
    let mut environment = environment(&[("SOURCE", "debian:bookworm")]);
    let exclusion = Exclusion::of(&[&["steps", "*", "exec", "script"]]);
    let value: Value =
      serde_yaml::from_str("steps:\n  - exec:\n      from: $SOURCE\n      script: |\n        echo $HOME\n").unwrap();

    let value = Resolve::new(&mut environment, &exclusion).value(value).unwrap();

    assert_eq!(value["steps"][0]["exec"]["from"].as_str(), Some("debian:bookworm"));
    assert_eq!(value["steps"][0]["exec"]["script"].as_str(), Some("echo $HOME\n"));
  }

  #[test]
  fn unset_name_reads_the_variable_out_of_the_message() {
    assert_eq!(unset_name("'FOO' is unset"), Some("FOO".to_owned()));
    assert_eq!(unset_name("'FOO' is unset or empty"), Some("FOO".to_owned()));
    assert_eq!(unset_name("'5' is unset"), None);
    assert_eq!(unset_name("something else"), None);
  }
}
