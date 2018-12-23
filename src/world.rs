//! The game world
use crate::entity::*;
use crate::types::*;
use std::collections::HashMap;

pub const LIMBO: ID = 0;

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
        let mut world = World {
            entities: Vec::new(),
            tag_map: HashMap::new(),
            clock: 0,
            pid: 0,
        };

        world.make("LIMBO").build(); // ID=0

        world
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
        assert!(
            self.entities[id].vars.is_some(),
            "Entity has no vars: {}",
            id
        );

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
    pub fn player(&self) -> PlayerView {
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
        let id = self
            .lookup_id(tag)
            .unwrap_or_else(|| panic!("No entity found: {}", tag));
        self.get(id)
    }

    /// Looks up an entity's ID in the tag map.
    pub fn lookup_id(&self, tag: &str) -> Option<ID> {
        if let Some(id) = self.tag_map.get(tag) {
            Some(*id)
        } else {
            None
        }
    }

    /// Returns the location of the thing with the given ID
    pub fn loc(&self, id: ID) -> ID {
        assert!(
            self.entities[id].thing_info.is_some(),
            "Entity has no location: {}",
            id
        );
        self.entities[id].thing_info.as_ref().unwrap().location
    }

    /// Puts the thing in the container's inventory, and sets the thing's location.
    /// No op if the thing is already in the location.
    pub fn put_in(&mut self, thing: ID, container: ID) {
        if let Some(inv) = &mut self.entities[container].inventory {
            inv.insert(thing);
        }

        if let Some(thing_info) = &mut self.entities[thing].thing_info {
            thing_info.location = container;
        }
    }

    /// Removes the thing from its container's inventory, and puts it in LIMBO.
    pub fn take_out(&mut self, thing: ID) {
        let container = self.loc(thing);

        if let Some(inv) = &mut self.entities[container].inventory {
            inv.remove(&thing);
        }

        if let Some(thing_info) = &mut self.entities[thing].thing_info {
            thing_info.location = LIMBO;
        }
    }

    /// Tries to follow a link in the given direction; returns the linked
    /// location if any.
    pub fn follow(&self, loc: ID, dir: Dir) -> Option<ID> {
        if let Some(room_info) = &self.entities[loc].room_info {
            if let Some(dest) = room_info.links.get(&dir) {
                return Some(*dest);
            }
        }
        None
    }
}
