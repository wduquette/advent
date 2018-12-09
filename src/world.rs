//! The game world
use crate::types::*;

/// The game state.  Uses a variant of the Entity-Component-System architecture.
pub struct World {
    pub clock: usize,
    pub entities: Vec<Entity>,
    pub player: ID,
}

impl World {
    //--------------------------------------------------------------------------------------------
    // Low-level Infrastructure

    /// Creates a new instance of the World, with an empty entity for the player.
    pub fn new() -> World {
        let mut world = World {
            clock: 0,
            entities: Vec::new(),
            player: 0,
        };

        // Add the player entity, which must still be initialized.
        world.alloc();

        world
    }

    /// Allocate an entity and return its ID
    pub fn alloc(&mut self) -> ID {
        let id = self.entities.len();
        self.entities.push(Entity::new());
        id
    }

    //--------------------------------------------------------------------------------------------
    // Helpers

    /// Retrieves the location of something that has a location.
    pub fn loc(&self, id: ID) -> ID {
        self.entities[id]
            .loc
            .expect(&format!("Entity has no location: {}", id))
    }

    /// Tries to follow a link in the given direction; returns the linked
    /// location if any.
    pub fn follow(&self, loc: ID, dir: Dir) -> Option<ID> {
        if let Some(links) = &self.entities[loc].links {
            if let Some(dest) = links.map.get(&dir) {
                return Some(*dest);
            }
        }
        None
    }
}
