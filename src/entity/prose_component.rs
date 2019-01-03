//! The prose component.  It stores the entity's prose visuals.

use crate::entity::ID;
use crate::types::EntityProseHook;
use crate::types::ProseType;
use crate::types::ProseBuffer;
use crate::world::World;
use std::collections::HashMap;
use std::fmt;

/// A hook to convert an entity into prose.
/// We define this struct because we can't add traits to EntityStringHook.
#[derive(Clone)]
pub struct ProseHook {
    hook: EntityProseHook,
}

impl ProseHook {
    /// Creates the hook
    pub fn new(hook: EntityProseHook) -> Self {
        Self { hook }
    }

    /// Call the hook
    pub fn call(&self, world: &World, id: ID) -> String {
        let buff = &mut ProseBuffer::new();
        (self.hook)(world, &world.tag(id), buff);
        buff.get()
    }
}

impl fmt::Debug for ProseHook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ProseHook(...)")
    }
}

/// A Prose value: how to produce a visual string for an entity.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Prose {
    Default,
    Prose(String),
    Hook(ProseHook),
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
    pub types: HashMap<ProseType, Prose>,
}

impl ProseComponent {
    /// Create a new room with a name, noun, visual, and related info.
    pub fn new() -> ProseComponent {
        ProseComponent {
            types: HashMap::new(),
        }
    }
}
