//! The game world
use std::collections::HashSet;
use std::collections::HashMap;
use crate::entity::*;
use crate::types::*;

/// The game state.  Uses a variant of the Entity-Component-System architecture.
/// This struct provides many methods for querying and mutating entities.  These methods
/// constitute a low-level interface for interacting with the world; e.g., `set_location()`
/// will set the player's location, but that's all it does.  The game logic for entering a new
/// room should be implemented elsewhere.
pub struct World {
    // The entity vector
    pub entities: Vec<Entity>,

    /// The hash map
    pub tag_map: HashMap<String,ID>,

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
    pub fn add_entity(&mut self, entity: Entity) -> ID {
        let id = self.entities.len();
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

    /// Looks up an entity's ID in the tag map.
    /// Panics if the entity is unknown.
    pub fn lookup(&self, tag: &str) -> ID {
        *self.tag_map.get(tag).expect(&format!("Unknown tag: {}", tag))
    }

    /// Returns the name of the entity with the given ID.
    pub fn name(&self, id: ID) -> &str {
        // TODO: retrieve the entity's name once we have one.
        &self.entities[id].name.as_ref().expect(&format!("Name missing: {}", id))
    }

    /// Determines whether the entity is a rule or not
    pub fn is_rule(&self, id: ID) -> bool {
        self.entities[id].rule.is_some()
    }

    // Determines whether the entity is a room or not, i.e., a place the player can be.
    #[allow(dead_code)]
    pub fn is_room(&self, id: ID) -> bool {
        let ent = &self.entities[id];

        ent.prose.is_some() && ent.links.is_some() && ent.inventory.is_some()
    }

    // Determines whether the entity is a thing or not, i.e., an object that can
    // be in a room and that the user can interact with.
    #[allow(dead_code)]
    pub fn is_thing(&self, id: ID) -> bool {
        let ent = &self.entities[id];
        ent.thing.is_some() && ent.prose.is_some()
    }

    // Determines whether the entity is scenery or not, i.e., an object that is in a
    // room and can't be moved.
    #[allow(dead_code)]
    pub fn is_scenery(&self, id: ID) -> bool {
        let ent = &self.entities[id];

        if let Some(thing) = &ent.thing {
            !thing.portable
        } else {
            false
        }
    }

    /// Gets the entity's prose.  Panics if none.
    pub fn prose(&self, id: ID) -> &str {
        &self.entities[id].prose.as_ref()
            .unwrap_or_else(|| panic!("Entity has no prose: {}", id))
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
    pub fn follow(&self, loc: ID, dir: &Dir) -> Option<ID> {
        if let Some(links) = &self.entities[loc].links {
            if let Some(dest) = links.get(&dir) {
                return Some(*dest);
            }
        }
        None
    }

    /// Sets the entity's location
    pub fn set_location(&mut self, id: ID, loc: ID) {
        self.entities[id].loc = Some(loc);
    }

    /// Puts the thing in the container's inventory, and sets the thing's location.
    /// No op if the thing is already in the location.
    pub fn put_in(&mut self, thing: ID, container: ID) {
        if let Some(inv) = &mut self.entities[container].inventory {
            if !inv.things.contains(&thing) {
                inv.things.insert(thing);
                self.entities[thing].loc = Some(container);
            }
        }
    }

    /// Takes the thing out of the container's inventory, and clears the thing's location.
    pub fn take_out(&mut self, thing: ID, container: ID) {
        if let Some(inv) = &mut self.entities[container].inventory {
            if inv.things.contains(&thing) {
                inv.things.remove(&thing);
                self.entities[thing].loc = None;
            }
        }
    }
}
