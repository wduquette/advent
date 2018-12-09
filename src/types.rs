//! Type definitions for this app.

use crate::world::*;
use std::collections::hash_map::HashMap;

/// The entity ID type: an integer.
pub type ID = usize;

/// Directions
#[derive(PartialEq, Eq, Hash, Debug)]
#[allow(dead_code)]
pub enum Dir {
    North,
    South,
    East,
    West,
}

/// Entity prose
pub struct ProseComponent {
    pub description: String,
}

/// Inter-room LinksComponent
pub struct LinksComponent {
    pub map: HashMap<Dir, ID>,
}

impl LinksComponent {
    pub fn new() -> LinksComponent {
        LinksComponent {
            map: HashMap::new(),
        }
    }
}

/// Actions taken by triggers (and maybe other things)
#[derive(Debug)]
pub enum Action {
    Print,
}

pub struct TriggerComponent {
    pub predicate: Box<Fn(&World) -> bool>,
    pub action: Action,
    pub once_only: bool,
    pub fired: bool,
}

/// The entity type: a set of optional components
pub struct Entity {
    pub name: String,
    pub prose: Option<ProseComponent>,
    pub loc: Option<ID>,
    pub links: Option<LinksComponent>,
    pub trigger: Option<TriggerComponent>,
}

impl Entity {
    pub fn new() -> Entity {
        Entity {
            name: "Entity".into(),
            prose: None,
            loc: None,
            links: None,
            trigger: None,
        }
    }
}
