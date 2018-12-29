//! Rule Monitor System

use crate::entity::ID;
use crate::types::Event;
use crate::types::Flag::*;
use crate::world::World;

/// The Rule System.  Processes all rules, executing those that should_fire.
pub fn system(world: &mut World, event: &Event) {
    let rules: Vec<ID> = world
        .rules
        .keys()
        .cloned()
        .filter(|id| !world.has_flag(*id, FireOnce) || !world.has_flag(*id, Fired))
        .collect();

    for id in rules {
        let rulec = &world.rules[&id];
        if rulec.event == *event && (rulec.predicate)(world) {
            fire_rule(world, id);
            world.set_flag(id, Fired);
        }
    }
}

/// Execute the given rule
fn fire_rule(world: &mut World, id: ID) {
    let script = world.rules[&id].script.clone();
    script.execute(world);
}
