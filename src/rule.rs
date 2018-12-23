//! Rule System

use crate::entity::RuleView;
use crate::types::*;
use crate::world::*;

/// The Rule System.  Processes all rules, executing those that should_fire.
pub fn system(world: &mut World) {
    let rules: Vec<RuleView> = world
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
fn fire_rule(world: &mut World, rule: &RuleView) {
    for action in &rule.actions {
        match action {
            // Print the rule's visual
            Action::Print(visual) => {
                println!("{}\n", visual);
            }

            // Set the flag on the entity's flag set
            Action::SetFlag(id, flag) => {
                world.set_flag(*id, *flag);
            }

            // Clear the flag on the entity's flag set
            Action::ClearFlag(id, flag) => {
                world.unset_flag(*id, flag);
            }

            // Swap a, in a place, with b, in LIMBO
            Action::Swap(a, b) => {
                let loc = world.loc(*a);
                world.take_out(*a);
                world.put_in(*b, loc);
            }
        }
    }
}

// Mark the rule fired (if it's once_only).
fn mark_fired(world: &mut World, rule: &mut RuleView) {
    if rule.once_only {
        rule.fired = true;
    }

    rule.save(world);
}
