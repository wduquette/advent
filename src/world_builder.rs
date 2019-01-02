//! A scenario builder.
//!
//! It provides builder methods for each kind of entity that the framework supports, with
//! various aids.
//!
//! TODO: Some features of World will move into this module.  World should be primarily a
//! runtime object, not a scenario-building object.

use crate::entity::ID;
use crate::entity::flag_set_component::*;
use crate::entity::inventory_component::*;
use crate::entity::location_component::*;
use crate::entity::player_component::*;
use crate::entity::prose_component::*;
use crate::entity::room_component::*;
use crate::entity::rule_component::*;
use crate::entity::thing_component::*;
use crate::phys;
use crate::types::*;
use crate::world::World;

//-----------------------------------------------------------------------------------------------
// Constants

const LIMBO: &str = "LIMBO";
const PLAYER: &str = "PLAYER";

/// # WorldBuilder
///
/// This struct is used to build game worlds.  It provides an API intended to make it
/// as easy and painless as possible to add game entities to the world with a minimum
/// of error.  As such, it's a precursor to a compiler for a game definition file
/// format.
///
/// The usage pattern is as follows:
///
/// * Create an instance of WorldBuilder.
/// * Use the player(), room(), thing(), etc., methods to add and configure game entities.
/// * When the scenario is complete, the world() method returns the newly created world.
///
/// Two entities are created automatically: LIMBO and PLAYER.  LIMBO is an entity
/// containing only an inventory set; it's a place to park things that shouldn't yet
/// or should no longer be visible in the game; and the PLAYER is, of course, the player.
pub struct WorldBuilder {
    world: World,
}

impl WorldBuilder {
    //-------------------------------------------------------------------------------------------
    // Public Methods

    /// Creates a new world with LIMBO and a player with default settings.
    pub fn new() -> Self {
        // FIRST, create the new world
        let mut this = Self {
            world: World::new(),
        };

        // NEXT, create LIMBO, the container for things which aren't anywhere else.
        let limbo = this.world.alloc(LIMBO);
        assert!(limbo == 0);
        this.add_inventory(limbo);

        // NEXT, create the player basics.  The scenario can customize the player
        // as needed.
        let pid = this.world.alloc(PLAYER);
        this.world.pid = pid;

        this.world.players.insert(pid, PlayerComponent::new());
        this.world.things.insert(pid, ThingComponent::new("Yourself", "self"));
        this.add_inventory(pid);
        this.add_location(pid);
        this.add_flag(pid, Flag::Scenery);

        this
    }

    /// Configures the player.
    pub fn player(&mut self) -> PlayerBuilder {
        PlayerBuilder {
            wb: self,
        }
    }

    /// Creates or configures a room.
    pub fn room(&mut self, tag: &str, name: &str) -> RoomBuilder {
        let id = self.world.alloc(tag);

        self.world.rooms.insert(id, RoomComponent::new(name));
        self.add_inventory(id);
        self.add_flag_set(id);

        RoomBuilder {
            wb: self,
            tag: tag.to_string(),
            id,
        }
    }

    /// Creates or configures a thing.
    pub fn thing(&mut self, tag: &str, name: &str, noun: &str) -> ThingBuilder {
        let id = self.world.alloc(tag);

        self.world.things.insert(id, ThingComponent::new(name, noun));
        self.add_location(id);
        self.add_flag_set(id);

        ThingBuilder {
            wb: self,
            tag: tag.to_string(),
            id,
        }
    }

    /// Creates and configures a rule.
    pub fn rule(&mut self, tag: &str) -> RuleBuilder {
        let id = self.world.alloc(tag);

        self.world.rules.insert(id, RuleComponent::new());
        self.add_flag_set(id);

        RuleBuilder {
            wb: self,
            tag: tag.to_string(),
            id,
        }
    }

    /// Retrieves the created world.
    pub fn world(self) -> World {
        self.world
    }


    //-------------------------------------------------------------------------------------------
    // Utility methods

    /// Adds a location to an entity if it doesn't have one.  The entity will initially
    /// be in LIMBO.
    fn add_location(&mut self, id: ID) {
        if self.world.locations.get(&id).is_none() {
            self.world.locations.insert(id, LocationComponent::new());
        }
    }

