//! The Entity Data Type and Builder

use crate::types::*;
use crate::world::World;
use crate::world::LIMBO;
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

    // Most entities have a visual, e.g., what a room or a thing looks like.
    pub visual: Option<String>,

    // Some entities (e.g., the player) have a location.
    pub loc: Option<ID>,

    // Rooms link to other rooms in a variety of directions
    pub links: Option<Links>,

    // Some entities can own/contain Things.
    pub inventory: Option<Inventory>,

    // Entity variable settings
    pub vars: Option<VarSet>,

    // Prose, i.e., a book's content.
    pub prose: Option<ProseComponent>,

    // Some entities are rules, actions to be taken when a condition is met.
    pub rule: Option<RuleComponent>,
}

impl Entity {
    /// Can this entity function as a player?
    pub fn is_player(&self) -> bool {
        self.name.is_some()
            && self.visual.is_some()
            && self.loc.is_some()
            && self.inventory.is_some()
            && self.vars.is_some()
    }

    /// Retrieve a view of the entity as a Player
    pub fn as_player(&self) -> PlayerView {
        assert!(self.is_player(), "Not a player: [{}] {}", self.id, self.tag);
        PlayerView::from(self)
    }
    /// Can this entity function as a room?  I.e., a place the player can be?
    pub fn is_room(&self) -> bool {
        self.name.is_some()
            && self.visual.is_some()
            && self.links.is_some()
            && self.inventory.is_some()
            && self.vars.is_some()
    }

    /// Retrieve a view of the entity as a Room
    pub fn as_room(&self) -> RoomView {
        assert!(self.is_room(), "Not a room: [{}] {}", self.id, self.tag);
        RoomView::from(self)
    }

    /// Can this entity function as a thing?  I.e., as a noun?
    pub fn is_thing(&self) -> bool {
        self.name.is_some() && self.visual.is_some() && self.vars.is_some()
    }

    /// Retrieve a view of the entity as a Thing
    pub fn as_thing(&self) -> ThingView {
        assert!(self.is_thing(), "Not a thing: [{}] {}", self.id, self.tag);
        ThingView::from(self)
    }

    /// Is this entity a rule?
    pub fn is_rule(&self) -> bool {
        self.rule.is_some() && self.visual.is_some()
    }

    /// Retrieve a view of the entity as a Rule
    pub fn as_rule(&self) -> RuleView {
        assert!(self.is_rule(), "Not a rule: [{}] {}", self.id, self.tag);
        RuleView::from(self)
    }

    /// Does this entity have prose?
    pub fn is_prose(&self) -> bool {
        self.prose.is_some()
    }

    /// Retrieve a view of the entity as a Prose
    pub fn as_prose(&self) -> ProseView {
        assert!(self.is_prose(), "Not prose: [{}] {}", self.id, self.tag);
        ProseView::from(self)
    }

    /// Does this entity have an inventory?
    pub fn is_inventory(&self) -> bool {
        self.inventory.is_some()
    }

    /// Retrieve a view of the entity as an Inventory
    pub fn as_inventory(&self) -> InventoryView {
        assert!(
            self.is_inventory(),
            "Not inventory: [{}] {}",
            self.id,
            self.tag
        );
        InventoryView::from(self)
    }
    /// Does this entity have a visual component?
    pub fn is_visual(&self) -> bool {
        self.visual.is_some()
    }

    /// Return the entity's visual.
    pub fn as_visual(&self) -> String {
        assert!(self.is_visual(), "Not visual: [{}] {}", self.id, self.tag);
        self.visual.as_ref().unwrap().clone()
    }
}

//------------------------------------------------------------------------------------------------
// Player View

/// Player view: A view of an entity as a Player
pub struct PlayerView {
    pub id: ID,
    pub tag: String,
    pub name: String,
    pub visual: String,

    // Saved
    pub loc: ID,
    pub inventory: Inventory,
    pub vars: VarSet,
}

impl PlayerView {
    /// Creates a PlayerView for the Entity.  For use by Entity::as_player().
    fn from(this: &Entity) -> PlayerView {
        PlayerView {
            id: this.id,
            tag: this.tag.clone(),
            name: this.name.as_ref().unwrap().clone(),
            visual: this.visual.as_ref().unwrap().clone(),
            loc: this.loc.unwrap(),
            inventory: this.inventory.as_ref().unwrap().clone(),
            vars: this.vars.as_ref().unwrap().clone(),
        }
    }

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
pub struct RoomView {
    pub id: ID,
    pub tag: String,
    pub name: String,
    pub visual: String,

    // Saved
    pub links: Links,
    pub inventory: Inventory,
    pub vars: VarSet,
}

impl RoomView {
    /// Creates a RoomView for the Entity.  For use by Entity::as_room().
    fn from(this: &Entity) -> RoomView {
        RoomView {
            id: this.id,
            tag: this.tag.clone(),
            name: this.name.as_ref().unwrap().clone(),
            visual: this.visual.as_ref().unwrap().clone(),
            links: this.links.as_ref().unwrap().clone(),
            inventory: this.inventory.as_ref().unwrap().clone(),
            vars: this.vars.as_ref().unwrap().clone(),
        }
    }

    /// Save the room back to the world.  Replaces the links and inventory.
    pub fn save(&mut self, world: &mut World) {
        world.entities[self.id].links = Some(self.links.clone());
        world.entities[self.id].inventory = Some(self.inventory.clone());
        world.entities[self.id].vars = Some(self.vars.clone());
    }
}

//------------------------------------------------------------------------------------------------
// Thing View

/// Thing view: A view of an entity as a Thing
pub struct ThingView {
    pub id: ID,
    pub tag: String,
    pub name: String,
    pub visual: String,

