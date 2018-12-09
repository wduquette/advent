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
    Up,
    Down,
    In,
    Out,
}

/// Entity prose
pub struct ProseComponent {
    pub text: String,
}

/// Inter-room links
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

/// Game triggers: actions taken when a predicate is met, and probably never repeated.
pub struct TriggerComponent {
    pub predicate: Box<Fn(&World) -> bool>,
    pub action: Action,
    pub once_only: bool,
    pub fired: bool,
}
