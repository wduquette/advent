//! The game world
use crate::entity::*;
use crate::types::*;
use crate::types::flags::*;
use std::collections::HashMap;
use std::collections::HashSet;

pub const LIMBO: ID = 0;

/// The game state.  Uses a variant of the Entity-Component-System architecture.
/// This struct provides many methods for querying and mutating entities.  These methods
/// constitute a low-level interface for interacting with the world; e.g., `set_location()`
/// will set the player's location, but that's all it does.  The game logic for entering a new
/// room should be implemented elsewhere.
#[derive(Clone)]
pub struct World {
    // The entity vector
    pub entities: Vec<Entity>,

    /// The hash map
    pub tag_map: HashMap<String, ID>,

    // The game clock
    pub clock: usize,

    // The player's entity ID.
    pub pid: ID,

    // The valid verbs
    pub verbs: HashSet<String>,

    // Mapping from verb synonyms to verbs
    pub synonyms: HashMap<String,String>,
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
            verbs: HashSet::new(),
            synonyms: HashMap::new(),
        };

        // NEXT, add LIMBO, the container for things which aren't anywhere else.
        world.make("LIMBO").build(); // ID=0

        // NEXT, add the standard verbs and synonyms
        world.add_verb("go");

        world.add_verb("north");
        world.add_syn("north", "n");

        world.add_verb("south");
        world.add_syn("south", "s");

        world.add_verb("east");
        world.add_syn("east", "e");

        world.add_verb("west");
        world.add_syn("west", "w");

        world.add_verb("help");
        world.add_verb("look");

        world.add_verb("inventory");
        world.add_syn("inventory", "invent");
        world.add_syn("inventory", "i");

        world.add_verb("examine");
        world.add_syn("examine", "x");

        world.add_verb("get");
        world.add_syn("get", "take");

        world.add_verb("drop");

        world.add_verb("read");

        world.add_verb("quit");
        world.add_syn("quit", "exit");
        world.add_syn("quit", "bye");

        // NEXT, add debugging-only verbs
        world.add_verb("list");
        world.add_verb("dump");

        // NEXT, add custom verbs
        // TODO: Should be part of scenario, once the scenario can define
        // custom command handlers.
        world.add_verb("wash");

        world
    }

    //--------------------------------------------------------------------------------------------
    // Verbs

    /// Adds a single canonical verb to the set of valid verbs.
    pub fn add_verb(&mut self, verb: &str) {
        assert!(!self.verbs.contains(verb));
        self.verbs.insert(verb.to_string());
        self.synonyms.insert(verb.to_string(), verb.to_string());
    }

    /// Adds a synonym verb to the set of valid verbs.
    pub fn add_syn(&mut self, canon: &str, verb: &str) {
        assert!(!self.verbs.contains(verb), "verb already defined: {}", verb);
        self.verbs.insert(verb.to_string());
        self.synonyms.insert(verb.to_string(), canon.to_string());
    }

    //--------------------------------------------------------------------------------------------
    // Entities

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
    // Flags

    /// Set the flag on the entity
    pub fn set_flag(&mut self, id: ID, flag: Flag) {
        assert!(
            self.entities[id].flags.is_some(),
            "Entity has no flags: {}",
            id
        );

        // Consider adding as_flags() to Entity
        if let Some(flags) = &mut self.entities[id].flags {
            flags.set(flag);
        }
    }

    /// Clear the flag from the entity
    #[allow(dead_code)]
    pub fn unset_flag(&mut self, id: ID, flag: &Flag) {
        if let Some(flags) = &mut self.entities[id].flags {
            flags.unset(*flag);
        }
    }

    /// Is the flag set on the entity?
    #[allow(dead_code)]
    pub fn has_flag(&self, id: ID, flag: &Flag) -> bool {
        if let Some(flags) = &self.entities[id].flags {
            flags.has(*flag)
        } else {
            false
        }
    }

    //--------------------------------------------------------------------------------------------
    // Helpers

    /// Can this entity function as a player?
    pub fn is_player(&self, id: ID) -> bool {
        PlayerView::is_player(&self.entities[id])
    }

    /// Retrieve a view of the entity as a Player
    pub fn as_player(&self, id: ID) -> PlayerView {
        PlayerView::from(&self.entities[id])
    }

    /// Can this entity function as a room?  I.e., a place the player can be?
    pub fn is_room(&self, id: ID) -> bool {
        RoomView::is_room(&self.entities[id])
    }

    /// Retrieve a view of the entity as a Room
    pub fn as_room(&self, id: ID) -> RoomView {
        RoomView::from(&self.entities[id])
    }

    /// Can this entity function as a thing?  I.e., as a noun?
    pub fn is_thing(&self, id: ID) -> bool {
        ThingView::is_thing(&self.entities[id])
    }

    /// Retrieve a view of the entity as a Thing
    pub fn as_thing(&self, id: ID) -> ThingView {
        ThingView::from(&self.entities[id])
    }

    /// Is this entity a rule?
    pub fn is_rule(&self, id: ID) -> bool {
        RuleView::is_rule(&self.entities[id])
    }

    /// Retrieve a view of the entity as a Rule
    pub fn as_rule(&self, id: ID) -> RuleView {
        RuleView::from(&self.entities[id])
    }

    /// Does this entity have prose?
    pub fn is_book(&self, id: ID) -> bool {
        BookView::is_book(&self.entities[id])
    }

    /// Retrieve a view of the entity as a Book
    pub fn as_book(&self, id: ID) -> BookView {
        BookView::from(&self.entities[id])
    }

    /// Gets a view of the player entity
    pub fn player(&self) -> PlayerView {
        self.as_player(self.pid)
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
