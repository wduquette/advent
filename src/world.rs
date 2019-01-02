//! The game world
use crate::entity::flag_set_component::*;
use crate::entity::inventory_component::*;
use crate::entity::location_component::*;
use crate::entity::player_component::*;
use crate::entity::prose_component::*;
use crate::entity::room_component::*;
use crate::entity::rule_component::*;
use crate::entity::tag_component::*;
use crate::entity::thing_component::*;
use crate::entity::ID;
use crate::types::*;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;

pub const LIMBO: ID = 0;

/// The game state.  Uses a variant of the Entity-Component-System architecture.
/// This struct provides many methods for querying and mutating entities.  These methods
/// constitute a low-level interface for interacting with the world; e.g., `set_location()`
/// will set the player's location, but that's all it does.  The game logic for entering a new
/// room should be implemented elsewhere.
#[derive(Clone, Default)]
pub struct World {
    //--------------------------------------------------------------------------------------------
    // World-Global Data
    /// The next entity ID
    next_id: ID,

    /// A map from tags to entity IDs
    pub tag_map: HashMap<String, ID>,

    // The player's entity ID.
    pub pid: ID,

    // The game clock
    pub clock: Time,

    //--------------------------------------------------------------------------------------------
    // Entity Components
    /// Tag Components: Identifiers for the entities.  This is a BTreeMap so that we can
    /// easily reference entities in order of creation.
    pub tags: BTreeMap<ID, TagComponent>,

    /// FlagSets, used for storing arbitrary data about the entity.  Flags include "engine"
    /// flags and custom flags defined by the scenario.
    pub flag_sets: HashMap<ID, FlagSetComponent>,

    /// Inventory Components: For entities that can contain other entities: rooms, boxes,
    /// the player.
    pub inventories: HashMap<ID, InventoryComponent>,

    /// Location Components: Where entities are located.
    pub locations: HashMap<ID, LocationComponent>,

    /// Prose Components: contains all the different kinds of prose an entity can have.
    pub proses: HashMap<ID, ProseComponent>,

    /// Player Components: There should be only one, but it's easier to treat it like the others.
    pub players: HashMap<ID, PlayerComponent>,

    /// Room Components: Information about locations in which the player or NPCs can be.
    pub rooms: HashMap<ID, RoomComponent>,

    /// Thing Components: Information about things that the player can interact with.
    pub things: HashMap<ID, ThingComponent>,

    /// Rule Components: Rules that can fire.  We use BTreeMap to ensure that rules fire
    /// in order of definition.
    pub rules: BTreeMap<ID, RuleComponent>,

    //--------------------------------------------------------------------------------------------
    // Resources

    // The valid verbs
    pub verbs: HashSet<String>,

    // Mapping from verb synonyms to verbs
    pub synonyms: HashMap<String, String>,
}

impl World {
    //--------------------------------------------------------------------------------------------
    // Low-level Infrastructure

