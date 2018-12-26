//! Rule Data

use crate::types::Flag;
use crate::entity::ID;
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

//------------------------------------------------------------------------------------------------
// Rule View

/// Rule view: A view of an entity as a Rule
pub struct RuleView {
    pub id: ID,
    pub tag: String,
    pub rule: RuleComponent,
}

impl RuleView {
    /// Creates a RuleView for the entity.
    pub fn from(world: &World, id: ID) -> RuleView {
        let tc = world.tags.get(&id).unwrap();
        assert!(
            world.is_rule(id),
            "Not a rule: [{}] {}", tc.id, tc.tag,
        );

        RuleView {
            id: tc.id,
            tag: tc.tag.clone(),
            rule: world.rules.get(&id).unwrap().clone(),
        }
    }

    /// Save the rule back to the world.  Replaces the links and inventory.
    pub fn save(&self, world: &mut World) {
        world.rules.insert(self.id, self.rule.clone());
    }
}
