//! Rule Data

use crate::script::Script;
use crate::types::WorldPredicate;

/// Game rules: actions taken when a predicate is met
#[derive(Clone)]
pub struct RuleComponent {
    pub predicate: WorldPredicate,
    pub script: Script,
}

impl RuleComponent {
    pub fn new(predicate: WorldPredicate) -> RuleComponent {
        RuleComponent {
            predicate: predicate,
            script: Script::new(),
        }
    }
}
