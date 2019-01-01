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
use crate::phys;
use crate::types::EntityStringHook;
use crate::types::*;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;

pub const LIMBO: ID = 0;
pub const PLAYER: ID = 1;

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

        // NEXT, add LIMBO, the container for things which aren't anywhere else.
        let id = world.add("LIMBO").inventory().id();
        assert!(id == 0);

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

    //--------------------------------------------------------------------------------------------
    // Entity types

    /// Add a new entity using the builder pattern.  The entity will have the given tag.
    /// See also EBuilder, below.
    pub fn add(&mut self, tag: &str) -> EBuilder {
        while self.tags.get(&self.next_id).is_some() {
            self.next_id += 1;
        }
        let id = self.next_id;

        let tc = TagComponent::new(id, tag);
        self.tags.insert(id, tc);
        self.tag_map.insert(tag.into(), id);

        EBuilder {
            world: self,
            id,
            tag: tag.into(),
        }
    }

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

    /// Links one room to another in the given direction.
    /// Links are not bidirectional.  If you want links both ways, you
    /// have to add them.
    pub fn oneway(&mut self, from: ID, dir: Dir, to: ID) {
        assert!(self.is_room(from), "Not a room: [{}]", from);
        assert!(self.is_room(to), "Not a room: [{}]", to);

        let fromc = self.rooms.get_mut(&from).unwrap();
        fromc.links.insert(dir, LinkDest::Room(to));
    }

    /// Links two rooms in the given directions.
    pub fn twoway(&mut self, a: ID, to_b: Dir, to_a: Dir, b: ID) {
        assert!(self.is_room(a), "Not a room: [{}]", a);
        assert!(self.is_room(b), "Not a room: [{}]", b);

        let fromc = self.rooms.get_mut(&a).unwrap();
        fromc.links.insert(to_b, LinkDest::Room(b));

        let toc = self.rooms.get_mut(&b).unwrap();
        toc.links.insert(to_a, LinkDest::Room(a));
    }

    pub fn tag_owns(&self, owner: ID, thing: &str) -> bool {
        let tid = self.lookup(thing);
        if let Some(inv) = self.inventories.get(&owner) {
            inv.has(tid)
        } else {
            false
        }
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
    pub fn has(&self, id: ID, flag: Flag) -> bool {
        assert!(self.has_flags(id) "Not a flag set: [{}]", id);
        let fc = &self.flag_sets[&id];

        fc.has(flag)
    }

    /// Is the flag set on the entity?
    pub fn tag_has(&self, tag: &str, flag: Flag) -> bool {
        let id = self.lookup(tag);
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

/// # EBuilder -- A tool for building entities
///
/// Use World.add() to create an entity and assign it a tag and ID.  This returns an
/// EBuilder struct.  Use the EBuilder methods to add components to the entity.
pub struct EBuilder<'a> {
    pub world: &'a mut World,
    pub id: ID,
    pub tag: String,
}

impl<'a> EBuilder<'a> {
    /// Adds a location component to the entity if it doesn't already have one.
    /// It will initially be put in LIMBO.
    pub fn location(self) -> EBuilder<'a> {
        if self.world.locations.get(&self.id).is_none() {
            self.world
                .locations
                .insert(self.id, LocationComponent::new());
            phys::put_in(self.world, self.id, LIMBO);
        }

        self
    }

    /// Puts the thing in the given location.
    pub fn put_in(self, loc: ID) -> EBuilder<'a> {
        phys::put_in(self.world, self.id, loc);

        self
    }

    /// Adds an inventory component to the entity if it doesn't already have one.
    pub fn inventory(self) -> EBuilder<'a> {
        if self.world.inventories.get(&self.id).is_none() {
            self.world
                .inventories
                .insert(self.id, InventoryComponent::new());
        }

        self
    }

    /// Adds a flag set component to the entity if it doesn't already have one.
    pub fn flag_set(self) -> EBuilder<'a> {
        if self.world.flag_sets.get(&self.id).is_none() {
            self.world
                .flag_sets
                .insert(self.id, FlagSetComponent::new());
        }

        self
    }

    /// Adds a flag to the entity, creating the flag set if needed.
    pub fn flag(mut self, flag: Flag) -> EBuilder<'a> {
        self = self.flag_set();

        self.world.flag_sets.get_mut(&self.id).unwrap().set(flag);
        self
    }

    /// Adds a prose description of the given type to the entity as a literal string,
    /// creating the prose component if necessary.
    pub fn prose(self, prose_type: ProseType, text: &str) -> EBuilder<'a> {
        if self.world.proses.get(&self.id).is_none() {
            self.world.proses.insert(self.id, ProseComponent::new());
        }

        self.world
            .proses
            .get_mut(&self.id)
            .unwrap()
            .types
            .insert(prose_type, Prose::Prose(text.into()));

        self
    }

    /// Adds a prose description of the given type to the entity using a prose hook,
    /// creating the prose component if necessary.
    pub fn prose_hook(self, prose_type: ProseType, hook: EntityStringHook) -> EBuilder<'a> {
        if self.world.proses.get(&self.id).is_none() {
            self.world.proses.insert(self.id, ProseComponent::new());
        }

        self.world
            .proses
            .get_mut(&self.id)
            .unwrap()
            .types
            .insert(prose_type, Prose::Hook(ProseHook::new(hook)));

        self
    }

    /// Adds the essential trimmings for a player to the entity.
    pub fn player(mut self) -> EBuilder<'a> {
        assert!(
            !self.world.players.get(&self.id).is_some(),
            "Tried to add player component twice: [{}] {}",
            self.id,
            self.tag
        );

        self = self.location();
        self = self.inventory();
        self = self.flag(Flag::Scenery);

        self.world.pid = self.id;
        self.world.players.insert(self.id, PlayerComponent::new());
        self.world
            .things
            .insert(self.id, ThingComponent::new("Yourself", "self"));

        self
    }

    /// Adds the essential trimmings for a room.
    pub fn room(mut self, name: &str) -> EBuilder<'a> {
        assert!(
            !self.world.rooms.get(&self.id).is_some(),
            "Tried to add room component twice: [{}] {}",
            self.id,
            self.tag
        );

        self.world.rooms.insert(self.id, RoomComponent::new(name));
        self = self.inventory();
        self = self.flag_set();

        self
    }

    /// Adds the essential trimmings for a thing.
    pub fn thing(mut self, name: &str, noun: &str) -> EBuilder<'a> {
        assert!(
            !self.world.things.get(&self.id).is_some(),
            "Tried to add thing component twice: [{}] {}",
            self.id,
            self.tag
        );

        self = self.location();
        self = self.flag_set();

        self.world
            .things
            .insert(self.id, ThingComponent::new(name, noun));

        self
    }

    pub fn dead_end(self, dir: Dir, prose: &str) -> EBuilder<'a> {
        assert!(
            self.world.rooms.get(&self.id).is_some(),
            "Tried to a dead end to a non-Room entity: [{}] {}",
            self.id,
            self.tag
        );

        self
            .world.rooms.get_mut(&self.id).unwrap().links.insert(dir, LinkDest::DeadEnd(prose.into()));

        self
    }

    /// Adds an event guard.  If the predicate is true, the event will be allowed to
    /// occur; if it is false, the guard's actions will execute.
    pub fn before(mut self, event: Event, predicate: EventPredicate) -> EBuilder<'a> {
        assert!(
            !self.world.rules.get(&self.id).is_some(),
            "Tried to add rule component twice: [{}] {}",
            self.id,
            self.tag
        );

        let others: Vec<&RuleComponent> = self.world.rules.values()
            .filter(|rc| rc.event == event && rc.is_guard)
            .collect();
        assert!(others.is_empty(), "Tried to add two guards for event: {:?}", event);

        self = self.flag_set();

        self.world
            .rules
            .insert(self.id, RuleComponent::guard(event, predicate));

        self
    }

    /// Adds a predicate for a rule that will fire at most once.
    pub fn always(mut self, event: Event, predicate: EventPredicate) -> EBuilder<'a> {
        assert!(
            !self.world.rules.get(&self.id).is_some(),
            "Tried to add rule component twice: [{}] {}",
            self.id,
            self.tag
        );

        self = self.flag_set();

        self.world
            .rules
            .insert(self.id, RuleComponent::new(event, predicate));

        self
    }

    /// Adds a predicate for a rule that will fire at most once.
    pub fn once(mut self, event: Event, predicate: EventPredicate) -> EBuilder<'a> {
        assert!(
            !self.world.rules.get(&self.id).is_some(),
            "Tried to add rule component twice: [{}] {}",
            self.id,
            self.tag
        );

        self = self.flag(Flag::FireOnce);

        self.world
            .rules
            .insert(self.id, RuleComponent::new(event, predicate));

        self
    }

    /// Adds an action to a rule.
    /// TODO: Probably want to use a closure that returns an ActionScript rather than
    /// adding individual actions.
    pub fn action(self, action: Action) -> EBuilder<'a> {
        assert!(
            self.world.rules.get(&self.id).is_some(),
            "Tried to add action to non-rule: [{}] {}",
            self.id,
            self.tag
        );

        let rule = &mut self.world.rules.get_mut(&self.id).unwrap();
        rule.script.add(action);

        self
    }

    /// Returns the entity's ID, for use at the end of the chain.
    pub fn id(self) -> ID {
        self.id
    }
}
