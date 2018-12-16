//! The Entity Data Type and Builder

use crate::types::*;
use crate::world::World;
use std::collections::HashMap;
use std::collections::HashSet;

/// The entity type: a set of optional components defining an entity in the game.
pub struct Entity {
    /// The entity's ID, which identifies it uniquely.
    pub id: ID,

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
    pub links: Option<Links>,

    // Some entities can own/contain Things.
    pub inventory: Option<Inventory>,

    // Entity variable settings
    pub vars: Option<VarSet>,

    // Some entities are rules, actions to be taken when a condition is met.
    pub rule: Option<RuleComponent>,
}

impl Entity {
    /// Can this entity function as a player?
    pub fn is_player(&self) -> bool {
        self.name.is_some()
            && self.prose.is_some()
            && self.loc.is_some()
            && self.inventory.is_some()
            && self.vars.is_some()
    }

    /// Retrieve a view of the entity as a Player
    pub fn as_player(&self) -> Player {
        assert!(self.is_player(), "Not a player: [{}] {}", self.id, self.tag);
        Player {
            id: self.id,
            tag: self.tag.clone(),
            name: self.name.as_ref().unwrap().clone(),
            prose: self.prose.as_ref().unwrap().clone(),
            loc: self.loc.unwrap(),
            inventory: self.inventory.as_ref().unwrap().clone(),
            vars: self.vars.as_ref().unwrap().clone(),
        }
    }
    /// Can this entity function as a room?  I.e., a place the player can be?
    pub fn is_room(&self) -> bool {
        self.name.is_some()
            && self.prose.is_some()
            && self.links.is_some()
            && self.inventory.is_some()
            && self.vars.is_some()
    }

    /// Retrieve a view of the entity as a Room
    pub fn as_room(&self) -> Room {
        assert!(self.is_room(), "Not a room: [{}] {}", self.id, self.tag);
        Room {
            id: self.id,
            tag: self.tag.clone(),
            name: self.name.as_ref().unwrap().clone(),
            prose: self.prose.as_ref().unwrap().clone(),
            links: self.links.as_ref().unwrap().clone(),
            inventory: self.inventory.as_ref().unwrap().clone(),
            vars: self.vars.as_ref().unwrap().clone(),
        }
    }

    /// Can this entity function as a thing?  I.e., as a noun?
    pub fn is_thing(&self) -> bool {
        self.name.is_some() && self.prose.is_some() && self.vars.is_some()
    }

    /// Retrieve a view of the entity as a Thing
    pub fn as_thing(&self) -> Thing {
        assert!(self.is_thing(), "Not a thing: [{}] {}", self.id, self.tag);
        Thing {
            id: self.id,
            tag: self.tag.clone(),
            name: self.name.as_ref().unwrap().clone(),
            prose: self.prose.as_ref().unwrap().clone(),
            loc: self.loc.unwrap(),
            vars: self.vars.as_ref().unwrap().clone(),
        }
    }

    /// Is this entity a rule?
    pub fn is_rule(&self) -> bool {
        self.rule.is_some() && self.prose.is_some()
    }

    /// Retrieve a view of the entity as a Rule
    pub fn as_rule(&self) -> Rule {
        assert!(self.is_rule(), "Not a rule: [{}] {}", self.id, self.tag);
        let rule = self.rule.as_ref().unwrap().clone();
        Rule {
            id: self.id,
            tag: self.tag.clone(),
            predicate: rule.predicate,
            actions: rule.actions,
            once_only: rule.once_only,
            prose: self.prose.as_ref().unwrap().clone(),
            fired: rule.fired,
        }
    }

    /// Does this entity have a prose component?
    pub fn is_prose(&self) -> bool {
        self.prose.is_some()
    }

    /// Return the entity's prose.
    pub fn as_prose(&self) -> String {
        assert!(self.is_prose(), "Not prose: [{}] {}", self.id, self.tag);
        self.prose.as_ref().unwrap().clone()
    }
}

//------------------------------------------------------------------------------------------------
// Player View

/// Player view: A view of an entity as a Player
pub struct Player {
    pub id: ID,
    pub tag: String,
    pub name: String,
    pub prose: String,

    // Saved
    pub loc: ID,
    pub inventory: Inventory,
    pub vars: VarSet,
}

