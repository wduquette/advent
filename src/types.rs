//! Type definitions for this app.

use crate::entity::ID;
use crate::world::WorldQuery;

//------------------------------------------------------------------------------------------------
// Basic Types

/// A closure that's a predicate on the World.
pub type RulePredicate = &'static Fn(&WorldQuery) -> bool;

/// A closure to produce a string from an entity
pub type EntityStringHook = &'static Fn(&WorldQuery, &str) -> String;

/// The time, in game turns
pub type Time = usize;

/// Directions
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
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

/// The different kinds of prose supported by an entity.
#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum ProseType {
    /// Prose describing a room's interior
    Room,

    /// Prose describing a thing's visible appearance.
    Thing,

    /// The prose contents of a book, note, etc.
    Book,
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
#[allow(dead_code)]
/// Game flags.  At present this is a mixture of engine flags and scenario flags.
pub enum Flag {
    /// This rule should only be fired once.
    FireOnce,

    /// This rule has fired at least once.
    Fired,

    /// Has the entity been killed?
    Dead,

    /// Has this entity been seen by the player?  Used mostly for locations.
    Seen(ID),

    /// Is the thing immovable?  I.e., something that cannot be picked up?
    Immovable,

    /// Is the thing scenery, i.e., something that appears in an entity
    /// description but doesn't usually appear in printed inventory
    /// lists?  Scenery is also assumed to be immovable, but isn't
    /// explicitly flagged as such.
    Scenery,

    /// A generic flag type for use by users
    User(&'static str),

    /// A (flag + ID) flag type for use by users
    UserId(&'static str, ID)
}

/// Actions taken by rules (and maybe other things)
/// TODO: Move this to types, and define ActionScript.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Action {
    /// Print the entity's visual
    Print(String),

    /// Set the flag for the entity with the given ID
    SetFlag(ID, Flag),

    /// Unset the flag on the entity with the given ID.
    UnsetFlag(ID, Flag),

    /// PutIn(thing, inv)
    PutIn(ID, ID),

    /// Swap an item in the world for one in LIMBO
    Swap(ID, ID),

    /// Drop(player,thing): Drop a held item into the current location.
    Drop(ID, ID),

    /// Kill the player/NPC with the given ID
    Kill(ID),

    /// Revie the player/NPC with the given ID
    Revive(ID),
}

/// Things that can happen in the game, to which rules, guards, and hooks can be attached
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Event {
    /// A game turn has elapsed
    Turn,

    /// EnterRoom(player, room): A player has entered (or wants to enter) a room
    EnterRoom(ID, ID),

    /// GetThing(player, thing): A player has gotten (or wants to get) a thing
    GetThing(ID, ID),

    /// ReadThing(player, thing): A player has read (or wants to read) a thing's
    /// Book prose.
    ReadThing(ID, ID),
}

/// The destination of a link.
#[derive(Clone, Debug)]
pub enum LinkDest {
    /// The link goes to another room.
    Room(ID),

    /// The link is a dead end.  The string is the prose to display to
    /// the user.
    DeadEnd(String)
}

/// ProseBuffer: A buffer for building up strings of prose.
///
/// Output prose is structured as sentences with block paragraphs.
/// This struct allows the client to build up output prose using
/// natural semantics.  The built-up string uses "conmark" syntax,
/// e.g., it is intended to be reformatted for display on a
/// console terminal.  See the "conmark" module for details.

#[derive(Default)]
pub struct ProseBuffer {
    buff: String,
}

impl ProseBuffer {
    /// Creates an empty buffer.  Prefer visual::prose().
    pub fn new() -> Self {
        Self {
            buff: String::new(),
        }
    }

    /// Adds a sentence to the buffer, separating it from previous text
    /// with a blank space if necessary.
    pub fn puts(&mut self, text: &str) {
        self.add_white_space_if_needed();
        self.put_raw(text);
    }

    /// Adds a paragraph break to the buffer.
    pub fn para(&mut self) {
        if self.buff.ends_with("||") {
            // Do nothing
        } else if self.buff.ends_with("|") {
            self.buff.push_str("|");
        } else {
            self.buff.push_str("||");
        }
    }

    /// Adds text to the buffer, with no special handling.
    pub fn put_raw(&mut self, text: &str) {
        self.buff.push_str(text);
    }

    /// Converts the buffer to a string.
    pub fn get(&self) -> String {
        self.buff.clone()
    }

    /// Adds white space before a new sentence, if needed.
    fn add_white_space_if_needed(&mut self) {
        let len = self.buff.len();

        if len > 0 {
            let last_char: char = self.buff.chars().nth(len - 1).unwrap();
            if !last_char.is_whitespace() && last_char != '|' {
                self.buff.push('\n');
            }
        }
    }
}
