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
    if world.entities[id].name.is_some() {
        println!("[{}] {} \"{}\"", id, world.entities[id].tag, world.name(id));
    } else {
        println!("[{}] {}", id, world.entities[id].tag);
    }
}

/// Dump info about the entity with the given ID
pub fn dump_entity(world: &World, id: ID) {
    list_entity(world, id);

    if let Some(loc) = world.entities[id].loc {
        println!("  Location: [{}] -- {}", loc, world.name(loc));
    }

    if let Some(links) = &world.entities[id].links {
        for (dir, id) in links {
            println!("  Link: {:?} to {}", dir, id);
        }
    }

    if let Some(vars) = &world.entities[id].vars {
        for var in vars {
            println!("  Var: {:?}", var);
        }
    }

    if let Some(inventory) = &world.entities[id].inventory {
        for tid in inventory {
            println!("  Contains: [{}] {}", *tid, world.name(*tid));
        }
    }

    if let Some(rule) = &world.entities[id].rule {
        for action in &rule.actions {
            println!("  Rule Action: {:?}", action);
        }
    }

    if let Some(visual) = &world.entities[id].visual {
        println!("  Visual: {}", visual);
    }
}
