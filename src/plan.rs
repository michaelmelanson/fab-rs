use std::collections::HashMap;

use crate::makefile::{Makefile, Rule, Target};

#[derive(Clone, Debug, PartialEq)]
pub struct Invocation {
    pub rule: Rule,
    pub target: Target,
}

pub type Phase = Vec<Invocation>;

#[derive(Debug, PartialEq)]
pub struct Plan {
    pub makefile: Makefile,
    pub phases: Vec<Phase>,
}

///
/// Plans the execution of a `Makefile` to build a `Target`.
///
/// It breaks up the dependency tree into a set of "phases", such that any
/// dependencies of invocations in a phase are resolved in a prior phase.
///
/// For example, if we have a makefile with two rules `a: b` and `b:`, then
/// planning to build `a` will create two phases: 1) build `a`, then 2) build `b`:
///
/// ```
/// use fab::makefile::{parse_makefile, Target, Rule};
/// use fab::plan::{plan_execution, Plan, Invocation};
///
/// let makefile = parse_makefile(&"a: b\nb:\n".to_string()).unwrap();
/// let plan = plan_execution(
///     makefile.clone(),
///     &Target::named("a")
/// );
///
/// assert_eq!(Plan { makefile: makefile.clone(), phases: vec![
///     vec![Invocation { rule: makefile.rules[1].clone(), target: Target::named("b") }],
///     vec![Invocation { rule: makefile.rules[0].clone(), target: Target::named("a") }]
/// ] }, plan);
/// ```
///
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

/// Finds a rule matching the target in a `Makefile`.
///
/// ```
/// # use fab::makefile::{Rule, Makefile, Target};
/// # use fab::plan::{find_rule};
///
/// let compile_c_code_rule = Rule {
///     target: Target::named("main.c"),
///     dependencies: vec![Target::named("main.o")],
///     commands: vec![]
/// };
///
/// let makefile = Makefile { rules: vec![compile_c_code_rule.clone()] };
/// assert_eq!(compile_c_code_rule, find_rule(&makefile, &Target::named("main.c")));
/// ```

pub fn find_rule<'a>(makefile: &Makefile, target: &Target) -> Rule {
    makefile
        .rules
        .iter()
        .find(|r| r.target == *target)
        .unwrap_or_else(|| panic!("No such rule {:?}", target))
        .clone()
}
