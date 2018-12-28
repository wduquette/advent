//! The prose component.  It stores the entity's prose visuals.

use std::collections::HashMap;
use crate::entity::ID;
use crate::world::World;
use crate::types::EntityStringHook;
use crate::types::ProseType;
use std::fmt;

/// A hook to convert an entity into prose.
/// We define this struct because we can't add traits to EntityStringHook.
#[derive(Clone)]
pub struct ProseHook {
    hook: EntityStringHook,
}

impl ProseHook {
    /// Creates the hook
    pub fn new(hook: EntityStringHook) -> Self {
        Self { hook }
    }

    /// Call the hook
    pub fn call(&self, world: &World, id: ID) -> String {
        (self.hook)(world, id)
    }
}

impl fmt::Debug for ProseHook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ProseHook(...)")
    }
}

/// A Prose value: how to produce a visual string for an entity.
#[allow(dead_code)]
#[derive(Clone,Debug)]
pub enum Prose {
    Default,
    Prose(String),
    Hook(ProseHook)
}

impl Prose {
    /// Converts the prose to an actual string.
    pub fn as_string(&self, world: &World, id: ID) -> String {
        match self {
            Prose::Default => "You don't see anything special.".to_string(),
            Prose::Prose(str) => str.to_string(),
            Prose::Hook(hook) => hook.call(world, id),
        }
    }
}

/// Information specific to entity prose
#[derive(Debug, Clone, Default)]
pub struct ProseComponent {
    pub types: HashMap<ProseType,Prose>,
}

impl ProseComponent {
    /// Create a new room with a name, noun, visual, and related info.
    pub fn new() -> ProseComponent {
        ProseComponent {
            types: HashMap::new(),
        }
    }
}

//------------------------------------------------------------------------------------------------
// Prose View

/// Prose view: A view of an entity as a read-only collection of prose
pub struct ProseView {
    pub id: ID,
    pub tag: String,
    pub prose: ProseComponent,
}

impl ProseView {
    /// Creates a ProseView for the entity.
    pub fn from(world: &World, id: ID) -> Self {
        let tc = &world.tags[&id];

        assert!(world.is_prose(id), "Not prose: [{}] {}", tc.id, tc.tag,);

        Self {
            id: tc.id,
            tag: tc.tag.clone(),
            prose: world.proses[&id].clone(),
        }
    }
}
