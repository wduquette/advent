//! The game world
use crate::types::*;

/// The game state
pub struct World {
    pub clock: usize,
    pub entities: Vec<Entity>,
    pub player: ID,
}

impl World {
    pub fn new() -> World {
        let mut world = World {
            clock: 0,
            entities: Vec::new(),
            player: 0,
        };

        // Add the player entity, which must still be initialized.
        world.make_entity();

        world
    }

    // Allocate an entity and return its ID
    pub fn make_entity(&mut self) -> ID {
        let id = self.entities.len();
        self.entities.push(Entity::new());
        id
    }

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
