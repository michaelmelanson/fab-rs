#![feature(vec_resize_default)]

extern crate clap;

use clap::{Arg, App};

use std::fs::File;
use std::io::prelude::*;
use std::process::{Command, Stdio};
use std::error::Error;

mod makefile;
mod plan;

use makefile::makefile;
use plan::{plan_execution, Invocation};

fn main() {
    let args = App::new("fab")
        .about("The fabulous, somewhat Make-compatible, fabricator of things.")
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

    let plan = plan_execution(&makefile, &target);
    for phase in plan.phases {
        for invocation in phase {
            execute(&invocation);
        }
    }
}

fn execute(invocation: &Invocation) {
    let target = invocation.target;
    let rule = invocation.rule;
    println!("make: Building target '{}'", target);

    for cmd in invocation.rule.commands.iter() {
        let cmd = cmd.replace("$@", &target)
                     .replace("$<", &rule.dependencies.join(" "));

        println!("make: Running command '{}'", cmd);
        let status = Command::new("sh").arg("-c").arg(cmd.clone())
            .stdout(Stdio::inherit())
            .status().expect("failed to execute");
        println!("make: Command completed '{}'", cmd);

        if !status.success() {
            println!("make: Rule '{}' failed", rule.target);
            std::process::exit(1);
        }
    }
    println!("make: Finished rule '{}'", rule.target);

}