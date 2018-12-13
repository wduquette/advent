//! The Entity Data Type and Builder

use crate::types::*;
use crate::world::World;

/// The entity type: a set of optional components defining an entity in the game.
pub struct Entity {
    // The entity's name.  All entities have a name, if only for debugging.
    pub name: String,

    // Many entities have prose, i.e., a room's basic description.
    pub prose: Option<ProseComponent>,

    // Some entities (e.g., the player) have a location.
    pub loc: Option<ID>,

    // Rooms link to other rooms in a variety of directions
    pub links: Option<LinksComponent>,

    // Some entities are Things and have Thing details.
    pub thing: Option<ThingComponent>,

    // Some entities can own/contain Things.
    pub inventory: Option<InventoryComponent>,

    // Some entities are rules, actions to be taken when a condition is met.
    pub rule: Option<RuleComponent>,
}

pub struct EntityBuilder<'a> {
    pub world: &'a mut World,
    pub name: String,
    pub prose: Option<ProseComponent>,
    pub loc: Option<ID>,
    pub links: Option<LinksComponent>,
    pub thing: Option<ThingComponent>,
    pub inventory: Option<InventoryComponent>,
    pub rule: Option<RuleComponent>,
}

impl<'a> EntityBuilder<'a> {
    pub fn make<'b>(world: &'b mut World, name: &str) -> EntityBuilder<'b> {
        EntityBuilder {
            world: world,
            name: name.to_string(),
            prose: None,
            loc: None,
            links: None,
            thing: None,
            inventory: None,
            rule: None,
        }
    }

    pub fn prose(mut self, text: &str) -> EntityBuilder<'a> {
        self.prose = Some(ProseComponent {
            text: text.trim().into(),
        });
        self
    }

    pub fn location(mut self, loc: ID) -> EntityBuilder<'a> {
        self.loc = Some(loc);
        self
    }

    pub fn links(mut self) -> EntityBuilder<'a> {
        self.links = Some(LinksComponent::new());
        self
    }

    pub fn thing(mut self, portable: bool) -> EntityBuilder<'a> {
        self.thing = Some(ThingComponent { portable });
        self
    }

    pub fn inventory(mut self) -> EntityBuilder<'a> {
        self.inventory = Some(InventoryComponent::new());
        self
    }

    pub fn rule<F: 'static>(
        mut self,
        predicate: F,
        action: Action,
        once_only: bool,
    ) -> EntityBuilder<'a>
    where
        F: Fn(&World) -> bool,
    {
        self.rule = Some(RuleComponent::new(predicate, action, once_only));
        self
    }

    pub fn build(self) -> ID {
        self.world.add_entity(Entity {
            name: self.name,
            prose: self.prose,
            loc: self.loc,
            links: self.links,
            thing: self.thing,
            inventory: self.inventory,
            rule: self.rule,
        })
    }
}
