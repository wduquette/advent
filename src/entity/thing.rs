//! Thing Data

use crate::types::ID;
use crate::world::LIMBO;

/// Information specific to things.
#[derive(Debug, Clone)]
pub struct ThingComponent {
    /// The thing's name, for display in inventory lists.
    pub name: String,

    /// The thing's noun, for use in commands
    pub noun: String,

    /// The thing's current location, or LIMBO if it isn't currently
    /// present in any location
    pub location: ID,

    /// The thing's base description.
    pub visual: String,
}

impl ThingComponent {
    /// Create a new room with a name, noun, visual, and related info.
    pub fn new(name: &str, noun: &str, visual: &str) -> ThingComponent {
        ThingComponent {
            name: name.into(),
            noun: noun.into(),
            location: LIMBO,
            visual: visual.trim().into(),
        }
    }
}
