//! A scenario builder.
//!
//! It provides builder methods for each kind of entity that the framework supports, with
//! various aids.
//!
//! TODO: Some features of World will move into this module.  World should be primarily a
//! runtime object, not a scenario-building object.

use crate::entity::ID;
use crate::world::World;
use crate::entity::inventory_component::InventoryComponent;

//-----------------------------------------------------------------------------------------------
// Constants

const LIMBO: &str = "LIMBO";
const PLAYER: &str = "PLAYER";

pub struct WorldBuilder {
    world: World,
}

impl WorldBuilder {
    /// Creates a new world with LIMBO and a player with default settings.
    pub fn new() -> Self {
        // FIRST, create the new world
        let mut this = Self {
            world: World::new(),
        };

        // NEXT, create LIMBO, the container for things which aren't anywhere else.
        let limbo = this.world.alloc(LIMBO);
        assert!(limbo == 0);
        this.add_inventory(limbo);

        // TODO: Should create PLAYER here.

        this
    }

    //-------------------------------------------------------------------------------------------
    // Utility methods

    /// Adds an inventory to an entity if it doesn't have one.
    fn add_inventory(&mut self, id: ID) {
        if self.world.inventories.get(&id).is_none() {
            self.world.inventories.insert(id, InventoryComponent::new());
        }
    }

    /// Retrieves the created world.
    pub fn world(self) -> World {
        self.world
    }

    // /// Returns a PlayerBuilder for the existing player.  It should provide methods to set
    // /// player details, e.g., the player's description, the player's starting location,
    // /// initial inventory, etc.
    // pub fn player(&mut self) -> PlayerBuilder {
    //     assert!(
    //         self.world.players.is_empty(),
    //         "Tried to add player component twice."
    //     );
    //
    //     let pid = self.world.alloc(PLAYER)
    //
    //     // self = self.location();
    //     // self = self.inventory();
    //     // self = self.flag(Flag::Scenery);
    //     //
    //     // self.world.pid = self.id;
    //     // self.world.players.insert(self.id, PlayerComponent::new());
    //     // self.world
    //     //     .things
    //     //     .insert(self.id, ThingComponent::new("Yourself", "self"));
    //
    //     self
    // }

    //-------------------------------------------------------------------------------------------
    // World Utilities
    //
    // These should be moved to World once WorldBuilder is done.

}
// /// # PlayerBuilder -- A tool for building the player entity
// ///
// /// Use World.add() to create an entity and assign it a tag and ID.  This returns an
// /// EBuilder struct.  Use the EBuilder methods to add components to the entity.
// pub struct EntityBuilder<'a> {
//     pub : &'a mut World,
//     pub id: ID,
//     pub tag: String,
// }
//
// impl<'a> EntityBuilder<'a> {
//     /// Adds a location component to the entity if it doesn't already have one.
//     /// It will initially be put in LIMBO.
//     pub fn location(self) -> EBuilder<'a> {
//         if self.world.locations.get(&self.id).is_none() {
//             self.world
//                 .locations
//                 .insert(self.id, LocationComponent::new());
//             phys::put_in(self.world, self.id, LIMBO);
//         }
//
//         self
//     }
// }
