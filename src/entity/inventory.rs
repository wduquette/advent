//! Inventory

use crate::entity::ID;
use crate::world::World;
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
    pub fn new() -> Self {
        Self {
            things: BTreeSet::new(),
        }
    }

    /// Is the inventory empty?
    pub fn is_empty(&self) -> bool {
        self.things.is_empty()
    }

    /// Does the inventory contain the entity?
    pub fn has(&self, id: ID) -> bool {
        self.things.contains(&id)
    }

    /// Puts the entity in the inventory list.  Performs no other game logic.
    pub fn add(&mut self, id: ID) {
        self.things.insert(id);
    }

    /// Remove an entity from the inventory.
    pub fn remove(&mut self, id: ID) {
        self.things.remove(&id);
    }

    /// An interator over the inventory.
    pub fn iter(&self) -> std::collections::btree_set::Iter<ID> {
        self.things.iter()
    }
}

//------------------------------------------------------------------------------------------------
// Flag View

/// Flag view: A view of an entity as a Flag set
pub struct InventoryView {
    pub id: ID,
    pub tag: String,
    pub inventory: InventoryComponent,
}

impl InventoryView {
    /// Creates a InventoryView for the entity.
    pub fn from(world: &World, id: ID) -> InventoryView {
        let tc = &world.tags[&id];
        assert!(
            world.is_inventory(id),
            "Not an inventory: [{}] {}",
            tc.id,
            tc.tag,
        );

        InventoryView {
            id: tc.id,
            tag: tc.tag.clone(),
            inventory: world.inventories[&id].clone(),
        }
    }

    /// Save the book back to the world.  Replaces the links and inventory.
    pub fn save(&self, world: &mut World) {
        world.inventories.insert(self.id, self.inventory.clone());
    }
}
