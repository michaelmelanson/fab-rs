#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Target(String);

impl Target {
    pub fn named<T: Into<String>>(name: T) -> Target {
        Target(name.into())
    }

    pub fn name(&self) -> &String {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Command(String);

impl Command {
    pub fn with<T: Into<String>>(command: T) -> Command {
        Command(command.into())
    }

    pub fn text(&self) -> &String {
        &self.0
    }
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

fn parse_line(original: &String) -> Result<MakefileLine, ParseError> {
    // Strip off comments
    let line: String = original.split('#').next().unwrap_or("").to_owned().clone();

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

pub fn parse_makefile(input: &String) -> Result<Makefile, ParseError> {
    let mut makefile = Makefile::new(Vec::new());

    for line in input.lines() {
        match parse_line(&line.to_owned())? {
            MakefileLine::EmptyLine => {}

            MakefileLine::Command(command) => {
                let last_index = makefile.rules.len() - 1;
                if let Some(rule) = makefile.rules.get_mut(last_index) {
                    if !command.is_empty() {
                        rule.commands.push(Command::with(command));
                    }
                } else {
                    return Err(ParseError::CommandOutsideOfRule);
                }
            }

            MakefileLine::RuleDefinition(target, dependencies) => {
                makefile.rules.push(Rule::new(
                    Target::named(target),
                    dependencies
                        .iter()
                        .map(|dep| Target::named(dep.clone()))
                        .collect::<Vec<Target>>(),
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
        assert_eq!(
            parse_makefile(&"".to_owned()),
            Ok(Makefile::new(Vec::new()))
        );
    }

    #[test]
    fn makefile_one_rule_test() {
        assert_eq!(
            parse_makefile(&"all: build test".to_owned()),
            Ok(Makefile::new(vec![Rule::new(
                Target::named("all"),
                vec![Target::named("build"), Target::named("test")],
                vec![]
            )]))
        );
    }

    #[test]
    fn makefile_blank_line_test() {
        assert_eq!(
            parse_makefile(&"\nall: build test\n".to_owned()),
            Ok(Makefile::new(vec![Rule::new(
                Target::named("all"),
                vec![Target::named("build"), Target::named("test")],
                vec![]
            )]))
        );
    }

    #[test]
    fn makefile_with_multiple_rules_test() {
        assert_eq!(
            parse_makefile(&"all: a\n\na:\n\tfoo".to_owned()),
            Ok(Makefile::new(vec![
                Rule::new(Target::named("all"), vec![Target::named("a")], vec![]),
                Rule::new(Target::named("a"), vec![], vec![Command::with("foo")])
            ]))
        );
    }

    #[test]
    fn makefile_with_commands_test() {
        assert_eq!(
            parse_makefile(&"foo: bar baz\n\tone\n\ttwo\n".to_owned()),
            Ok(Makefile::new(vec![Rule::new(
                Target::named("foo"),
                vec![Target::named("bar"), Target::named("baz")],
                vec![Command::with("one"), Command::with("two")]
            )]))
        );
    }

    #[test]
    fn makefile_commands_with_blank_lines_test() {
        assert_eq!(
            parse_makefile(&"foo:\n\tone\n\n\ttwo\n\t\n\tthree\n".to_owned()),
            Ok(Makefile::new(vec![Rule::new(
                Target::named("foo"),
                vec![],
                vec![
                    Command::with("one"),
                    Command::with("two"),
                    Command::with("three")
                ]
            )]))
        );
    }
}
