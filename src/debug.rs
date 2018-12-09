//! Debugging tools module

use crate::types::*;

#[allow(dead_code)]
pub fn list_world(world: &World) {
    for id in 0..world.entities.len() {
        list_entity(world, id);
    }
}

#[allow(dead_code)]
pub fn list_entity(world: &World, id: ID) {
    println!("Entity [{}] -- {}", id, name_or_na(world, id));
}

/// Dump info about all entities.
#[allow(dead_code)]
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
        println!("  Location: [{}] -- {}", loc, name_or_na(world, loc));
    }

    if let Some(links) = &world.entities[id].links {
        for (dir,id) in &links.map {
            println!("  Link: {:?} to {}", dir, id);
        }
    }

    if let Some(trigger) = &world.entities[id].trigger {
        println!("  Trigger Action: {:?}", trigger.action);
    }

    if let Some(p) = &world.entities[id].prose {
        println!("  Description: {}", p.description);
    }
}

#[allow(dead_code)]
fn name(world: &World, id: ID) -> Option<&str> {
    if let Some(p) = &world.entities[id].prose {
        Some(&p.name)
    } else {
        None
    }
}

#[allow(dead_code)]
fn name_or_na(world: &World, id: ID) -> &str {
    name(world, id).unwrap_or_else(|| "n/a")
}
