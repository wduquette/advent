//! Rule Monitor System

use crate::entity::rule::Action::*;
use crate::entity::ID;
use crate::types::Flag::*;
use crate::visual;
use crate::world::World;

/// The Rule System.  Processes all rules, executing those that should_fire.
pub fn system(world: &mut World) {
    let rules: Vec<ID> = world
        .rules
        .keys()
        .cloned()
        .filter(|id| !world.has_flag(*id, FireOnce) || !world.has_flag(*id, Fired))
        .collect();

    for id in rules {
        if (&world.rules[&id].predicate)(world) {
            fire_rule(world, id);
            world.set_flag(id, Fired);
        }
    }
}

/// Execute the given rule
fn fire_rule(world: &mut World, id: ID) {
    let script = world.rules[&id].actions.clone();
    for action in script {
        match action {
            // Print the rule's visual
            Print(visual) => {
                visual::info(&visual);
            }

            // Set the flag on the entity's flag set
            SetFlag(id, flag) => {
                world.set_flag(id, flag);
            }

            // Clear the flag on the entity's flag set
            ClearFlag(id, flag) => {
                world.unset_flag(id, flag);
            }

            // Swap a, in a place, with b, in LIMBO
            Swap(a, b) => {
                let loc = world.loc(a);
                world.take_out(a);
                world.put_in(b, loc);
            }
        }
    }
}
