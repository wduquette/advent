//! Rule Monitor System

use crate::entity::ID;
use crate::types::Event;
use crate::types::Flag::*;
use crate::world::World;

/// Executes the guard that applies to the given event (if any), and returns
/// whether or not the event is allowed.  If the event is denied, the guard's
/// script is executed.
pub fn allows(world: &mut World, event: &Event) -> bool {
    for id in world.rules.keys().cloned() {
        let rulec = &world.rules[&id];
        if rulec.is_guard && event == &rulec.event {
            if (rulec.predicate)(world) {
                // The action is not allowed; execute the script.
                let script = rulec.script.clone();
                script.execute(world);
                return false;
            } else {
                // The action is allowed.
                return true;
            }
        }
    }

    // NEXT, no guard matches; carry on normally.
    true
}

/// Fire all rules for the given event, and execute those whose predicates are met.
pub fn fire_event(world: &mut World, event: &Event) {
    fire_events(world, &[event]);
}

/// Fire all rules whose events are in the events set, and execute those whose
/// predicates are met.
pub fn fire_events(world: &mut World, events: &[&Event]) {
    let rules: Vec<ID> = world
        .rules
        .keys()
        .cloned()
        .filter(|id| !world.has_flag(*id, FireOnce) || !world.has_flag(*id, Fired))
        .collect();

    for id in rules {
        let rulec = &world.rules[&id];
        if !rulec.is_guard
            && events.contains(&&rulec.event)
            && (rulec.predicate)(world)
        {
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
