//! The Room Component

use crate::types::LinkDest;
use crate::types::Dir;
use std::collections::HashMap;

/// Information specific to rooms.
#[derive(Debug, Clone)]
pub struct RoomComponent {
    /// The room's name, for display, e.g., "The Town Square"
    pub name: String,

    /// Links from this room to other rooms.
    pub links: HashMap<Dir, LinkDest>,
}

impl RoomComponent {
    /// Create a new room with a name, visual, and related info.
    pub fn new(name: &str) -> RoomComponent {
        RoomComponent {
            name: name.into(),
            links: HashMap::new(),
        }
    }
}
