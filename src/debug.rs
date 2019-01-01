//! Debugging tools module

use crate::entity::ID;
use crate::phys;
use crate::world::*;
use crate::types::LinkDest::*;

/// List all entities in the world
pub fn list_world(world: &World) {
    for id in world.tags.keys() {
        list_entity(world, *id);
    }
}

/// List just the given entity
fn list_entity(world: &World, id: ID) {
    let &tc = world.tags.get(&id).as_ref().unwrap();
    println!("[{}] {}", tc.id, tc.tag);
}

/// Dump info about the entity with the given ID
pub fn dump_entity(world: &World, id: ID) {
    list_entity(world, id);

    // FIRST, display its location, if any.
    if world.has_location(id) {
        let here = phys::loc(world, id);
        println!("  Location: [{}] {}", here, world.tag(here));
    }

    // FIRST, display the player info
    if world.players.get(&id).is_some() {
        println!("  Player");
    }

    // NEXT, if it's a thing display the thing info.
    if let Some(thingc) = &world.things.get(&id) {
        println!("  Thing name: {}", thingc.name);
        println!("    Noun: {}", thingc.noun);
    }

    // NEXT, if it's a room display the room info.
    if let Some(roomc) = &world.rooms.get(&id) {
        println!("  Room name: {}", roomc.name);
        for (dir, dest) in &roomc.links {
            match dest {
                Room(id) => {
                    println!("    Link: {:?} to [{}] {}", dir, id, world.tag(*id));
                },
                DeadEnd(prose) => {
                    println!("    Link: {:?} to DeadEnd: {}", dir, prose);
                }
            }
        }
    }

    // NEXT, if it's a rule display its actions.
    if let Some(rulec) = &world.rules.get(&id) {
        for action in &rulec.script.actions {
            println!("  Rule Action: {:?}", action);
        }
    }

    // NEXT, display its flags, if any.
    if let Some(flagc) = &world.flag_sets.get(&id) {
        for flag in flagc.iter() {
            println!("  Flag: {:?}", flag);
        }
    }

    // NEXT, display its inventory, if any.
    if let Some(invc) = world.inventories.get(&id) {
        if invc.things.is_empty() {
            println!("  Contains: nothing");
        } else {
            for tid in &invc.things {
                println!("  Contains: [{}] {}", tid, world.tag(*tid));
            }
        }
    }

    // NEXT, display any associated prose (given the entity's current state)
    if let Some(prosec) = &world.proses.get(&id) {
        for (prose_type, prose) in &prosec.types {
            println!("  Prose [{:?}]: {}", prose_type, prose.as_string(world, id));
        }
    }
}
