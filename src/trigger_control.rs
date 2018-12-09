//! Trigger Control System

use crate::types::*;
use crate::world::*;

/// The Trigger Control System.  Processes all triggers, executing those that should_fire.
pub fn system(world: &mut World) {
    for tid in 1..world.entities.len() {
        if world.is_trigger(tid) && should_fire(world, tid) {
            fire_trigger(world, tid);
        }
    }
}

/// Proce
fn fire_trigger(world: &mut World, tid: ID) {
    match &world.entities[tid].trigger.as_ref().unwrap().action {
        Action::Print => {
            println!("{}\n", world.prose(tid));
        }
    }

    mark_fired(world, tid);
}

// Returns true if the trigger ought to fire, and false otherwise.
fn should_fire(world: &World, tid: ID) -> bool {
    let trigger = &world.entities[tid].trigger.as_ref().unwrap();

    // The trigger ought to fire if it hasn't yet fired, and if its predicate is met.
    // Note that the fired flag is only set if the trigger has fired and is once_only.
    !trigger.fired && (trigger.predicate)(world)
}

// Mark the trigger fired (if it's once_only).
fn mark_fired(world: &mut World, tid: ID) {
    if let Some(trigger) = &mut world.entities[tid].trigger {
        if trigger.once_only {
            trigger.fired = true;
        }
    }
}