    /// Sets the location of the thing to the entity with the given tag, creating
    /// the containing entity if need be.
    fn set_location(&mut self, thing: ID, loc_tag: &str) {
        // FIRST, make sure that the location exists and has an inventory.
        let loc = self.world.alloc(loc_tag);
        self.add_inventory(loc);

        // NEXT, make sure that the thing has a location.
        self.add_location(thing);

        // NEXT, put the thing in the location.
        phys::put_in(&mut self.world, thing, loc);
    }

    /// Adds an inventory to an entity if it doesn't have one.
    fn add_inventory(&mut self, id: ID) {
        if self.world.inventories.get(&id).is_none() {
            self.world.inventories.insert(id, InventoryComponent::new());
        }
    }

    /// Adds a flag set to an entity if it doesn't have one.
    fn add_flag_set(&mut self, id: ID) {
        if self.world.flag_sets.get(&id).is_none() {
            self.world.flag_sets.insert(id, FlagSetComponent::new());
        }
    }

    /// Adds a specific flag to the entity, creating the flag set component if
    /// necessary.
    fn add_flag(&mut self, id: ID, flag: Flag) {
        self.add_flag_set(id);
        self.world.set(id, flag);
    }

    /// Adds a prose component to an entity if it doesn't have one.
    fn add_prose_component(&mut self, id: ID) {
        if self.world.proses.get(&id).is_none() {
            self.world.proses.insert(id, ProseComponent::new());
        }
    }

    /// Adds a prose string of a given type to an entity's prose component,
    /// creating the component if necessary.
    fn add_prose(&mut self, id: ID, prose_type: ProseType, text: &str) {
        self.add_prose_component(id);

        let prose = Prose::Prose(text.into());
        self.world.proses.get_mut(&id).unwrap().types.insert(prose_type, prose);
    }

    /// Adds a prose hook of a given type to an entity's prose component,
    /// creating the component if necessary.
    fn add_prose_hook(&mut self, id: ID, prose_type: ProseType, hook: EntityStringHook) {
        self.add_prose_component(id);

        let prose = Prose::Hook(ProseHook::new(hook));
        self.world.proses.get_mut(&id).unwrap().types.insert(prose_type, prose);
    }
}


/// # PlayerBuilder -- A tool for configuring the player entity.
///
/// WorldBuilder creates and initializes the player automatically; this struct allows
/// the scenario author to configure scenario-specific features.
pub struct PlayerBuilder<'a> {
    wb: &'a mut WorldBuilder,
}

impl<'a> PlayerBuilder<'a> {
    /// Sets the player's initial location given the location's tag
    pub fn location(self, loc_tag: &str) -> PlayerBuilder<'a> {
        self.wb.set_location(self.wb.world.pid, loc_tag);
        let loc = self.wb.world.lookup(loc_tag);
        self.wb.add_flag(self.wb.world.pid, Flag::Seen(loc));

        // TODO: Add expectation that the location is a room.

        self
    }

    /// Adds descriptive prose to the player.
    pub fn prose(self, text: &str) -> PlayerBuilder<'a> {
        self.wb.add_prose(self.wb.world.pid, ProseType::Thing, text);
        self
    }

    /// Adds a prose hook to the player, to produce descriptive prose
    /// on demand.
    pub fn prose_hook(self, hook: EntityStringHook) -> PlayerBuilder<'a> {
        self.wb.add_prose_hook(self.wb.world.pid, ProseType::Thing, hook);
        self
    }

    pub fn flag(self, flag: Flag) -> PlayerBuilder<'a> {
        self.wb.add_flag(self.wb.world.pid, flag);
        self
    }
}

/// # RoomBuilder -- A tool for creating and configuring room entities.
pub struct RoomBuilder<'a> {
    wb: &'a mut WorldBuilder,
    tag: String,
    id: ID,
}

