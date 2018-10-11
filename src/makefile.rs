use std::str;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Target {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Command {
    pub rule: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Rule {
    pub target: Target,
    pub dependencies: Vec<Target>,
    pub commands: Vec<Command>,
}

impl Rule {
    fn new(target: Target, dependencies: Vec<Target>, commands: Vec<Command>) -> Rule {
        Rule {
            target,
            dependencies,
            commands,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Makefile {
    pub rules: Vec<Rule>,
}

impl Makefile {
    fn new(rules: Vec<Rule>) -> Makefile {
        Makefile { rules }
    }
}

#[derive(Debug)]
enum MakefileLine {
    EmptyLine,
    RuleDefinition(String, Vec<String>),
    Command(String),
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    LineParse,
    CommandOutsideOfRule,
}

fn parse_line(line: &str) -> Result<MakefileLine, ParseError> {
    let original = line;

    // Strip off comments
    let line: &str = line.split_terminator('#').next().unwrap_or("");

    if line.chars().nth(0) == Some('\t') {
        Ok(MakefileLine::Command(String::from(line.trim())))
    } else if let Some(index) = line.find(':') {
        let (target, dependencies_with_separator) = line.split_at(index);
        let (_, dependencies) = dependencies_with_separator.split_at(1);

        Ok(MakefileLine::RuleDefinition(
            String::from(target),
            dependencies
                .split_whitespace()
                .map(|s| s.to_owned())
                .collect(),
        ))
    } else if line.is_empty() {
        Ok(MakefileLine::EmptyLine)
    } else {
        eprintln!("Failed to parse line: {:?}", original);
        Err(ParseError::LineParse)
    }
}

pub fn makefile(input: &str) -> Result<Makefile, ParseError> {
    let mut makefile = Makefile::new(Vec::new());

    for line in input.lines() {
        match parse_line(line)? {
            MakefileLine::EmptyLine => {}

            MakefileLine::Command(command) => {
                let last_index = makefile.rules.len() - 1;
                if let Some(rule) = makefile.rules.get_mut(last_index) {
                    if !command.is_empty() {
                        rule.commands.push(Command { rule: command });
                    }
                } else {
                    return Err(ParseError::CommandOutsideOfRule);
                }
            }

            MakefileLine::RuleDefinition(target, dependencies) => {
                makefile.rules.push(Rule::new(
                    Target { name: target },
                    dependencies
                        .iter()
                        .map(|dep| Target {
                            name: dep.to_string(),
                        }).collect::<Vec<Target>>(),
                    Vec::new(),
                ));
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
        assert_eq!(
            makefile("all: build test".to_owned()),
            Ok(Makefile::new(vec![Rule::new(
                "all".to_owned(),
                vec!["build".to_owned(), "test".to_owned()],
                vec![]
            )]))
        );
    }

    #[test]
    fn makefile_blank_line_test() {
        assert_eq!(
            makefile("\nall: build test\n".to_owned()),
            Ok(Makefile::new(vec![Rule::new(
                "all".to_owned(),
                vec!["build".to_owned(), "test".to_owned()],
                vec![]
            )]))
        );
    }

    #[test]
    fn makefile_with_multiple_rules_test() {
        assert_eq!(
            makefile("all: a\n\na:\n\tfoo".to_owned()),
            Ok(Makefile::new(vec![
                Rule::new("all".to_owned(), vec!["a".to_owned()], vec![]),
                Rule::new("a".to_owned(), vec![], vec!["foo".to_owned()])
            ]))
        );
    }

    #[test]
    fn makefile_with_commands_test() {
        assert_eq!(
            makefile("foo: bar baz\n\tone\n\ttwo\n".to_owned()),
            Ok(Makefile::new(vec![Rule::new(
                "foo".to_owned(),
                vec!["bar".to_owned(), "baz".to_owned()],
                vec!["one".to_owned(), "two".to_owned()]
            )]))
        );
    }

    #[test]
    fn makefile_commands_with_blank_lines_test() {
        assert_eq!(
            makefile("foo:\n\tone\n\n\ttwo\n\t\n\tthree\n".to_owned()),
            Ok(Makefile::new(vec![Rule::new(
                "foo".to_owned(),
                vec![],
                vec!["one".to_owned(), "two".to_owned(), "three".to_owned()]
            )]))
        );
    }
}
