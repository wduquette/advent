//! Entity Location Component
//! In principle, the location could be part of the Thing component; but location varies
//! and other ThingComponent attributes do not.

use crate::entity::ID;
use crate::world::LIMBO;

/// Inventories of Things
#[derive(Debug, Clone, Default)]
pub struct LocationComponent {
    /// The location of this entity, for entities that can have a location.
    pub id: ID,
}

impl LocationComponent {
    /// Create a new component
    pub fn new() -> Self {
        Self { id: LIMBO }
    }
}
