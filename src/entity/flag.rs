//! Entity Flags

use crate::types::flags::FlagSet;

/// Inventories of Things
#[derive(Debug, Clone, Default)]
pub struct FlagComponent {
    /// A set of things in the inventory.  We use a BTreeSet so that we preserve the order
    /// in which things were added.
    pub set: FlagSet,
}

// TODO: Probably move FlagSet code here, make FlagComponent the new FlagSet.
impl FlagComponent {
    /// Create a new inventory component
    pub fn new() -> FlagComponent {
        FlagComponent {
            set: FlagSet::new(),
        }
    }
}
