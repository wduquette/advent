//! Rule Data

use crate::script::Script;
use crate::types::Event;
use crate::types::RulePredicate;

/// Game rules: actions taken when a predicate is met
#[derive(Clone)]
pub struct RuleComponent {
    pub event: Event,
    pub is_guard: bool,
    pub predicate: RulePredicate,
    pub script: Script,
}

impl RuleComponent {
    /// Creates a new rule
    pub fn new() -> RuleComponent {
        RuleComponent {
            event: Event::Turn,
            is_guard: false,
            predicate: &|_| true,
            script: Script::new(),
        }
    }

    /// Creates a new standard rule, to which actions can be added.
    pub fn newx(event: Event, predicate: RulePredicate) -> RuleComponent {
        RuleComponent {
            event,
            is_guard: false,
            predicate,
            script: Script::new(),
        }
    }

    /// Creates a new guard rule, to which actions can be added.
    pub fn guard(event: Event, predicate: RulePredicate) -> RuleComponent {
        RuleComponent {
            event,
            is_guard: true,
            predicate,
            script: Script::new(),
        }
    }
}

impl Default for RuleComponent {
    fn default() -> Self {
        Self::new()
    }
}