    // Saved
    pub loc: ID,
    pub vars: VarSet,
}

impl ThingView {
    /// Creates a ThingView for the Entity.  For use by Entity::as_thing().
    fn from(this: &Entity) -> ThingView {
        ThingView {
            id: this.id,
            tag: this.tag.clone(),
            name: this.name.as_ref().unwrap().clone(),
            visual: this.visual.as_ref().unwrap().clone(),
            loc: this.loc.unwrap(),
            vars: this.vars.as_ref().unwrap().clone(),
        }
    }

    /// Save the room back to the world.  Replaces the location
    pub fn save(&mut self, world: &mut World) {
        world.entities[self.id].loc = Some(self.loc);
        world.entities[self.id].vars = Some(self.vars.clone());
    }
}

//------------------------------------------------------------------------------------------------
// Rule View

/// Rule view: A view of an entity as a Rule
pub struct RuleView {
    pub id: ID,
    pub tag: String,
    pub predicate: RulePred,
    pub actions: Vec<Action>,
    pub once_only: bool,
    pub visual: String,

    // Saved
    pub fired: bool,
}

impl RuleView {
    /// Creates a RuleView for the Entity.  For use by Entity::as_rule().
    fn from(this: &Entity) -> RuleView {
        let rule = this.rule.as_ref().unwrap().clone();
        RuleView {
            id: this.id,
            tag: this.tag.clone(),
            predicate: rule.predicate,
            actions: rule.actions,
            once_only: rule.once_only,
            visual: this.visual.as_ref().unwrap().clone(),
            fired: rule.fired,
        }
    }

    /// Save the rule back to the world
    pub fn save(&self, world: &mut World) {
        let mut ent = world.entities[self.id].rule.as_mut().unwrap();

        ent.fired = self.fired;
    }
}

//------------------------------------------------------------------------------------------------
// Inventory View

/// Inventory view: a view of an entity as an inventory
pub struct InventoryView {
    pub id: ID,
    pub tag: String,

    // Saved
    pub inventory: Inventory,
}

impl InventoryView {
    /// Creates a InventoryView for the Entity.  For use by Entity::as_inventory().
    fn from(this: &Entity) -> InventoryView {
        InventoryView {
            id: this.id,
            tag: this.tag.clone(),
            inventory: this.inventory.as_ref().unwrap().clone(),
        }
    }

    /// Save the prose back to the world.  Replaces the main text.
    pub fn save(&mut self, world: &mut World) {
        world.entities[self.id].inventory = Some(self.inventory.clone());
    }
}
//------------------------------------------------------------------------------------------------
// Prose View

/// Prose view: a view of an entity as a collection of prose.
pub struct ProseView {
    pub id: ID,
    pub tag: String,

    // Saved
    pub prose: ProseComponent,
}

impl ProseView {
    /// Creates a ProseView for the Entity.  For use by Entity::as_prose().
    fn from(this: &Entity) -> ProseView {
        ProseView {
            id: this.id,
            tag: this.tag.clone(),
            prose: this.prose.as_ref().unwrap().clone(),
        }
    }

    /// Save the prose back to the world.  Replaces the main text.
    pub fn save(&mut self, world: &mut World) {
        world.entities[self.id].prose = Some(self.prose.clone());
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
    pub visual: Option<String>,
    pub loc: Option<ID>,
    pub links: Option<Links>,
    pub inventory: Option<Inventory>,
    pub vars: Option<VarSet>,
    pub prose: Option<ProseComponent>,
    pub rule: Option<RuleComponent>,
}

impl<'a> EntityBuilder<'a> {
    pub fn make<'b>(world: &'b mut World, tag: &str) -> EntityBuilder<'b> {
        EntityBuilder {
            world: world,
            tag: tag.to_string(),
            name: None,
            visual: None,
            loc: None,
            links: None,
            inventory: None,
            vars: None,
            prose: None,
            rule: None,
        }
    }

    pub fn name(mut self, name: &str) -> EntityBuilder<'a> {
        self.name = Some(name.into());
        self
    }

    pub fn visual(mut self, visual: &str) -> EntityBuilder<'a> {
        self.visual = Some(visual.trim().into());
        self
    }

    pub fn location(mut self, loc: ID) -> EntityBuilder<'a> {
        self.loc = Some(loc);
        self
    }

    pub fn limbo(mut self) -> EntityBuilder<'a> {
        self.loc = Some(LIMBO);
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

    /// Adds a variable to the entity, creating the var set if needed.
    pub fn prose(mut self, main: &str) -> EntityBuilder<'a> {
        self.prose = Some(ProseComponent {
            main: main.into(),
            pages: HashMap::new(),
        });
        self
    }

    /// Adds a page to an existing prose component; the page can be looked up by its
    /// index.
    pub fn page(mut self, index: &str, text: &str) -> EntityBuilder<'a> {
        assert!(
            self.prose.is_some(),
            "Can't add page, no prose component: {}",
            self.tag
        );
        self.prose
            .as_mut()
            .unwrap()
            .pages
            .insert(index.into(), text.into());
        self
    }

    /// Adds a rule that will fire at most once.
    pub fn once(mut self, predicate: RulePred, actions: Vec<Action>) -> EntityBuilder<'a> {
        self.rule = Some(RuleComponent::once(predicate, actions));
        self
    }

    /// Adds a rule that will fire every time the predicate is met.
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
            visual: self.visual,
            loc: self.loc,
            links: self.links,
            inventory: self.inventory,
            vars: self.vars,
            prose: self.prose,
            rule: self.rule,
        })
    }
}
