//! Player Data Module

/// Information specific to Player Entities
#[derive(Debug, Clone, Default)]
pub struct PlayerComponent {
    // None yet; at present, this component serves only as a marker that the entity
// is a (the) player.  This will eventually change.
}

impl PlayerComponent {
    /// Create a new PlayerComponent
    pub fn new() -> Self {
        Self {}
    }
}
