use std::str;

#[derive(Clone, Debug, PartialEq)]
pub struct Rule {
  pub target: String,
  pub dependencies: Vec<String>,
  pub commands: Vec<String>
}

impl Rule {
  fn new(target: String, dependencies: Vec<String>, commands: Vec<String>) -> Rule {
    Rule {
      target: target,
      dependencies: dependencies,
      commands: commands
    }
  }
}

#[derive(Debug, PartialEq)]
pub struct Makefile {
  pub rules: Vec<Rule>
}

impl Makefile {
  fn new(rules: Vec<Rule>) -> Makefile {
    Makefile { rules: rules }
  }
}

#[derive(Debug)]
enum MakefileLine {
  EmptyLine,
  RuleDefinition(String, Vec<String>),
  Command(String)
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
  LineParse,
  CommandOutsideOfRule
}

fn parse_line(line: &str) -> Result<MakefileLine, ParseError> {
  let original = line;

  // Strip off comments
  let line:&str = line.split_terminator('#').next().unwrap_or("");

  if line.chars().nth(0) == Some('\t') {
    return Ok(MakefileLine::Command(String::from(line.trim())));
  } else if let Some(index) = line.find(':') {
    let (target, dependencies_with_separator) = line.split_at(index);
    let (_, dependencies) = dependencies_with_separator.split_at(1);
    
    return Ok(MakefileLine::RuleDefinition(String::from(target), dependencies.split_whitespace().map(|s| s.to_owned()).collect()));
  } else if line.is_empty() {
    return Ok(MakefileLine::EmptyLine);
  } else {
    eprintln!("Failed to parse line: {:?}", original);
    return Err(ParseError::LineParse);
  }

}

pub fn makefile(input: String) -> Result<Makefile, ParseError> {
  let mut makefile = Makefile::new(Vec::new());
  
  for line in input.lines() {
    match parse_line(line)? {
      MakefileLine::EmptyLine => {},

      MakefileLine::Command(command) => {
        let last_index = makefile.rules.len() - 1;
        if let Some(rule) = makefile.rules.get_mut(last_index) {
          rule.commands.push(String::from(command));
        } else {
          return Err(ParseError::CommandOutsideOfRule)
        }
      }

      MakefileLine::RuleDefinition(target, dependencies) => {
        makefile.rules.push(Rule::new(String::from(target), dependencies, Vec::new()));
      }
    }
  }

  Ok(makefile)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn makefile_empty_test() {
    assert_eq!(makefile("".to_owned()), Ok(Makefile::new(Vec::new())));
  }

  #[test]
  fn makefile_one_rule_test() {
    assert_eq!(makefile("all: build test".to_owned()), Ok(Makefile::new(vec![
      Rule::new("all".to_owned(), vec!["build".to_owned(), "test".to_owned()], vec![])
    ])));
  }

  #[test]
  fn makefile_blank_line_test() {
    assert_eq!(makefile("\nall: build test\n".to_owned()), Ok(Makefile::new(vec![
      Rule::new("all".to_owned(), vec!["build".to_owned(), "test".to_owned()], vec![])
    ])));
  }

  #[test]
  fn makefile_with_multiple_rules_test() {
    assert_eq!(makefile("all: a\n\na:\n\tfoo".to_owned()), Ok(Makefile::new(vec![
      Rule::new("all".to_owned(), vec!["a".to_owned()], vec![]),
      Rule::new("a".to_owned(), vec![], vec!["foo".to_owned()])
    ])));
  }
  


  #[test]
  fn makefile_with_commands_test() {
    assert_eq!(makefile("foo: bar baz\n\tone\n\ttwo\n".to_owned()), Ok(Makefile::new(vec![
      Rule::new("foo".to_owned(), vec!["bar".to_owned(), "baz".to_owned()], vec!["one".to_owned(), "two".to_owned()])
    ])));
  }
}