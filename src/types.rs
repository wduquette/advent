//! Type definitions for this app.

use std::collections::hash_map::HashMap;

/// The entity ID type: an integer.
pub type ID = usize;

/// Directions
#[derive(PartialEq, Eq, Hash, Debug)]
#[allow(dead_code)]
pub enum Dir {
    North,
    South,
    East,
    West,
}

/// Entity prose
pub struct ProseComponent {
    pub name: String,
    pub description: String,
}

/// Inter-room LinksComponent
pub struct LinksComponent {
    pub map: HashMap<Dir, ID>,
}

impl LinksComponent {
    pub fn new() -> LinksComponent {
        LinksComponent {
            map: HashMap::new(),
        }
    }
}

/// Actions taken by triggers (and maybe other things)
#[derive(Debug)]
pub enum Action {
    Print,
}

pub struct TriggerComponent {
    pub predicate: Box<Fn(&World) -> bool>,
    pub action: Action,
    pub once_only: bool,
    pub fired: bool,
}

/// The entity type: a set of optional components
pub struct Entity {
    pub prose: Option<ProseComponent>,
    pub loc: Option<ID>,
    pub links: Option<LinksComponent>,
    pub trigger: Option<TriggerComponent>,
}

impl Entity {
    pub fn new() -> Entity {
        Entity {
            prose: None,
            loc: None,
            links: None,
            trigger: None,
        }
    }
}

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
