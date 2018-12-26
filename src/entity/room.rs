//! The Room Component

use crate::world::World;
use crate::entity::inventory::InventoryComponent;
use crate::entity::flag::FlagSetComponent;
use crate::types::Dir;
use crate::entity::ID;
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

//------------------------------------------------------------------------------------------------
// Room View

/// Room view: A view of an entity as a Room
pub struct RoomView {
    pub id: ID,
    pub tag: String,
    pub room: RoomComponent,
    pub inventory: InventoryComponent,
    pub flag_set: FlagSetComponent,
}

impl RoomView {
    /// Creates a RoomView for the entity.
    pub fn from(world: &World, id: ID) -> RoomView {
        let tc = world.tags.get(&id).unwrap();
        assert!(
            world.is_room(id),
            "Not a room: [{}] {}", tc.id, tc.tag,
        );

        RoomView {
            id: tc.id,
            tag: tc.tag.clone(),
            room: world.rooms.get(&id).unwrap().clone(),
            inventory: world.inventories.get(&id).unwrap().clone(),
            flag_set: world.flag_sets.get(&id).unwrap().clone(),
        }
    }

    /// Save the room back to the world.  Replaces the links and inventory.
    pub fn save(&mut self, world: &mut World) {
        world.rooms.insert(self.id, self.room.clone());
        world.inventories.insert(self.id, self.inventory.clone());
        world.flag_sets.insert(self.id, self.flag_set.clone());
    }
}
