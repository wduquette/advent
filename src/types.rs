//! Type definitions for this app.

use crate::world::*;
use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;

/// The entity ID type: an integer.
pub type ID = usize;

/// Directions
#[derive(PartialEq, Eq, Hash, Debug)]
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

#[derive(PartialEq, Eq, Hash, Debug)]
#[allow(dead_code)]
/// A game variable
pub enum Var {
    /// Has this entity been seen by the player?  Used mostly for locations.
    Seen(ID),

    /// Does the player have dirty hands?
    DirtyHands,

    /// Does the location have clean water?
    HasWater(ID),
}

/// Inter-room links
pub type Links = HashMap<Dir, ID>;

/// A Thing is something that can be in a location and that the user can
/// interact with.  This structure contains details about Things, i.e.,
/// are they portable?
#[derive(Debug)]
pub struct ThingComponent {
    pub portable: bool,
}

/// An Inventory is the set of things contained with the current entity.
pub type Inventory = HashSet<ID>;

/// Actions taken by rules (and maybe other things)
#[derive(Clone, Debug)]
pub enum Action {
    PrintProse,
}

/// A rule predicate
pub type RulePred = &'static Fn(&World) -> bool;

/// Game rules: actions taken when a predicate is met, and probably never repeated.
pub struct RuleComponent {
    // pub predicate: Box<dyn Fn(&World) -> bool>,
    pub predicate: RulePred,
    pub action: Action,
    pub once_only: bool,
    pub fired: bool,
}

impl RuleComponent {
    pub fn new(predicate: RulePred, action: Action, once_only: bool) -> RuleComponent {
        RuleComponent {
            predicate: predicate,
            action,
            once_only,
            fired: false,
        }
    }
}

impl Clone for RuleComponent {
    fn clone(&self) -> RuleComponent {
        RuleComponent {
            predicate: self.predicate,
            action: self.action.clone(),
            once_only: self.once_only,
            fired: self.fired,
        }
    }
}
