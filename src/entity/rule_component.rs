//! Rule Data

use crate::types::Action;
use crate::world::World;

/// A rule predicate
pub type RulePred = &'static Fn(&World) -> bool;

/// Game rules: actions taken when a predicate is met
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
