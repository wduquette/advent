//! The game world
use crate::entity::*;
use crate::types::*;
use std::collections::HashMap;

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

    /// Set the variable on the entity
    pub fn set_var(&mut self, id: ID, var: Var) {
        assert!(self.entities[id].vars.is_some(), "No var set: {}", id);

        // Consider adding as_var_set() to Entity
        if let Some(vars) = &mut self.entities[id].vars {
            vars.insert(var);
        }
    }

    /// Clear the variable from the entity
    #[allow(dead_code)]
    pub fn clear_var(&mut self, id: ID, var: &Var) {
        if let Some(vars) = &mut self.entities[id].vars {
            vars.remove(var);
        }
    }

    /// Is the variable set on the entity?
    #[allow(dead_code)]
    pub fn has_var(&self, id: ID, var: &Var) -> bool {
        if let Some(vars) = &self.entities[id].vars {
            vars.contains(&var)
        } else {
            false
        }
    }

    //--------------------------------------------------------------------------------------------
    // Helpers

    /// Gets a view of the player entity
    pub fn player(&self) -> Player {
        self.entities[self.pid].as_player()
    }

    /// Retrieve a reference to the given entity.  Usually used in tandom with an
    /// "as_{whatsit}()" method.
    pub fn get(&self, id: ID) -> &Entity {
        assert!(id < self.entities.len(), "Entity ID out of range: {}", id);
        &self.entities[id]
    }

    /// Looks up an entity in the tag map.
    /// Panics if the entity is unknown.
    pub fn lookup(&self, tag: &str) -> &Entity {
        self.get(self.lookup_id(tag))
    }

    /// Looks up an entity's ID in the tag map.
    /// Panics if the entity is unknown.
    pub fn lookup_id(&self, tag: &str) -> ID {
        *self
            .tag_map
            .get(tag)
            .unwrap_or_else(|| panic!("No entity found: {}", tag))
    }

    /// Returns the name of the entity with the given ID.
    pub fn name(&self, id: ID) -> &str {
        // TODO: retrieve the entity's name once we have one.
        &self.entities[id]
            .name
            .as_ref()
            .unwrap_or_else(|| panic!("Name missing: {}", id))
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
