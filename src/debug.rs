//! Debugging tools module

use crate::types::*;
use crate::world::*;

#[allow(dead_code)]
/// List all entities in the world
pub fn list_world(world: &World) {
    for id in 0..world.entities.len() {
        list_entity(world, id);
    }
}

#[allow(dead_code)]
/// List just the given entity
fn list_entity(world: &World, id: ID) {
    println!("[{}] {}", id, world.name(id));
}

#[allow(dead_code)]
/// Dump all entities in the world
pub fn dump_world(world: &World) {
    for id in 0..world.entities.len() {
        dump_entity(world, id);
    }
}

/// Dump info about the entity with the given ID
#[allow(dead_code)]
pub fn dump_entity(world: &World, id: ID) {
    list_entity(world, id);

    if let Some(loc) = world.entities[id].loc {
        println!("  Location: [{}] -- {}", loc, world.name(loc));
    }

    if let Some(links) = &world.entities[id].links {
        for (dir, id) in &links.map {
            println!("  Link: {:?} to {}", dir, id);
        }
    }

    if let Some(trigger) = &world.entities[id].trigger {
        println!("  Trigger Action: {:?}", trigger.action);
    }

    if let Some(p) = &world.entities[id].prose {
        println!("  Prose: {}", p.text);
    }
}
