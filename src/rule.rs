//! Rule System

use crate::entity::Rule;
use crate::types::*;
use crate::world::*;

/// The Rule System.  Processes all rules, executing those that should_fire.
pub fn system(world: &mut World) {
    let rules: Vec<Rule> = world
        .entities
        .iter()
        .filter(|e| e.is_rule())
        .map(|e| e.as_rule())
        .collect();

    for mut rule in rules {
        if !rule.fired && (rule.predicate)(world) {
            fire_rule(world, &rule);
            mark_fired(world, &mut rule);
        }
    }
}

/// Execute the given rule
fn fire_rule(_world: &mut World, rule: &Rule) {
    match rule.action {
        Action::PrintProse => {
            println!("{}\n", rule.prose);
        }
    }
}

// Mark the rule fired (if it's once_only).
fn mark_fired(world: &mut World, rule: &mut Rule) {
    if rule.once_only {
        rule.fired = true;
    }

    rule.save(world);
}
