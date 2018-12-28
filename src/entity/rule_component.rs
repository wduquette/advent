//! Rule Data

use crate::entity::ID;
use crate::types::Flag;
use crate::world::World;

/// A rule predicate
pub type RulePred = &'static Fn(&World) -> bool;

/// Game rules: actions taken when a predicate is met, and probably never repeated.
#[derive(Clone)]
pub struct RuleComponent {
    pub predicate: RulePred,
    pub actions: Vec<Action>,
}

impl RuleComponent {
    pub fn new(predicate: RulePred) -> RuleComponent {
        RuleComponent {
            predicate: predicate,
            actions: Vec::new(),
        }
    }
}

/// Actions taken by rules (and maybe other things)
/// TODO: Move this to types, and define ActionScript.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Action {
    /// Print the entity's visual
    Print(String),

    /// Set the variable for the entity with the given ID
    SetFlag(ID, Flag),

    /// Clear the given variable
    ClearFlag(ID, Flag),

    /// Swap an item in the world for one in LIMBO
    Swap(ID, ID),
}
