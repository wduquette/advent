//! Rule System

use crate::entity::rule::*;
use crate::entity::rule::Action::*;
use crate::visual;
use crate::world::World;

/// The Rule System.  Processes all rules, executing those that should_fire.
pub fn system(world: &mut World) {
    // TODO: Need to provide an interator over IDs; or, world.rules(), an interator over a
    // set of IDs.
    let rules: Vec<RuleView> = world.rules.keys().cloned()
        .filter(|id| world.is_rule(*id))
        .map(|id| world.as_rule(id))
        .collect();

    for mut rulev in rules {
        if !rulev.rule.fired && (rulev.rule.predicate)(world) {
            fire_rule(world, &rulev);
            mark_fired(world, &mut rulev);
        }
    }
}

/// Execute the given rule
fn fire_rule(world: &mut World, rulev: &RuleView) {
    for action in &rulev.rule.actions {
        match action {
            // Print the rule's visual
            Print(visual) => {
                visual::info(visual);
            }

            // Set the flag on the entity's flag set
            SetFlag(id, flag) => {
                world.set_flag(*id, *flag);
            }

            // Clear the flag on the entity's flag set
            ClearFlag(id, flag) => {
                world.unset_flag(*id, *flag);
            }

            // Swap a, in a place, with b, in LIMBO
            Swap(a, b) => {
                let loc = world.loc(*a);
                world.take_out(*a);
                world.put_in(*b, loc);
            }
        }
    }
}

// Mark the rule fired (if it's once_only).
fn mark_fired(world: &mut World, rulev: &mut RuleView) {
    if rulev.rule.once_only {
        rulev.rule.fired = true;
    }

    rulev.save(world);
}
