use std::collections::HashMap;

use makefile::{Makefile, Rule, Target};

#[derive(Clone, Debug)]
pub struct Invocation {
    pub rule: Rule,
    pub target: Target,
}

pub type Phase = Vec<Invocation>;

pub struct Plan {
    pub makefile: Makefile,
    pub phases: Vec<Phase>,
}

pub fn plan_execution(makefile: Makefile, target: &Target) -> Plan {
    let mut rule_for_target = HashMap::new();
    let mut rank_for_target: HashMap<Target, u32> = HashMap::new();
    let mut max_rank = 0;

    let mut open: Vec<Target> = Vec::new();
    let mut closed: Vec<Target> = Vec::new();

    open.push(target.clone());

    while let Some(name) = open.pop() {
        let rule = rule_for_target
            .entry(name.clone())
            .or_insert_with(|| find_rule(&makefile, &name).clone());

        let mut rank = 0;
        let mut missing_dependencies: Vec<Target> = Vec::new();

        for dependency in &rule.dependencies {
            if closed.contains(&dependency) {
                rank = u32::max(rank, &rank_for_target[&dependency] + 1);
                max_rank = u32::max(max_rank, rank);
            } else {
                missing_dependencies.push(dependency.clone());
            }
        }

        if missing_dependencies.is_empty() {
            rank_for_target.insert(name.clone(), rank);
            closed.push(name.clone());
        } else {
            open.push(name.clone());
            for dep in missing_dependencies {
                if let Ok(index) = open.binary_search(&&dep) {
                    open.remove(index);
                }

                open.push(dep.clone());
            }
        }
    }

    let mut phases: Vec<Phase> = Vec::new();
    phases.resize((max_rank as usize) + 1, Phase::new());

    for (target, rank) in &rank_for_target {
        let rule = rule_for_target[target].clone();

        phases[*rank as usize].push(Invocation {
            rule: rule.clone(),
            target: target.clone(),
        });
    }

    Plan { makefile, phases }
}

fn find_rule<'a>(makefile: &'a Makefile, target: &Target) -> &'a Rule {
    makefile
        .rules
        .iter()
        .find(|r| r.target == *target)
        .unwrap_or_else(|| panic!("No such rule {:?}", target))
}
