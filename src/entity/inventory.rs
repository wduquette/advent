//! Inventory

use crate::types::ID;
use std::collections::BTreeSet;

/// Inventories of Things
#[derive(Debug, Clone, Default)]
pub struct InventoryComponent {
    /// A set of things in the inventory.  We use a BTreeSet so that we preserve the order
    /// in which things were added.
    pub things: BTreeSet<ID>,
}

impl InventoryComponent {
    /// Create a new inventory component
    pub fn new() -> InventoryComponent {
        InventoryComponent {
            things: BTreeSet::new(),
        }
    }
}
