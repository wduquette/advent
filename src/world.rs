//! The game world
use crate::entity::*;
use crate::types::*;
use std::collections::HashMap;
use std::collections::HashSet;

/// The game state.  Uses a variant of the Entity-Component-System architecture.
/// This struct provides many methods for querying and mutating entities.  These methods
/// constitute a low-level interface for interacting with the world; e.g., `set_location()`
/// will set the player's location, but that's all it does.  The game logic for entering a new
/// room should be implemented elsewhere.
pub struct World {
    // The entity vector
    pub entities: Vec<Entity>,

    /// The hash map
    pub tag_map: HashMap<String, ID>,

    /// Various variables about the world.
    pub vars: HashSet<Var>,

    // The game clock
    pub clock: usize,

    // The player's entity ID.
    pub pid: ID,
}

impl World {
    //--------------------------------------------------------------------------------------------
    // Low-level Infrastructure

    /// Creates a new instance of the World
    pub fn new() -> World {
        World {
            entities: Vec::new(),
            tag_map: HashMap::new(),
            vars: HashSet::new(),
            clock: 0,
            pid: 0,
        }
    }

    /// Add an entity and return its ID, saving it in the tag map.
    /// Sets the entity's ID field.
    pub fn add_entity(&mut self, mut entity: Entity) -> ID {
        let id = self.entities.len();
        entity.id = id;
        self.tag_map.insert(entity.tag.clone(), id);

        self.entities.push(entity);
        id
    }

    /// Make an entity using the builder pattern.
    pub fn make(&mut self, tag: &str) -> EntityBuilder {
        EntityBuilder::make(self, tag)
    }

    //--------------------------------------------------------------------------------------------
    // Variables

    /// Set the Var
    pub fn set(&mut self, var: Var) {
        self.vars.insert(var);
    }

    /// Clear the attribute
    #[allow(dead_code)]
    pub fn clear(&mut self, var: &Var) {
        self.vars.remove(var);
    }

    /// Is the variable set?
    pub fn is(&self, var: &Var) -> bool {
        self.vars.contains(var)
    }

    //--------------------------------------------------------------------------------------------
    // Helpers

    /// Retrieve a reference to the given entity.  Usually used in tandom with an
    /// "as_{whatsit}()" method.
    pub fn get(&self, id: ID) -> &Entity {
        &self.entities[id]
    }

    /// Looks up an entity's ID in the tag map.
    /// Panics if the entity is unknown.
    pub fn lookup(&self, tag: &str) -> ID {
        *self
            .tag_map
            .get(tag)
            .unwrap_or_else(|| panic!("Unknown tag: {}", tag))
    }

    /// Returns the name of the entity with the given ID.
    pub fn name(&self, id: ID) -> &str {
        // TODO: retrieve the entity's name once we have one.
        &self.entities[id]
            .name
            .as_ref()
            .unwrap_or_else(|| panic!("Name missing: {}", id))
    }

    /// Retrieves the location of something that has a location.
    /// Panics if it doesn't.
    pub fn loc(&self, id: ID) -> ID {
        self.entities[id]
            .loc
            .unwrap_or_else(|| panic!("Entity has no location: {}", id))
    }

    /// Tries to follow a link in the given direction; returns the linked
    /// location if any.
    pub fn follow(&self, loc: ID, dir: Dir) -> Option<ID> {
        if let Some(links) = &self.entities[loc].links {
            if let Some(dest) = links.get(&dir) {
                return Some(*dest);
            }
        }
        None
    }

}
