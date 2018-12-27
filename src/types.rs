//! Type definitions for this app.

use crate::world::World;
use crate::entity::ID;

//------------------------------------------------------------------------------------------------
// Basic Types

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

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
#[allow(dead_code)]
/// Game flags.  At present this is a mixture of engine flags and scenario flags.
pub enum Flag {
    /// Has this entity been seen by the player?  Used mostly for locations.
    Seen(ID),

    /// Does the player have dirty hands?
    DirtyHands,

    /// Does the location have clean water?
    HasWater,

    /// Is the thing Scenery?
    Scenery,

    /// Is the thing dirty?
    Dirty,
}

/// A closure to produce a string from an entity
pub type EntityStringHook = &'static Fn(&World, ID) -> String;
