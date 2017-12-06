use std::collections::HashMap;

use makefile::{Makefile, Rule, Target};

#[derive(Clone)]
pub struct Invocation<'a> {
  pub rule: &'a Rule,
  pub target: &'a Target
}

pub type Phase<'a> = Vec<Invocation<'a>>;

pub struct Plan<'a> {
  pub makefile: &'a Makefile,
  pub phases: Vec<Phase<'a>>
}

pub fn plan_execution<'a> (makefile: &'a Makefile, target: &'a Target) -> Plan<'a> {
  let mut rule_for_target = HashMap::new();
  let mut rank_for_target: HashMap<&'a Target, u32> = HashMap::new();
  let mut max_rank = 0;

  let mut open = Vec::new();
  let mut closed = Vec::new();

  open.push(target);

  while let Some(name) = open.pop() {
    let rule = rule_for_target.entry(name)
      .or_insert_with(|| find_rule(makefile, &name));

    let mut rank = 0;
    let mut missing_dependencies:Vec<&'a Target> = Vec::new();

    for dependency in &rule.dependencies {
      if closed.contains(&dependency) {
        rank = u32::max(rank, rank_for_target.get(&dependency).unwrap() + 1);
        max_rank = u32::max(max_rank, rank);
      } else {
        missing_dependencies.push(&dependency);
      }
    }

    if missing_dependencies.is_empty() {
      rank_for_target.insert(name, rank);
      closed.push(name);
    } else {
      open.push(name);
      for dep in missing_dependencies {
        if let Ok(index) = open.binary_search(&dep) {
          open.remove(index);
        }

        open.push(dep);
      }
    }
  }
  
  let mut phases: Vec<Phase> = Vec::new();
  phases.resize((max_rank as usize) + 1, Phase::new());

  for (target, rank) in rank_for_target.iter() {
    phases[*rank as usize].push(Invocation {
      rule: rule_for_target.get(target).unwrap(),
      target: target
    });
  }

  Plan {
    makefile: makefile,
    phases: phases
  }
}



fn find_rule<'a> (makefile: &'a Makefile, target: &String) -> &'a Rule {
    makefile.rules.iter()
        .find(|r| r.target == *target)
        .unwrap_or_else(|| panic!("No such rule {:?}", target))
}
