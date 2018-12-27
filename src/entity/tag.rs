//! TagComponent
//! The tag component is the basic identifier for an entity.
//! It consists of the entity's ID, used to identify it throughout
//! the data model, and its string tag, used for debugging and
//! lookups.

use crate::entity::ID;

#[derive(Clone, Debug, Ord, Eq, PartialEq, PartialOrd)]
/// The identifier for an entity.  All entities will have a TagComponent.
pub struct TagComponent {
    /// The entity's ID
    pub id: ID,

    // The entity's tag, used for identification and lookups.
    // All entities have a tag.
    pub tag: String,
}

impl TagComponent {
    pub fn new(id: ID, tag: &str) -> Self {
        Self {
            id,
            tag: tag.into(),
        }
    }
}
