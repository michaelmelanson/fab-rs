extern crate clap;

use clap::{App, Arg};

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process::{Command, Stdio};

mod makefile;
mod plan;

use makefile::{parse_makefile, Target};
use plan::{plan_execution, Invocation};

fn main() {
    let args = App::new("fab")
        .about("The fabulous, somewhat Make-compatible, fabricator of things.")
        .arg(
            Arg::with_name("file")
                .help("Read FILE as a makefile")
                .long("file")
                .short("f")
                .alias("makefile")
                .takes_value(true)
                .default_value("Makefile"),
        ).arg(
            Arg::with_name("target")
                .help("Target to build")
                .default_value("all")
                .index(1),
        ).get_matches();

    let file = args.value_of("file").unwrap();
    let target = Target::named(args.value_of("target").unwrap());

    println!("fab: Building target '{}' from '{}'", target.name(), file);

    let mut file = File::open(file).unwrap_or_else(|err| {
        panic!(
            "Could not open {:?}: {} (caused by {:?})",
            file,
            err.description(),
            err.cause()
        )
    });

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("failed to read from file");

    let makefile = parse_makefile(&contents).expect("failed to parse makefile");

    let plan = plan_execution(makefile.clone(), &target);
    println!("fab: Executing plan: {:#?}", plan);
    for phase in plan.phases {
        for invocation in phase {
            execute(&invocation);
        }
    }
}

fn execute(invocation: &Invocation) {
    let target = &invocation.target;
    let rule = &invocation.rule;
    println!("fab: Building target '{}'", target.name());

    for cmd in &invocation.rule.commands {
        let cmd = cmd.text().replace("$@", target.name()).replace(
            "$<",
            &rule
                .dependencies
                .iter()
                .map(|t| t.name().clone())
                .collect::<Vec<String>>()
                .join(" "),
        );

        let status = Command::new("sh")
            .arg("-c")
            .arg(cmd.clone())
            .stdout(Stdio::inherit())
            .status()
            .expect("failed to execute");

        if !status.success() {
            println!(
                "fab: Target '{}' failed to execute {:?}",
                target.name(),
                cmd
            );
            std::process::exit(1);
        }
    }
    println!("fab: Finished rule '{}'", rule.target.name());
}
