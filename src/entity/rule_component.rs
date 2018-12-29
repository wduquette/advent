//! Rule Data

use crate::types::Event;
use crate::script::Script;
use crate::types::WorldPredicate;

/// Game rules: actions taken when a predicate is met
#[derive(Clone)]
pub struct RuleComponent {
    pub event: Event,
    pub predicate: WorldPredicate,
    pub script: Script,
}

impl RuleComponent {
    pub fn new(event: Event, predicate: WorldPredicate) -> RuleComponent {
        RuleComponent {
            event,
            predicate,
            script: Script::new(),
        }
    }
}
