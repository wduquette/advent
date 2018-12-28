//! The game world
use crate::types::EntityStringHook;
use crate::entity::flag::*;
use crate::entity::inventory::*;
use crate::entity::location::*;
use crate::entity::player::*;
use crate::entity::prose::*;
use crate::entity::room::*;
use crate::entity::rule::Action;
use crate::entity::rule::*;
use crate::entity::tag::*;
use crate::entity::thing::*;
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
    pub clock: usize,

    //--------------------------------------------------------------------------------------------
    // Entity Components
    /// Tag Components: Identifiers for the entities.  This is a BTreeMap so that we can
    /// easily reference entities in creation order, and can easily determine the next available
    /// ID.
    pub tags: BTreeMap<ID, TagComponent>,

    /// Flags, used for storing arbitrary data about the entity.  This is mostly for use by
    /// scenarios; infrastructure data should be stored in the regular components.
    pub flag_sets: HashMap<ID, FlagSetComponent>,

    /// Inventory Components: This is broken out separately because many kinds of entity can
    /// own or contain things.
    pub inventories: HashMap<ID, InventoryComponent>,

    /// Location Components: This where things are located.
    pub locations: HashMap<ID, LocationComponent>,

    /// Prose Components: contains all the different kinds of prose an entity can have.
    pub proses: HashMap<ID, ProseComponent>,

    /// Player Components: There should be only one, but it's easier to treat it like the others.
    pub players: HashMap<ID, PlayerComponent>,

    /// Room Components: Information about locations in which the player or NPCs can be.
    pub rooms: HashMap<ID, RoomComponent>,

    /// Thing Components: Information about things that the player can interact with.
    pub things: HashMap<ID, ThingComponent>,

    /// Rule Components: Rules that can fire.
    pub rules: HashMap<ID, RuleComponent>,

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
            rules: HashMap::new(),
            verbs: HashSet::new(),
            synonyms: HashMap::new(),
        };

        // NEXT, add LIMBO, the container for things which aren't anywhere else.
        // TODO: At present LIMBO doesn't have an inventory; it should.
        world.add("LIMBO"); // ID=0

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
    //
    // TODO: Use phrasing "has_X" instead of "is_X" for things like inventory, flag_set, etc.
    // Reserve "is_X" for things, rooms, etc.

    /// Add a new entity using the builder pattern.
    pub fn add(&mut self, tag: &str) -> EBuilder {
        let id = self.next_id;
        self.next_id += 1;

        let tc = TagComponent::new(id, tag);
        self.tags.insert(id, tc);
        self.tag_map.insert(tag.into(), id);

        EBuilder {
            world: self,
            id,
            tag: tag.into(),
        }
    }

    /// Can this entity function as a flag set?
    // TODO: should be has_flag_set()
    pub fn is_flag_set(&self, id: ID) -> bool {
        self.flag_sets.get(&id).is_some()
    }

    /// Can this entity function as an inventory
    // TODO: should be has_inventory()
    pub fn is_inventory(&self, id: ID) -> bool {
        self.inventories.get(&id).is_some()
    }

    /// Retrieve a view of the entity as an inventory
    pub fn as_inventory(&self, id: ID) -> InventoryView {
        InventoryView::from(&self, id)
    }

    /// Does this entity have a location?
    pub fn has_location(&self, id: ID) -> bool {
        self.locations.get(&id).is_some()
    }

    /// Can this entity function as an inventory
    /// TODO: See if this is needed.
    pub fn is_prose(&self, id: ID) -> bool {
        self.proses.get(&id).is_some()
    }

    /// Does this entity have prose of a given type?
    pub fn has_prose(&self, id: ID, prose_type: ProseType) -> bool {
        self.proses.get(&id).is_some() &&
        self.proses[&id].types.get(&prose_type).is_some()
    }

    /// Can this entity function as a player?
    pub fn is_player(&self, id: ID) -> bool {
        self.players.get(&id).is_some()
            && self.locations.get(&id).is_some()
            && self.inventories.get(&id).is_some()
            && self.flag_sets.get(&id).is_some()
            && self.things.get(&id).is_some()
    }

    /// Retrieve a view of the entity as a Player
    pub fn as_player(&self, id: ID) -> PlayerView {
        PlayerView::from(&self, id)
    }

    /// Can this entity function as a room?  I.e., a place the player can be?
    pub fn is_room(&self, id: ID) -> bool {
        self.rooms.get(&id).is_some()
            && self.is_inventory(id)
            && self.is_flag_set(id)
    }

    /// Retrieve a view of the entity as a Room
    pub fn as_room(&self, id: ID) -> RoomView {
        RoomView::from(&self, id)
    }

    /// Can this entity function as a thing?  I.e., as a noun?
    pub fn is_thing(&self, id: ID) -> bool {
        self.things.get(&id).is_some()
            && self.has_location(id)
            && self.is_flag_set(id)
    }

    /// Retrieve a view of the entity as a Thing
    fn as_thing(&self, id: ID) -> ThingView {
        ThingView::from(&self, id)
    }

    /// Is this entity a rule?
    pub fn is_rule(&self, id: ID) -> bool {
        self.rules.get(&id).is_some()
            && self.is_flag_set(id)
    }

    /// Gets a view of the player entity
    pub fn player(&self) -> PlayerView {
        self.as_player(self.pid)
    }

    //--------------------------------------------------------------------------------------------
    // Low-level entity queries and manipulations.

    /// Returns the tag of the thing with the given ID
    pub fn tag(&self, id: ID) -> String {
        self.tags[&id].tag.clone()
    }

    /// Looks up an entity's ID in the tag map.
    /// TODO: Make this just "lookup".  Might want two variants, one that returns
    /// Option and one that panics.
    pub fn lookup_id(&self, tag: &str) -> Option<ID> {
        if let Some(id) = self.tag_map.get(tag) {
            Some(*id)
        } else {
            None
        }
    }

    /// Returns the location of the thing with the given ID
    pub fn loc(&self, id: ID) -> ID {
        assert!(self.has_location(id) "Entity has no location: {}", id);
        self.locations.get(&id).as_ref().unwrap().id
    }

    /// Returns true if the loc owns the thing, and false otherwise.
    pub fn owns(&self, loc: ID, thing: ID) -> bool {
        assert!(self.is_inventory(loc), "Not an inventory: {}", loc);
        self.inventories[&loc].has(thing)
    }

    /// Moves the player (or some other NPC, ultimately) to a location. Performs no
    /// game logic.
    /// TODO: Should be handled by physical system.
    pub fn set_room(&mut self, player: ID, loc: ID) {
        assert!(self.has_location(player) "Not a locatable thing: [{}]", player);
        assert!(self.is_inventory(loc) "Not an inventory: [{}]", loc);

        self.locations.get_mut(&player).unwrap().id = loc;
    }

    /// Puts the thing in the container's inventory, and sets the thing's location.
    /// No op if the thing is already in the location.
    pub fn put_in(&mut self, thing: ID, container: ID) {
        assert!(self.has_location(thing) "Not a thing: [{}]", id);
        assert!(self.is_inventory(container) "Not an inventory: [{}]", container);

        let lc = self.locations.get_mut(&thing).unwrap();
        let ic = self.inventories.get_mut(&container).unwrap();

        ic.things.insert(thing);
        lc.id = container;
    }

    /// Removes the thing from its container's inventory, and puts it in LIMBO.
    pub fn take_out(&mut self, thing: ID) {
        assert!(self.has_location(thing), "Not a thing: [{}]", thing);
        let container = self.loc(thing);

        if container != LIMBO {
            let ic = self.inventories.get_mut(&container).unwrap();
            ic.things.remove(&thing);

            let lc = self.locations.get_mut(&thing).unwrap();
            lc.id = LIMBO;
        }
    }

    /// Tries to follow a link in the given direction; returns the linked
    /// location if any.
    pub fn follow(&self, loc: ID, dir: Dir) -> Option<ID> {
        assert!(self.is_room(loc) "Not a room: [{}]", loc);

        let rc = &self.rooms[&loc];

        rc.links.get(&dir).cloned()
    }

    /// Links one room to another in the given direction.
    /// Links are not bidirectional.  If you want links both ways, you
    /// have to add them.
    pub fn oneway(&mut self, from: ID, dir: Dir, to: ID) {
        assert!(self.is_room(from) "Not a room: [{}]", from);
        assert!(self.is_room(to) "Not a room: [{}]", to);

        let fromc = self.rooms.get_mut(&from).unwrap();
        fromc.links.insert(dir, to);
    }

    /// Links two rooms in the given directions.
    pub fn twoway(&mut self, a: ID, to_b: Dir, to_a: Dir, b: ID) {
        assert!(self.is_room(a) "Not a room: [{}]", a);
        assert!(self.is_room(b) "Not a room: [{}]", b);

        let fromc = self.rooms.get_mut(&a).unwrap();
        fromc.links.insert(to_b, b);

        let toc = self.rooms.get_mut(&b).unwrap();
        toc.links.insert(to_a, a);
    }

    /// Get the specific type of prose from the entity
    pub fn prose(&self, id: ID, prose_type: ProseType) -> String {
        assert!(self.is_prose(id) "Not prose: [{}]", id);

        let prosec = &self.proses[&id];

        if let Some(prose) = &prosec.types.get(&prose_type) {
            prose.as_string(self, id)
        } else {
            "You don't see anything special.".to_string()
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

    /// Set the flag on the entity
    pub fn set_flag(&mut self, id: ID, flag: Flag) {
        assert!(self.is_flag_set(id) "Not a flag set: [{}]", id);

        let fc = self.flag_sets.get_mut(&id).unwrap();

        // Consider adding as_flags() to Entity
        fc.set(flag);
    }

    /// Clear the flag from the entity
    #[allow(dead_code)]
    pub fn unset_flag(&mut self, id: ID, flag: Flag) {
        assert!(self.is_flag_set(id) "Not a flag set: [{}]", id);

        let fc = self.flag_sets.get_mut(&id).unwrap();

        // Consider adding as_flags() to Entity
        fc.unset(flag);
    }

    /// Is the flag set on the entity?
    #[allow(dead_code)]
    pub fn has_flag(&self, id: ID, flag: Flag) -> bool {
        assert!(self.is_flag_set(id) "Not a flag set: [{}]", id);
        let fc = &self.flag_sets[&id];

        fc.has(flag)
    }
}

/// # EBuilder -- A tool for building entities
///
/// Use World.ad() to create an entity and assign it a tag.  This returns an
/// EBuilder struct.  Use the EBuilder methods to add components to the entity.
pub struct EBuilder<'a> {
    pub world: &'a mut World,
    pub id: ID,
    pub tag: String,
}

