//! Thing Data

use crate::types::Visual;
use crate::entity::flag::FlagSetComponent;
use crate::entity::ID;
use crate::world::World;
use crate::world::LIMBO;

/// Information specific to things.
#[derive(Debug, Clone)]
pub struct ThingComponent {
    /// The thing's name, for display in inventory lists.
    pub name: String,

    /// The thing's noun, for use in commands
    pub noun: String,

    /// The thing's current location, or LIMBO if it isn't currently
    /// present in any location
    pub location: ID,

    /// The thing's base description.
    pub visual: Visual,
}

impl ThingComponent {
    /// Create a new room with a name, noun, visual, and related info.
    pub fn new(name: &str, noun: &str) -> ThingComponent {
        ThingComponent {
            name: name.into(),
            noun: noun.into(),
            location: LIMBO,
            visual: Visual::Default,
        }
    }
}

//------------------------------------------------------------------------------------------------
// Thing View

/// Thing view: A view of an entity as a Thing
pub struct ThingView {
    pub id: ID,
    pub tag: String,
    pub thing: ThingComponent,
    pub flag_set: FlagSetComponent,
}

impl ThingView {
    /// Creates a ThingView for the entity.
    pub fn from(world: &World, id: ID) -> ThingView {
        let tc = &world.tags[&id];
        assert!(world.is_thing(id), "Not a thing: [{}] {}", tc.id, tc.tag,);

        ThingView {
            id: tc.id,
            tag: tc.tag.clone(),
            thing: world.things[&id].clone(),
            flag_set: world.flag_sets[&id].clone(),
        }
    }

    /// Save the player back to the world.  Replaces the links and inventory.
    pub fn save(&self, world: &mut World) {
        world.things.insert(self.id, self.thing.clone());
        world.flag_sets.insert(self.id, self.flag_set.clone());
    }
}
