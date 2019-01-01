//! A scenario builder.
//!
//! It provides builder methods for each kind of entity that the framework supports, with
//! various aids.
//!
//! TODO: Some features of World will move into this module.  World should be primarily a
//! runtime object, not a scenario-building object.

use crate::world::World;

pub struct WorldBuilder {
    world: World,
}

impl WorldBuilder {
    /// Creates a new world with LIMBO and a player with default settings.
    pub fn new() -> Self {
        Self {
            world: World::new(),
        }
    }

    /// Returns a PlayerBuilder for the existing player.  It should provide methods to set
    /// player details, e.g., the player's description, the player's starting location,
    /// initial inventory, etc.
    pub fn player(&mut self) -> PlayerBuilder {
        // TODO
    }
}
/// # PlayerBuilder -- A tool for building the player entity
///
/// Use World.add() to create an entity and assign it a tag and ID.  This returns an
/// EBuilder struct.  Use the EBuilder methods to add components to the entity.
pub struct EntityBuilder<'a> {
    pub : &'a mut World,
    pub id: ID,
    pub tag: String,
}

impl<'a> EntityBuilder<'a> {
    /// Adds a location component to the entity if it doesn't already have one.
    /// It will initially be put in LIMBO.
    pub fn location(self) -> EBuilder<'a> {
        if self.world.locations.get(&self.id).is_none() {
            self.world
                .locations
                .insert(self.id, LocationComponent::new());
            phys::put_in(self.world, self.id, LIMBO);
        }

        self
    }
}