impl<'a> EBuilder<'a> {
    /// Adds a location component to the entity if it doesn't already have one.
    pub fn location(self) -> EBuilder<'a> {
        if self.world.locations.get(&self.id).is_none() {
            self.world
                .locations
                .insert(self.id, LocationComponent::new());
        }

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

    /// Adds a flag component to the entity if it doesn't already have one.
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

    /// Adds a prose description of the given type to the entity.
    pub fn prose(self, prose_type: ProseType, text: &str) -> EBuilder<'a> {
        if self.world.proses.get(&self.id).is_none() {
            self.world
                .proses
                .insert(self.id, ProseComponent::new());
        }

        self
            .world
            .proses
            .get_mut(&self.id)
            .unwrap()
            .types
            .insert(prose_type, Prose::Prose(text.into()));

        self
    }

    /// Adds a prose description of the given type to the entity.
    pub fn prose_hook(self, prose_type: ProseType, hook: EntityStringHook) -> EBuilder<'a> {
        if self.world.proses.get(&self.id).is_none() {
            self.world
                .proses
                .insert(self.id, ProseComponent::new());
        }

        self
            .world
            .proses
            .get_mut(&self.id)
            .unwrap()
            .types
            .insert(prose_type, Prose::Hook(ProseHook::new(hook)));

        self
    }

    /// Adds the essential trimmings for a player.
    pub fn player(mut self) -> EBuilder<'a> {
        assert!(
            !self.world.players.get(&self.id).is_some(),
            "Tried to add player component twice: [{}] {}",
            self.id,
            self.tag
        );

        self = self.location();
        self = self.inventory();
        self = self.flag_set();

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

        self.world
            .rooms
            .insert(self.id, RoomComponent::new(name));
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

    /// Adds a predicate for a rule that will fire at most once.
    pub fn always(mut self, predicate: RulePred) -> EBuilder<'a> {
        assert!(
            !self.world.rules.get(&self.id).is_some(),
            "Tried to add rule component twice: [{}] {}",
            self.id,
            self.tag
        );

        self = self.flag_set();

        self.world
            .rules
            .insert(self.id, RuleComponent::new(predicate));

        self
    }

    /// Adds a predicate for a rule that will fire at most once.
    pub fn once(mut self, predicate: RulePred) -> EBuilder<'a> {
        assert!(
            !self.world.rules.get(&self.id).is_some(),
            "Tried to add rule component twice: [{}] {}",
            self.id,
            self.tag
        );

        self = self.flag(Flag::FireOnce);

        self.world
            .rules
            .insert(self.id, RuleComponent::new(predicate));

        self
    }

    /// Adds an action to a rule.
    /// TODO: Probably want to add closure that returns ActionScript.
    pub fn action(self, action: Action) -> EBuilder<'a> {
        assert!(
            self.world.rules.get(&self.id).is_some(),
            "Tried to add action to non-rule: [{}] {}",
            self.id,
            self.tag
        );

        let rule = &mut self.world.rules.get_mut(&self.id).unwrap();
        rule.actions.push(action);

        self
    }

    /// Returns the entity's ID.
    pub fn id(self) -> ID {
        self.id
    }
}
