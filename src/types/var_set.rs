//! Vars and VarSets
use std::collections::HashSet;
use super::ID;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
#[allow(dead_code)]
/// A game variable
pub enum Var {
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

/// A set of variable settings.
pub type VarSet = HashSet<Var>;
