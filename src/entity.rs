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

    pub player_info: Option<PlayerInfo>,

    // Room details
    pub room_info: Option<RoomInfo>,

    // Thing details
    pub thing_info: Option<ThingInfo>,

    // Some entities can own/contain Things.
    pub inventory: Option<Inventory>,

    // Entity variable settings
    pub vars: Option<VarSet>,

    // Prose, i.e., a book's content.
    pub prose: Option<ProseComponent>,

    // Some entities are rules, actions to be taken when a condition is met.
    pub rule_info: Option<RuleInfo>,
}

impl Entity {
    /// Can this entity function as a player?
    pub fn is_player(&self) -> bool { PlayerView::is_player(&self) }

    /// Retrieve a view of the entity as a Player
    pub fn as_player(&self) -> PlayerView { PlayerView::from(self) }

    /// Can this entity function as a room?  I.e., a place the player can be?
    pub fn is_room(&self) -> bool { RoomView::is_room(&self) }

    /// Retrieve a view of the entity as a Room
    pub fn as_room(&self) -> RoomView { RoomView::from(self) }

    /// Can this entity function as a thing?  I.e., as a noun?
    pub fn is_thing(&self) -> bool { ThingView::is_thing(&self) }

    /// Retrieve a view of the entity as a Thing
    pub fn as_thing(&self) -> ThingView { ThingView::from(self) }

    /// Is this entity a rule?
    pub fn is_rule(&self) -> bool { RuleView::is_rule(self) }

    /// Retrieve a view of the entity as a Rule
    pub fn as_rule(&self) -> RuleView { RuleView::from(self) }

    /// Does this entity have prose?
    pub fn is_prose(&self) -> bool {
        self.prose.is_some()
    }

    /// Retrieve a view of the entity as a Prose
    pub fn as_prose(&self) -> ProseView {
        assert!(self.is_prose(), "Not prose: [{}] {}", self.id, self.tag);
        ProseView::from(self)
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
    pub location: ID,
    pub inventory: Inventory,
    pub vars: VarSet,
}

impl PlayerView {
    /// Can the entity function as a player?
    pub fn is_player(this: &Entity) -> bool {
        this.player_info.is_some() &&
        this.inventory.is_some() &&
        this.vars.is_some()
    }

    /// Creates a PlayerView for the Entity.  For use by Entity::as_player().
    fn from(this: &Entity) -> PlayerView {
        assert!(this.is_player(), "Not a player: [{}] {}", this.id, this.tag);
        let thing = &this.thing_info.as_ref().unwrap();

        PlayerView {
            id: this.id,
            tag: this.tag.clone(),
            name: thing.name.clone(),
            visual: thing.visual.clone(),
            location: thing.location,
            inventory: this.inventory.as_ref().unwrap().clone(),
            vars: this.vars.as_ref().unwrap().clone(),
        }
    }

