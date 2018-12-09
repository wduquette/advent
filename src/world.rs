//! The game world
use crate::types::*;

/// The entity type: a set of optional components
pub struct Entity {
    // The entity's name.  All entities have a name, if only for debugging.
    pub name: String,

    // Many entities have prose, i.e., a room's basic description.
    pub prose: Option<ProseComponent>,

    // Some entities (e.g., the player) have a location.
    pub loc: Option<ID>,

    // Rooms link to other rooms in a variety of directions
    pub links: Option<LinksComponent>,

    // Some entities are triggers, actions to be taken when a condition is met.
    pub trigger: Option<TriggerComponent>,
}

impl Entity {
    /// Create an empty entity.
    pub fn new() -> Entity {
        Entity {
            name: "Entity".into(),
            prose: None,
            loc: None,
            links: None,
            trigger: None,
        }
    }
}

/// The game state.  Uses a variant of the Entity-Component-System architecture.
/// This struct provides many methods for querying and mutating entities.  These methods
/// constitute a low-level interface for interacting with the world; e.g., `set_location()`
/// will set the player's location, but that's all it does.  The game logic for entering a new
/// room should be implemented elsewhere.
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

    /// Returns the name of the entity with the given ID.
    pub fn name(&self, id: ID) -> &str {
        &self.entities[id].name
    }

    /// Determines whether the entity is a trigger or not
    pub fn is_trigger(&self, id: ID) -> bool {
        self.entities[id].trigger.is_some()
    }

    /// Gets the entity's prose.  Panics if none.
    pub fn prose(&self, id: ID) -> &str {
        let prose = self.entities[id]
            .prose
            .as_ref()
            .expect(&format!("Entity has no prose: {}", id));

        &prose.text
    }

    /// Retrieves the location of something that has a location.
    /// Panics if it doesn't.
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

    /// Sets the entity's location
    pub fn set_location(&mut self, id: ID, loc: ID) {
        self.entities[id].loc = Some(loc);
    }
}