    /// Creates a new instance of the World
    pub fn new() -> World {
        let mut world = World {
            next_id: 0,
            tag_map: HashMap::new(),
            pid: 0,
            clock: 0,
            tags: BTreeMap::new(),
            flag_sets: HashMap::new(),
            inventories: HashMap::new(),
            locations: HashMap::new(),
            proses: HashMap::new(),
            players: HashMap::new(),
            rooms: HashMap::new(),
            things: HashMap::new(),
            rules: BTreeMap::new(),
            verbs: HashSet::new(),
            synonyms: HashMap::new(),
        };

        // NEXT, add the standard verbs and synonyms
        // TODO: Decide where this should go.  Possibly not here.
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

        world.add_verb("restart");
        world.add_verb("undo");
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

    //-------------------------------------------------------------------------------------------
    // Entity Creation

    /// Allocates a new entity with a given tag, or returns the old one if it
    /// already existed.
    pub fn alloc(&mut self, tag: &str) -> ID {
        if let Some(id) = self.lookup_id(tag) {
            // We already have it; just return it.
            id
        } else {
            // FIRST, get the next available ID
            while self.tags.get(&self.next_id).is_some() {
                self.next_id += 1;
            }
            let id = self.next_id;

            // NEXT, associate it with its tag.
            let tc = TagComponent::new(id, tag);
            self.tags.insert(id, tc);
            self.tag_map.insert(tag.into(), id);

            // NEXT, return it.
            id
        }
    }

    //--------------------------------------------------------------------------------------------
    // Entity types

    /// Does this entity have a set of flag?
    pub fn has_flags(&self, id: ID) -> bool {
        self.flag_sets.get(&id).is_some()
    }

    /// Does this inventory own other things?
    pub fn has_inventory(&self, id: ID) -> bool {
        self.inventories.get(&id).is_some()
    }

    /// Does this entity have a location?
    pub fn has_location(&self, id: ID) -> bool {
        self.locations.get(&id).is_some()
    }

    /// Does this entity contain prose?
    pub fn has_prose(&self, id: ID) -> bool {
        self.proses.get(&id).is_some()
    }

    /// Does this entity have prose of a given type?
    pub fn has_prose_type(&self, id: ID, prose_type: ProseType) -> bool {
        self.proses.get(&id).is_some() && self.proses[&id].types.get(&prose_type).is_some()
    }

    /// Is this entity a (the) player?
    pub fn is_player(&self, id: ID) -> bool {
        self.players.get(&id).is_some()
            && self.locations.get(&id).is_some()
            && self.inventories.get(&id).is_some()
            && self.flag_sets.get(&id).is_some()
            && self.things.get(&id).is_some()
    }

    /// Is this entity a room where the player can go?
    pub fn is_room(&self, id: ID) -> bool {
        self.rooms.get(&id).is_some() && self.has_inventory(id) && self.has_flags(id)
    }

    /// Is this entity a thing the player can interact with?
    pub fn is_thing(&self, id: ID) -> bool {
        self.things.get(&id).is_some() && self.has_location(id) && self.has_flags(id)
    }

    /// Is this entity a rule?
    pub fn is_rule(&self, id: ID) -> bool {
        self.rules.get(&id).is_some() && self.has_flags(id)
    }

    //--------------------------------------------------------------------------------------------
    // Low-level entity queries and manipulations.

    /// Returns the tag of the thing with the given ID
    pub fn tag(&self, id: ID) -> String {
        self.tags[&id].tag.clone()
    }

    /// Looks up an entity's ID in the tag map.
    pub fn lookup_id(&self, tag: &str) -> Option<ID> {
        if let Some(id) = self.tag_map.get(tag) {
            Some(*id)
        } else {
            None
        }
    }

    /// Looks up an entity's ID in the tag map.  Panics if there is none.
    pub fn lookup(&self, tag: &str) -> ID {
        *self.tag_map.get(tag)
            .unwrap_or_else(|| panic!("No entity with tag: {}", tag))
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
    // Flags

    /// Is the flag set on the entity?
    pub fn has_flag(&self, id: ID, flag: Flag) -> bool {
        assert!(self.has_flags(id) "Not a flag set: [{}]", id);
        let fc = &self.flag_sets[&id];

        fc.has(flag)
    }

    /// Set the flag on the entity
    pub fn set(&mut self, id: ID, flag: Flag) {
        assert!(self.has_flags(id) "Not a flag set: [{}]", id);

        let fc = self.flag_sets.get_mut(&id).unwrap();

        // Consider adding as_flags() to Entity
        fc.set(flag);
    }

    /// Clear the flag from the entity
    pub fn unset(&mut self, id: ID, flag: Flag) {
        assert!(self.has_flags(id) "Not a flag set: [{}]", id);

        let fc = self.flag_sets.get_mut(&id).unwrap();

        // Consider adding as_flags() to Entity
        fc.unset(flag);
    }
}

/// WorldQuery: A query interface, for use by scenario hooks
pub trait WorldQuery {
    // Gets the value of the turn clock
    fn clock(&self) -> usize;

    // Returns true if the given flag is set on the tagged entity, and false
    // otherwise.
    fn has(&self, tag: &str, flag: Flag) -> bool;

    // Returns true if the tagged owner owns the tagged thing, and
    // false otherwise
    fn owns(&self, owner: &str, thing: &str) -> bool;
}

impl WorldQuery for World {
    // Gets the value of the turn clock
    fn clock(&self) -> usize {
        self.clock
    }

    /// Is the flag set on the entity?
    fn has(&self, tag: &str, flag: Flag) -> bool {
        let id = self.lookup(tag);
        assert!(self.has_flags(id) "Not a flag set: [{}]", id);
        let fc = &self.flag_sets[&id];

        fc.has(flag)
    }

    // Returns true if the tagged owner owns the tagged thing, and
    // false otherwise
    fn owns(&self, owner: &str, thing: &str) -> bool {
        let oid = self.lookup(owner);
        let tid = self.lookup(thing);
        if let Some(inv) = self.inventories.get(&oid) {
            inv.has(tid)
        } else {
            false
        }
    }
}
