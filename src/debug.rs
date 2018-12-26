//! Debugging tools module

use crate::types::*;
use crate::world::*;

/// List all entities in the world
pub fn list_world(world: &World) {
    for id in 0..world.entities.len() {
        list_entity(world, id);
    }
}

/// List just the given entity
fn list_entity(world: &World, id: ID) {
    println!("[{}] {}", id, world.entities[id].tag);
}

/// Dump info about the entity with the given ID
pub fn dump_entity(world: &World, id: ID) {
    list_entity(world, id);

    // FIRST, display the player info
    if world.entities[id].player_info.is_some() {
        println!("  Player");
    }

    // NEXT, if it's a thing display the thing info.
    if let Some(thing_info) = &world.entities[id].thing_info {
        println!("  Thing name: {}", thing_info.name);
        println!("    Noun: {}", thing_info.noun);
        println!("    Location: {}", thing_info.location);
        println!("    Visual: {}", thing_info.visual);
    }

    // NEXT, if it's a room display the room info.
    if let Some(room_info) = &world.entities[id].room_info {
        println!("  Room name: {}", room_info.name);
        for (dir, id) in &room_info.links {
            println!("    Link: {:?} to {}", dir, id);
        }
        println!("    Visual: {}", room_info.visual);
    }

    if let Some(flags) = &world.entities[id].flags {
        for flag in flags.iter() {
            println!("  Flag: {:?}", flag);
        }
    }

    if let Some(inventory) = &world.entities[id].inventory {
        for tid in inventory {
            let thing = world.as_thing(*tid);
            println!("  Contains: [{}] {}", thing.id, thing.name);
        }
    }

    if let Some(rule_info) = &world.entities[id].rule_info {
        for action in &rule_info.actions {
            println!("  Rule Action: {:?}", action);
        }
    }
}
