//! Rule System

use crate::types::*;
use crate::world::*;

/// The Rule System.  Processes all rules, executing those that should_fire.
pub fn system(world: &mut World) {
    for tid in 1..world.entities.len() {
        if world.is_rule(tid) && should_fire(world, tid) {
            fire_rule(world, tid);
        }
    }
}

/// Execute the given rule
fn fire_rule(world: &mut World, tid: ID) {
    match &world.entities[tid].rule.as_ref().unwrap().action {
        Action::PrintProse => {
            println!("{}\n", world.prose(tid));
        }
    }

    mark_fired(world, tid);
}

// Returns true if the rule ought to fire, and false otherwise.
fn should_fire(world: &World, tid: ID) -> bool {
    let rule = &world.entities[tid].rule.as_ref().unwrap();

    // The rule ought to fire if it hasn't yet fired, and if its predicate is met.
    // Note that the fired flag is only set if the rule has fired and is once_only.
    !rule.fired && (rule.predicate)(world)
}

// Mark the rule fired (if it's once_only).
fn mark_fired(world: &mut World, tid: ID) {
    if let Some(rule) = &mut world.entities[tid].rule {
        if rule.once_only {
            rule.fired = true;
        }
    }
}
