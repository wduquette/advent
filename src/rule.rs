//! Rule Monitor System

use std::collections::HashSet;
use crate::entity::ID;
use crate::types::Event;
use crate::types::Flag::*;
use crate::world::World;

/// Fire all rules for the given event, and execute those whose predicates are met.
pub fn fire_event(world: &mut World, event: &Event) {
    let mut events = HashSet::new();
    events.insert(event);
    fire_events(world, &events);
}

/// Fire all rules whose events are in the events set, and execute those whose
/// predicates are met.
pub fn fire_events(world: &mut World, events: &HashSet<&Event>) {
    let rules: Vec<ID> = world
        .rules
        .keys()
        .cloned()
        .filter(|id| !world.has_flag(*id, FireOnce) || !world.has_flag(*id, Fired))
        .collect();

    for id in rules {
        let rulec = &world.rules[&id];
        if events.contains(&rulec.event) && (rulec.predicate)(world) {
            fire_rule(world, id);
        }
    }
}

/// Execute the given rule
fn fire_rule(world: &mut World, id: ID) {
    let script = world.rules[&id].script.clone();
    script.execute(world);
    world.set_flag(id, Fired);
}
