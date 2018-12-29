//! Type definitions for this app.

use crate::entity::ID;
use crate::world::World;

//------------------------------------------------------------------------------------------------
// Basic Types

/// A closure that's a predicate on the World
pub type WorldPredicate = &'static Fn(&World) -> bool;

/// A closure to produce a string from an entity
pub type EntityStringHook = &'static Fn(&World, ID) -> String;

/// The time, in game turns
pub type Time = usize;

/// Directions
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Dir {
    North,
    South,
    East,
    West,
    Up,
    Down,
    In,
    Out,
}

/// The different kinds of prose supported by an entity.
#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum ProseType {
    /// Prose describing a room's interior
    Room,

    /// Prose describing a thing's visible appearance.
    Thing,

    /// The prose contents of a book, note, etc.
    Book,
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
#[allow(dead_code)]
/// Game flags.  At present this is a mixture of engine flags and scenario flags.
pub enum Flag {
    /// This rule should only be fired once.
    FireOnce,

    /// This rule has fired at least once.
    Fired,

    /// Has the entity been killed?
    Dead,

    /// Has this entity been seen by the player?  Used mostly for locations.
    Seen(ID),

    /// Is the thing Scenery?
    Scenery,

    /// Does the player have dirty hands?
    DirtyHands,

    /// Does the location have clean water?
    HasWater,

    /// Is the thing dirty?
    Dirty,
}

/// Actions taken by rules (and maybe other things)
/// TODO: Move this to types, and define ActionScript.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Action {
    /// Print the entity's visual
    Print(String),

    /// Set the flag for the entity with the given ID
    SetFlag(ID, Flag),

    /// Unset the flag on the entity with the given ID.
    UnsetFlag(ID, Flag),

    /// Swap an item in the world for one in LIMBO
    Swap(ID, ID),

    /// Drop(player,thing): Drop a held item into the current location.
    Drop(ID, ID),

    /// Kill the player/NPC with the given ID
    Kill(ID),

    /// Revie the player/NPC with the given ID
    Revive(ID),
}

/// Things that can happen in the game, to which rules, guards, and hooks can be attached
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Event {
    /// A game turn has elapsed
    Turn,

    /// Get(player, thing): A player has gotten (or wants to get) a thing
    GetThing(ID, ID),
}
