extern crate clap;

use clap::{Arg, App};

use std::fs::File;
use std::io::prelude::*;
use std::process::{Command, Stdio};
use std::error::Error;

mod makefile;
use makefile::makefile;

fn main() {
    let args = App::new("make")
        .about("Maker of things")
        .arg(Arg::with_name("file")
            .help("Read FILE as a makefile")
            .long("file")
            .short("f")
            .alias("makefile")
            .takes_value(true)
            .default_value("Makefile"))
        .arg(Arg::with_name("target")
            .help("Target to build")
            .default_value("all")
            .index(1))
        .get_matches();

    let file = args.value_of("file").unwrap();
    let target = args.value_of("target").unwrap().to_owned();

    println!("make: Building target '{}' from '{}'", target, file);

    let mut file = File::open(file).unwrap_or_else(|err| panic!("Could not open {:?}: {} (caused by {:?})", file, err.description(), err.cause()));

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("failed to read from file");

    let makefile = makefile(contents).expect("failed to parse makefile");

    let resolved = resolve_dependencies(&makefile, &target);
    for rule in resolved {
        execute_rule(rule);
    }
}

fn resolve_dependencies<'a> (makefile: &'a makefile::Makefile, target: &'a String) -> Vec<&'a makefile::Rule> {
    let mut dependencies = vec![];

    let mut open = vec![target];
    let mut closed = vec![];

    while let Some(name) = open.pop() {
        let rule = find_rule(makefile, &name);

        for dependency in &rule.dependencies {
            if !open.contains(&dependency) && !closed.contains(&dependency) {
                open.insert(0, dependency);
            }
        }

        dependencies.insert(0, rule);
        closed.push(name);
    }

    return dependencies;
}

fn find_rule<'a> (makefile: &'a makefile::Makefile, target: &String) -> &'a makefile::Rule {
    makefile.rules.iter()
        .find(|r| r.target == *target)
        .unwrap_or_else(|| panic!("No such rule {:?}", target))
}

fn execute_rule(rule: &makefile::Rule) {
    for cmd in rule.commands.iter() {
        let cmd = cmd.replace("$@", &rule.target)
                     .replace("$<", &rule.dependencies.join(" "));

        let status = Command::new("sh").arg("-c").arg(cmd)
            .stdout(Stdio::inherit())
            .status().expect("failed to execute");

        if !status.success() {
            println!("Command failed with exit code {}", status);
        }
    }
}