    /// Save the player back to the world.  Replaces the links and inventory.
    pub fn save(&mut self, world: &mut World) {
        let thing_info = &mut world.entities[self.id].thing_info.as_mut().unwrap();

        thing_info.location = self.location;
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
    /// Determines whether or not an entity is a room.
    pub fn is_room(this: &Entity) -> bool {
        this.room_info.is_some() &&
        this.inventory.is_some() &&
        this.vars.is_some()
    }

    /// Creates a RoomView for the Entity.  For use by Entity::as_room().
    fn from(this: &Entity) -> RoomView {
        assert!(RoomView::is_room(this), "Not a room: [{}] {}", this.id, this.tag);

        let room_info = &this.room_info.as_ref().unwrap();

        RoomView {
            id: this.id,
            tag: this.tag.clone(),
            name: room_info.name.clone(),
            visual: room_info.visual.clone(),
            links: room_info.links.clone(),
            inventory: this.inventory.as_ref().unwrap().clone(),
            vars: this.vars.as_ref().unwrap().clone(),
        }
    }

    /// Save the room back to the world.  Replaces the links and inventory.
    pub fn save(&mut self, world: &mut World) {
        let room_info = &mut world.entities[self.id].room_info.as_mut().unwrap();

        room_info.links = self.links.clone();
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
    pub location: ID,
    pub vars: VarSet,
}

impl ThingView {
    pub fn is_thing(this: &Entity) -> bool {
        this.thing_info.is_some() &&
        this.vars.is_some()
    }

    /// Creates a ThingView for the Entity.  For use by Entity::as_thing().
    fn from(this: &Entity) -> ThingView {
        assert!(ThingView::is_thing(this), "Not a thing: [{}] {}", this.id, this.tag);
        let thing = &this.thing_info.as_ref().unwrap();

        ThingView {
            id: this.id,
            tag: this.tag.clone(),
            name: thing.name.clone(),
            visual: thing.visual.clone(),
            location: thing.location,
            vars: this.vars.as_ref().unwrap().clone(),
        }
    }

    /// Save the room back to the world.  Replaces the location
    pub fn save(&mut self, world: &mut World) {
        let info = &mut world.entities[self.id].thing_info.as_mut().unwrap();
        info.location = self.location;
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

    // Saved
    pub fired: bool,
}

impl RuleView {
    /// Is the entity a rule?
    pub fn is_rule(this: &Entity) -> bool {
        this.rule_info.is_some()
    }

    /// Creates a RuleView for the Entity.  For use by Entity::as_rule().
    fn from(this: &Entity) -> RuleView {
        assert!(this.is_rule(), "Not a rule: [{}] {}", this.id, this.tag);

        let rule_info = this.rule_info.as_ref().unwrap().clone();
        RuleView {
            id: this.id,
            tag: this.tag.clone(),
            predicate: rule_info.predicate,
            actions: rule_info.actions,
            once_only: rule_info.once_only,
            fired: rule_info.fired,
        }
    }

    /// Save the rule back to the world
    pub fn save(&self, world: &mut World) {
        let mut rule_info = world.entities[self.id].rule_info.as_mut().unwrap();

        rule_info.fired = rule_info.fired;
    }
}

//------------------------------------------------------------------------------------------------
// Prose View

/// Prose view: a view of an entity as a collection of prose.
pub struct ProseView {
    pub id: ID,
    pub tag: String,

    // Saved
    pub main: String,
    pub pages: HashMap<String, String>,
}

impl ProseView {
    /// Creates a ProseView for the Entity.  For use by Entity::as_prose().
    fn from(this: &Entity) -> ProseView {
        let prose = this.prose.as_ref().unwrap();
        ProseView {
            id: this.id,
            tag: this.tag.clone(),
            main: prose.main.clone(),
            pages: prose.pages.clone(),
        }
    }

    /// Save the prose back to the world.  Replaces the main text.
    #[allow(dead_code)]
    pub fn save(&mut self, world: &mut World) {
        world.entities[self.id].prose = Some(ProseComponent {
            main: self.main.clone(),
            pages: self.pages.clone(),
        });
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
    pub player_info: Option<PlayerInfo>,
    pub room_info: Option<RoomInfo>,
    pub thing_info: Option<ThingInfo>,
    pub inventory: Option<Inventory>,
    pub vars: Option<VarSet>,
    pub prose: Option<ProseComponent>,
    pub rule_info: Option<RuleInfo>,
}

impl<'a> EntityBuilder<'a> {
    pub fn make<'b>(world: &'b mut World, tag: &str) -> EntityBuilder<'b> {
        EntityBuilder {
            world: world,
            tag: tag.to_string(),
            player_info: None,
            room_info: None,
            thing_info: None,
            inventory: None,
            vars: None,
            prose: None,
            rule_info: None,
        }
    }

    /// Adds the essential trimmings for a player.
    pub fn player(mut self, start: ID, visual: &str) -> EntityBuilder<'a> {
        assert!(self.player_info.is_none(), "Tried to build player_info twice: {}", self.tag);

        // Someday we'll have some data to go with this.
        self.player_info = Some(PlayerInfo {});

        let mut thing_info = ThingInfo::new("Yourself", "self", visual);
        thing_info.location = start;
        self.thing_info = Some(thing_info);

        if self.inventory.is_none() {
            self.inventory = Some(HashSet::new());
        }

        if self.vars.is_none() {
            self.vars = Some(HashSet::new());
        }

        // We've seen the starting point.
        let vars = &mut self.vars.as_mut().unwrap();
        vars.insert(Var::Seen(start));

        self
    }

    /// Adds the essential trimmings for a room.
    pub fn room(mut self, name: &str, visual: &str) -> EntityBuilder<'a> {
        assert!(self.room_info.is_none(), "Tried to build room_info twice: {}", self.tag);
        self.room_info = Some(RoomInfo::new(name, visual));

        if self.inventory.is_none() {
            self.inventory = Some(HashSet::new());
        }

        if self.vars.is_none() {
            self.vars = Some(HashSet::new());
        }

        self
    }

    /// Adds the essential trimmings for a thing.
    pub fn thing(mut self, name: &str, noun: &str, visual: &str) -> EntityBuilder<'a> {
        assert!(self.thing_info.is_none(), "Tried to build thing_info twice: {}", self.tag);
        self.thing_info = Some(ThingInfo::new(name, noun, visual));

        if self.vars.is_none() {
            self.vars = Some(HashSet::new());
        }

        self
    }

    /// Adds an inventory list to the entity.
    #[allow(dead_code)]
    pub fn inventory(mut self) -> EntityBuilder<'a> {
        self.inventory = Some(HashSet::new());
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
            main: main.trim().into(),
            pages: HashMap::new(),
        });
        self
    }

    /// Adds a page to an existing prose component; the page can be looked up by its
    /// index.
    #[allow(dead_code)]
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
    pub fn once(mut self, predicate: RulePred) -> EntityBuilder<'a> {
        self.rule_info = Some(RuleInfo::once(predicate));
        self
    }

    /// Adds a rule that will fire every time the predicate is met.
    pub fn always(mut self, predicate: RulePred) -> EntityBuilder<'a> {
        self.rule_info = Some(RuleInfo::always(predicate));
        self
    }

    /// Adds an action to a rule.
    pub fn action(mut self, action: Action) -> EntityBuilder<'a> {
        assert!(self.rule_info.is_some(), "Adding action to non-rule: {}", self.tag);
        self.rule_info.as_mut().unwrap().actions.push(action);
        self
    }


    /// Builds the entity, adds it to the world, and sets its ID.  Returns the ID.
    pub fn build(self) -> ID {
        self.world.add_entity(Entity {
            id: 0,
            tag: self.tag,
            player_info: self.player_info,
            room_info: self.room_info,
            thing_info: self.thing_info,
            inventory: self.inventory,
            vars: self.vars,
            prose: self.prose,
            rule_info: self.rule_info,
        })
    }
}
