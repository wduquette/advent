//! Trigger Control System

use crate::types::*;
use crate::world::*;

pub fn system(world: &mut World) {
    for tid in 1..world.entities.len() {
        if world.entities[tid].trigger.is_some() {
            if should_fire(world, tid) {
                process_trigger(world, tid);
            }
        }
    }
}

fn should_fire(world: &World, tid: ID) -> bool {
    let trigger = &world.entities[tid].trigger.as_ref().unwrap();

    !trigger.fired && (trigger.predicate)(world)
}

fn process_trigger(world: &mut World, tid: ID) {
    match &world.entities[tid].trigger.as_ref().unwrap().action {
        Action::Print => {
            print_description(world, tid);
        }
    }

    mark_fired(world, tid);
}

// Mark the trigger fired (if it's once_only).
fn mark_fired(world: &mut World, tid: ID) {
    if let Some(trigger) = &mut world.entities[tid].trigger {
        if trigger.once_only {
            trigger.fired = true;
        }
    }
}

fn print_description(world: &World, loc: ID) {
    // TODO: Need helper.
    let prose = world.entities[loc]
        .prose
        .as_ref()
        .expect(&format!("Entity has no prose: {}", loc));

    println!("{}\n", prose.description);
}
