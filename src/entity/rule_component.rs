//! Rule Data

use crate::script::Script;
use crate::types::Event;
use crate::types::EventPredicate;

/// Game rules: actions taken when a predicate is met
#[derive(Clone)]
pub struct RuleComponent {
    pub event: Event,
    pub is_guard: bool,
    pub predicate: EventPredicate,
    pub script: Script,
}

impl RuleComponent {
    /// Creates a new rule
    pub fn new() -> RuleComponent {
        RuleComponent {
            event: Event::Turn,
            is_guard: false,
            predicate: &|w,e| true,
            script: Script::new(),
        }
    }

    /// Creates a new standard rule, to which actions can be added.
    pub fn newx(event: Event, predicate: EventPredicate) -> RuleComponent {
        RuleComponent {
            event,
            is_guard: false,
            predicate,
            script: Script::new(),
        }
    }

    /// Creates a new guard rule, to which actions can be added.
    pub fn guard(event: Event, predicate: EventPredicate) -> RuleComponent {
        RuleComponent {
            event,
            is_guard: true,
            predicate,
            script: Script::new(),
        }
    }
}
