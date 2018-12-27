//! Type definitions for this app.

use crate::world::World;
use crate::entity::ID;
use std::fmt;

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

/// A clojure to produce a string from an entity
pub type EntityStringHook = &'static Fn(&World, ID) -> String;

/// A Visual Hook: Computes the visual for an entity.
#[derive(Clone)]
pub struct VisualHook {
    hook: EntityStringHook,
}

impl VisualHook {
    /// Creates the hook
    pub fn new(hook: EntityStringHook) -> Self {
        Self { hook }
    }

    /// Call the hook
    pub fn call(&self, world: &World, id: ID) -> String {
        (self.hook)(world, id)
    }
}

impl fmt::Debug for VisualHook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VisualHook(...)")
    }
}

/// A Visual: how to produce a visual string for an entity.
#[allow(dead_code)]
#[derive(Clone,Debug)]
pub enum Visual {
    Default,
    Prose(String),
    Hook(VisualHook)
}

impl Visual {
    /// Converts the visual to an actual string.
    pub fn as_string(&self, world: &World, id: ID) -> String {
        match self {
            Visual::Default => "You don't see anything special.".to_string(),
            Visual::Prose(str) => str.to_string(),
            Visual::Hook(hook) => hook.call(world, id),
        }
    }
}
