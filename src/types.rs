//! Type definitions for this app.

use crate::world::*;
use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;

//------------------------------------------------------------------------------------------------
// Basic Types

/// The entity ID type: an integer.
pub type ID = usize;

/// Level of Detail
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum Detail {
    Full,
    Brief,
}

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

/// Inter-room links
pub type Links = HashMap<Dir, ID>;

/// An Inventory is the set of things contained with the current entity.
pub type Inventory = HashSet<ID>;

//------------------------------------------------------------------------------------------------
// Prose

/// Prose for an object that's readable.
#[derive(Debug, Clone)]
pub struct ProseComponent {
    /// The object's main string: used for signs, notes, and simple books
    pub main: String,

    /// Additional pages that you need to look up given their names.
    pub pages: HashMap<String, String>,
}

//------------------------------------------------------------------------------------------------
// Variables and Variable Sets

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

//------------------------------------------------------------------------------------------------
// Rules and Actions

/// A rule predicate
pub type RulePred = &'static Fn(&World) -> bool;

/// Game rules: actions taken when a predicate is met, and probably never repeated.
pub struct RuleComponent {
    pub predicate: RulePred,
    pub actions: Vec<Action>,
    pub once_only: bool,
    pub fired: bool,
}

impl RuleComponent {
    pub fn once(predicate: RulePred, actions: Vec<Action>) -> RuleComponent {
        RuleComponent {
            predicate: predicate,
            actions,
            once_only: true,
            fired: false,
        }
    }

    pub fn always(predicate: RulePred, actions: Vec<Action>) -> RuleComponent {
        RuleComponent {
            predicate: predicate,
            actions,
            once_only: false,
            fired: false,
        }
    }
}

impl Clone for RuleComponent {
    fn clone(&self) -> RuleComponent {
        RuleComponent {
            predicate: self.predicate,
            actions: self.actions.clone(),
            once_only: self.once_only,
            fired: self.fired,
        }
    }
}

/// Actions taken by rules (and maybe other things)
#[derive(Clone, Debug)]
pub enum Action {
    /// Print the entity's visual
    PrintVisual,

    /// Set the variable for the entity with the given ID
    SetVar(ID, Var),

    /// Clear the given variable
    ClearVar(ID, Var),

    /// Swap an item in the world for one in LIMBO
    Swap(ID, ID),
}
