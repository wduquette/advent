//! # Visual system
//
// This module centralizes all of the code that turns game entities into prose for display
// to the user.  It depends on the output routines from the `console` module, and
// indirectly on the formatting and wrapping routines in the `conmark` module.
//
// In a normal ECS architecture, the visual system is called in the game loop after the
// physics system to render the current scene.  In a text adventure, text is displayed at
// appropriate moments in processing; thus, this module is called as needed, rather than
// doing its work all at once.

use crate::console::para;
use crate::world::World;
use crate::types::*;
use crate::types::flags::Flag::*;

//-----------------------------------------------------------------------------
// Types

/// Level of Detail
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Detail {
    Full,
    Brief,
}

//-----------------------------------------------------------------------------
// Basic Messages
//
// At present these are all treated like "para"; but this gives the opportunity
// to distinguish them at some future time.

/// Outputs a player action, e.g., "Taken."
pub fn act(msg: &str) {
    para(msg);
}

/// Outputs an error message.
pub fn error(msg: &str) {
    para(msg);
}

/// Outputs information (e.g., help)
pub fn info(msg: &str) {
    para(msg);
}

pub fn prose(text: &str) -> Buffer {
    Buffer::new().add(text)
}

//-----------------------------------------------------------------------------
// Room Visuals

/// Outputs a full description of a room.
///
/// A full description includes the room's name, visual, and any things that are present.
pub fn room(world: &World, id: ID) {
    print_room(world, id, Detail::Full);
}

/// Outputs a brief description of a room.
///
/// A description includes the room's detailed visual.
pub fn room_brief(world: &World, id: ID) {
    print_room(world, id, Detail::Brief);
}

/// Outputs a full or brief description of a room.
///
/// * A full description includes the room's name, visual, and any things that are present.
/// * A brief description omits the visual; it's used for rooms that the player has visited
///   before.
fn print_room(world: &World, id: ID, detail: Detail) {
    let room = world.as_room(id);

    // FIRST, display the room's description
    if detail == Detail::Full {
        para!("{}|{}", room.name, room.visual);
    } else {
        para(&room.name);
    }

    // NEXT, list any objects in the room's inventory.  (We don't list
    // scenary; presumably that's in the description.)
    let list = invent_list(world, &room.inventory);

    if !list.is_empty() {
        para!("You see: {}.", list);
    }
}

//-----------------------------------------------------------------------------
// Thing Visuals

/// Outputs a description of a thing.
pub fn thing(world: &World, id: ID) {
    let thing = world.as_thing(id);

    // FIRST, display the thing's description
    para(&thing.visual);

    // TODO: eventually we will want to describe its contents, if it has
    // contents, or other changeable state.
}

/// Outputs the content of a book.
pub fn book(world: &World, id: ID) {
    let book = world.as_book(id);
    para(&book.text);
}

//-----------------------------------------------------------------------------
// Player Visuals

/// Outputs a visual of the player.
///
/// TODO: figure out how to handle optional content, e.g., dirty hands.
pub fn player(world: &World) {
    let player = world.player();

    let mut msg = String::new();
    msg.push_str(&player.visual);

    // TODO: This stuff is scenario-dependent.  There really ought to be
    // a mechanism for this.
    prose(&player.visual)
        .when(player.flags.has(DirtyHands), "Your hands are kind of dirty, though")
        .when(!player.flags.has(DirtyHands), "Plus, they're clean bits!")
        .para();
}

/// Outputs the player's inventory
pub fn player_inventory(world: &World) {
    let player = world.player();

    if player.inventory.is_empty() {
        para("You aren't carrying anything.");
    } else {
        para!("You have: {}.\n", invent_list(world, &player.inventory));
    }
}

/// List the names of the entities, separated by commas.  Omits scenery.
fn invent_list(world: &World, inventory: &Inventory) -> String {
    let mut list = String::new();

    for id in inventory {
        let thing = world.as_thing(*id);

        if !thing.flags.has(Scenery) {
            if !list.is_empty() {
                list.push_str(", ");
            }
            list.push_str(&thing.name);
        }
    }

    list
}

//-----------------------------------------------------------------------------
// Helpers

/// A buffer for building up text strings for output.
pub struct Buffer {
    buff: String,
}

impl Buffer {
    /// Creates an empty buffer.  Prefer visual::prose().
    pub fn new() -> Buffer {
        Buffer { buff: String::new() }
    }

    /// Adds a text string to the buffer, adding a blank if necessary.
    pub fn add(mut self, text: &str) -> Buffer {
        if !self.buff.is_empty() {
            self.buff.push(' ');
        }
        self.buff.push_str(text);
        self
    }

    /// Adds a text string only if the flag is true
    pub fn when(self, flag: bool, text: &str) -> Buffer {
        if flag {
            self.add(text)
        } else {
            self
        }
    }

    /// Outputs the constructed message as a para.
    pub fn para(self) {
        para(&self.buff);
    }
}
