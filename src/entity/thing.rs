//! Thing Data

use crate::entity::flag::FlagSetComponent;
use crate::entity::ID;
use crate::world::World;

/// Information specific to things.
#[derive(Debug, Clone)]
pub struct ThingComponent {
    /// The thing's name, for display in inventory lists.
    pub name: String,

    /// The thing's noun, for use in commands
    pub noun: String,
}

impl ThingComponent {
    /// Create a new room with a name, noun, visual, and related info.
    pub fn new(name: &str, noun: &str) -> ThingComponent {
        ThingComponent {
            name: name.into(),
            noun: noun.into(),
        }
    }
}