impl Player {
    /// Save the player back to the world.  Replaces the links and inventory.
    pub fn save(&mut self, world: &mut World) {
        world.entities[self.id].loc = Some(self.loc);
        world.entities[self.id].inventory = Some(self.inventory.clone());
        world.entities[self.id].vars = Some(self.vars.clone());
    }
}

//------------------------------------------------------------------------------------------------
// Room View

/// Room view: A view of an entity as a Room
pub struct Room {
    pub id: ID,
    pub tag: String,
    pub name: String,
    pub prose: String,

    // Saved
    pub links: Links,
    pub inventory: Inventory,
    pub vars: VarSet,
}

impl Room {
    /// Save the room back to the world.  Replaces the links and inventory.
    pub fn save(&mut self, world: &mut World) {
        world.entities[self.id].links = Some(self.links.clone());
        world.entities[self.id].inventory = Some(self.inventory.clone());
        world.entities[self.id].vars = Some(self.vars.clone());
    }
}

//------------------------------------------------------------------------------------------------
// Thing View

/// Room view: A view of an entity as a Room
pub struct Thing {
    pub id: ID,
    pub tag: String,
    pub name: String,
    pub prose: String,

    // Saved
    pub loc: ID,
    pub vars: VarSet,
}

impl Thing {
    /// Save the room back to the world.  Replaces the location
    pub fn save(&mut self, world: &mut World) {
        world.entities[self.id].loc = Some(self.loc);
        world.entities[self.id].vars = Some(self.vars.clone());
    }
}

//------------------------------------------------------------------------------------------------
// Rule View

/// Rule view: A view of an entity as a Rule
pub struct Rule {
    pub id: ID,
    pub tag: String,
    pub predicate: RulePred,
    pub actions: Vec<Action>,
    pub once_only: bool,
    pub prose: String,

    // Saved
    pub fired: bool,
}

impl Rule {
    /// Save the rule back to the world
    pub fn save(&self, world: &mut World) {
        let mut ent = world.entities[self.id].rule.as_mut().unwrap();

        ent.fired = self.fired;
    }
}

//------------------------------------------------------------------------------------------------
// Entity Builder

/// # EntityBuilder -- A tool for building entities
///
/// Use World.make() to create an EntityBuilder and assign it a tag.  Then use the
/// EntityBuilder methods to add components; then use build() to finish building the
/// Entity and add it to the World's entity vector.
pub struct EntityBuilder<'a> {
    pub world: &'a mut World,
    pub tag: String,
    pub name: Option<String>,
    pub prose: Option<String>,
    pub loc: Option<ID>,
    pub links: Option<Links>,
    pub inventory: Option<Inventory>,
    pub vars: Option<VarSet>,
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
            inventory: None,
            vars: None,
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
        self.links = Some(HashMap::new());
        self
    }

    /// Adds an inventory list to the entity.
    pub fn inventory(mut self) -> EntityBuilder<'a> {
        self.inventory = Some(HashSet::new());
        self
    }

    /// Adds a variable set to the entity.
    pub fn vars(mut self) -> EntityBuilder<'a> {
        self.vars = Some(HashSet::new());
        self
    }

    /// Adds a variable to the entity, creating the var set if needed.
    pub fn var(mut self, var: Var) -> EntityBuilder<'a> {
        if self.vars.is_none() {
            self.vars = Some(HashSet::new());
        }

        self.vars.as_mut().unwrap().insert(var);
        self
    }

    pub fn once(mut self, predicate: RulePred, actions: Vec<Action>) -> EntityBuilder<'a> {
        self.rule = Some(RuleComponent::once(predicate, actions));
        self
    }

    pub fn always(mut self, predicate: RulePred, actions: Vec<Action>) -> EntityBuilder<'a> {
        self.rule = Some(RuleComponent::always(predicate, actions));
        self
    }

    /// Builds the entity, adds it to the world, and sets its ID.  Returns the ID.
    pub fn build(self) -> ID {
        self.world.add_entity(Entity {
            id: 0,
            tag: self.tag,
            name: self.name,
            prose: self.prose,
            loc: self.loc,
            links: self.links,
            inventory: self.inventory,
            vars: self.vars,
            rule: self.rule,
        })
    }
}
