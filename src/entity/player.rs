//! Player Data Module

use super::ID;
use crate::entity::flag::FlagSetComponent;
use crate::entity::inventory::InventoryComponent;
use crate::entity::thing::ThingComponent;
use crate::world::World;

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

//------------------------------------------------------------------------------------------------
// Player View

/// Player view: A view of an entity as a Player
pub struct PlayerView {
    pub id: ID,
    pub tag: String,
    pub player: PlayerComponent,
    pub thing: ThingComponent,
    pub inventory: InventoryComponent,
    pub flag_set: FlagSetComponent,
}

impl PlayerView {
    /// Creates a PlayerView for the entity.
    pub fn from(world: &World, id: ID) -> PlayerView {
        let tc = &world.tags[&id];
        assert!(world.is_player(id), "Not a player: [{}] {}", tc.id, tc.tag,);

        PlayerView {
            id: tc.id,
            tag: tc.tag.clone(),
            player: world.players[&id].clone(),
            thing: world.things[&id].clone(),
            inventory: world.inventories[&id].clone(),
            flag_set: world.flag_sets[&id].clone(),
        }
    }

    /// Save the player back to the world.  Replaces the links and inventory.
    pub fn save(&self, world: &mut World) {
        world.players.insert(self.id, self.player.clone());
        world.things.insert(self.id, self.thing.clone());
        world.inventories.insert(self.id, self.inventory.clone());
        world.flag_sets.insert(self.id, self.flag_set.clone());
    }

    /// Gets the player's current location, as a convenience.
    pub fn location(&self) -> ID {
        self.thing.location
    }

    /// Sets the player's current location, as a convenience.
    pub fn set_location(&mut self, loc: ID) {
        self.thing.location = loc;
    }
}
