//! The Room Component

use crate::types::Dir;
use crate::types::ID;
use std::collections::HashMap;

/// Information specific to rooms.
#[derive(Debug, Clone)]
pub struct RoomComponent {
    /// The room's name, for display, e.g., "The Town Square"
    pub name: String,

    /// The room's base description, e.g., "A busy town square, with..."
    pub visual: String,

    /// Links from this room to other rooms.
    pub links: HashMap<Dir, ID>,
}

impl RoomComponent {
    /// Create a new room with a name, visual, and related info.
    pub fn new(name: &str, visual: &str) -> RoomComponent {
        RoomComponent {
            name: name.into(),
            visual: visual.trim().into(),
            links: HashMap::new(),
        }
    }
}
