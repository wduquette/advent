//! Rule Data

use crate::script::Script;
use crate::types::Event;
use crate::types::EventPredicate;

/// Game rules: actions taken when a predicate is met
#[derive(Clone)]
pub struct RuleComponent {
    pub event: Event,
    pub predicate: EventPredicate,
    pub script: Script,
}

impl RuleComponent {
    pub fn new(event: Event, predicate: EventPredicate) -> RuleComponent {
        RuleComponent {
            event,
            predicate,
            script: Script::new(),
        }
    }
}
