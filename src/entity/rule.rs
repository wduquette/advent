//! Rule Data

use crate::types::flags::Flag;
use crate::types::ID;
use crate::world::World;

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
    pub fn once(predicate: RulePred) -> RuleComponent {
        RuleComponent {
            predicate: predicate,
            actions: Vec::new(),
            once_only: true,
            fired: false,
        }
    }

    pub fn always(predicate: RulePred) -> RuleComponent {
        RuleComponent {
            predicate: predicate,
            actions: Vec::new(),
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
