//! The Entity Data Type and Builder

use crate::types::*;
use crate::world::World;

/// The entity type: a set of optional components defining an entity in the game.
pub struct Entity {
    // The entity's tag, used for identification and lookups.
    // All entities have a tag.
    pub tag: String,

    // Many entities have names for display.
    pub name: Option<String>,

    // Many entities have prose, i.e., a room's basic description.
    pub prose: Option<String>,

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
    pub tag: String,
    pub name: Option<String>,
    pub prose: Option<String>,
    pub loc: Option<ID>,
    pub links: Option<LinksComponent>,
    pub thing: Option<ThingComponent>,
    pub inventory: Option<InventoryComponent>,
    pub rule: Option<RuleComponent>,
}

impl<'a> EntityBuilder<'a> {
    pub fn make<'b>(world: &'b mut World, tag: &str) -> EntityBuilder<'b> {
        EntityBuilder {
            world: world,
            tag: tag.to_string(),
            name: None,
            prose: None,
            loc: None,
            links: None,
            thing: None,
            inventory: None,
            rule: None,
        }
    }

    pub fn name(mut self, name: &str) -> EntityBuilder<'a> {
        self.name = Some(name.into());
        self
    }

    pub fn prose(mut self, prose: &str) -> EntityBuilder<'a> {
        self.prose = Some(prose.trim().into());
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
            tag: self.tag,
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