impl<'a> RoomBuilder<'a> {
    /// Adds descriptive prose to the room.
    pub fn prose(self, text: &str) -> RoomBuilder<'a> {
        self.wb.add_prose(self.id, ProseType::Room, text);
        self
    }

    /// Adds a prose hook to the room, to produce descriptive prose
    /// on demand.
    pub fn prose_hook(self, hook: EntityStringHook) -> RoomBuilder<'a> {
        self.wb.add_prose_hook(self.id, ProseType::Room, hook);
        self
    }

    /// Sets a flag on the room.
    pub fn flag(self, flag: Flag) -> RoomBuilder<'a> {
        self.wb.add_flag(self.id, flag);
        self
    }

    /// Creates a link from this room to another room given the direction and
    /// the other room's tag.
    pub fn link(self, dir: Dir, room_tag: &str) -> RoomBuilder<'a> {
        // FIRST, get the id of the destination.
        let dest = self.wb.world.alloc(room_tag);
        // TODO: Add expectation that the destination is a room.

        let link = LinkDest::Room(dest);
        self.wb.world.rooms.get_mut(&self.id).unwrap().links.insert(dir, link);

        self
    }

    /// Adds a dead end in the given direction.
    pub fn dead_end(self, dir: Dir, text: &str) -> RoomBuilder<'a> {
        let dead_end = LinkDest::DeadEnd(text.into());
        self.wb.world.rooms.get_mut(&self.id).unwrap().links.insert(dir, dead_end);
        self
    }
}

/// # ThingBuilder -- A tool for creating and configuring thing entities.
pub struct ThingBuilder<'a> {
    wb: &'a mut WorldBuilder,
    tag: String,
    id: ID,
}

impl<'a> ThingBuilder<'a> {
    /// Sets the thing's initial location given the location's tag.
    pub fn location(self, loc: &str) -> ThingBuilder<'a> {
        self.wb.set_location(self.id, loc);
        self
    }
    /// Adds descriptive prose to the thing.
    pub fn prose(self, text: &str) -> ThingBuilder<'a> {
        self.wb.add_prose(self.id, ProseType::Thing, text);
        self
    }

    /// Adds a prose hook to the thing, to produce descriptive prose
    /// on demand.
    pub fn prose_hook(self, hook: EntityStringHook) -> ThingBuilder<'a> {
        self.wb.add_prose_hook(self.id, ProseType::Thing, hook);
        self
    }

    /// Adds readable prose to the thing.
    pub fn book_prose(self, text: &str) -> ThingBuilder<'a> {
        self.wb.add_prose(self.id, ProseType::Book, text);
        self
    }

    /// Adds a prose hook to the thing, to produce readable prose
    /// on demand.
    pub fn book_hook(self, hook: EntityStringHook) -> ThingBuilder<'a> {
        self.wb.add_prose_hook(self.id, ProseType::Book, hook);
        self
    }

    /// Sets a flag on the thing.
    pub fn flag(self, flag: Flag) -> ThingBuilder<'a> {
        self.wb.add_flag(self.id, flag);
        self
    }
}

/// # RuleBuilder -- A tool for creating and configuring rules.
pub struct RuleBuilder<'a> {
    wb: &'a mut WorldBuilder,
    tag: String,
    id: ID,
}

impl<'a> RuleBuilder<'a> {
    /// Specifies the triggering event.  If omitted, the rule triggers
    /// every turn.
    pub fn on(self, event: Event) -> RuleBuilder<'a> {
        self.wb.world.rules.get_mut(&self.id).unwrap().event = event;
        self
    }

    /// Specifies the predicate.  If omitted, the rule fires every time it
    /// is triggered.
    pub fn when(self, predicate: EventPredicate) -> RuleBuilder<'a> {
        self.wb.world.rules.get_mut(&self.id).unwrap().predicate = predicate;
        self
    }

    /// Specifies that the rule should execute no more than once.
    pub fn once_only(self) -> RuleBuilder<'a> {
        self.wb.add_flag(self.id, Flag::FireOnce);
        self
    }
    /// Specifies text to print when the rule fires.
    pub fn print(self, text: &str) -> RuleBuilder<'a> {
        let rulec = &mut self.wb.world.rules.get_mut(&self.id).unwrap();
        rulec.script.add(Action::Print(text.into()));
        self
    }
}
