//! The Player Control System

use crate::debug;
use crate::entity::PlayerView;
use crate::types::Detail::*;
use crate::types::Dir::*;
use crate::types::Var::*;
use crate::types::*;
use crate::world::*;

/// An error result
type CmdResult = Result<(), String>;

/// The Player Control system.  Processes player commands.
pub fn system(world: &mut World, command: &str) {
    let player = &mut world.get(world.pid).as_player();

    let tokens: Vec<&str> = command.split_whitespace().collect();

    // TODO: Map synonyms, remove punctuation, before pattern matching

    let result = match tokens.as_slice() {
        ["n"] => cmd_go(world, player, North),
        ["north"] => cmd_go(world, player, North),
        ["s"] => cmd_go(world, player, South),
        ["south"] => cmd_go(world, player, South),
        ["e"] => cmd_go(world, player, East),
        ["east"] => cmd_go(world, player, East),
        ["w"] => cmd_go(world, player, West),
        ["west"] => cmd_go(world, player, West),
        ["help"] => cmd_help(),
        ["look"] => cmd_look(world, player),
        ["i"] => cmd_inventory(world, player),
        ["invent"] => cmd_inventory(world, player),
        ["inventory"] => cmd_inventory(world, player),
        ["x", "self"] => cmd_examine_self(world, player),
        ["x", "me"] => cmd_examine_self(world, player),
        ["x", name] => cmd_examine(world, player, name),
        ["examine", "self"] => cmd_examine_self(world, player),
        ["examine", "me"] => cmd_examine_self(world, player),
        ["examine", name] => cmd_examine(world, player, name),
        ["read", name] => cmd_read(world, player, name),
        ["get", name] => cmd_get(world, player, name),
        ["drop", name] => cmd_drop(world, player, name),
        ["wash", "hands"] => cmd_wash_hands(world, player),
        ["wash", _] => Err("Whatever for?".into()),
        ["exit"] => cmd_quit(world),
        ["quit"] => cmd_quit(world),

        // Debugging
        ["!list"] => cmd_debug_list(world),
        ["!dump", id_arg] => cmd_debug_dump(world, id_arg),
        ["!look", id_arg] => cmd_debug_look(world, id_arg),
        ["!examine", id_arg] => cmd_debug_examine(world, id_arg),
        ["!x", id_arg] => cmd_debug_examine(world, id_arg),
        ["!go", id_arg] => cmd_debug_go(world, player, id_arg),

        // Error
        _ => Err("I don't understand.".into()),
    };

    // NEXT, handle the result
    if let Err(msg) = result {
        println!("{}\n", msg);
    } else {
        player.save(world);
    }
}

// User Commands

/// Move the player in the given direction
fn cmd_go(world: &mut World, player: &mut PlayerView, dir: Dir) -> CmdResult {
    if let Some(dest) = world.follow(player.location, dir) {
        player.location = dest;

        if !player.vars.contains(&Seen(dest)) {
            describe_room(world, dest, Full);
        } else {
            describe_room(world, dest, Brief);
        }

        player.vars.insert(Seen(dest));
        Ok(())
    } else {
        Err("You can't go that way.".into())
    }
}

/// Display basic help, i.e., what commands are available.
fn cmd_help() -> CmdResult {
    println!(
        "\
You've got the usual commands: n, s, e, w, look, get, drop, quit.
You know.  Like that.
    "
    );

    Ok(())
}

/// Re-describe the current location.
fn cmd_look(world: &World, player: &PlayerView) -> CmdResult {
    describe_room(world, player.location, Full);
    Ok(())
}

/// Re-describe the current location.
fn cmd_inventory(world: &World, player: &PlayerView) -> CmdResult {
    if player.inventory.is_empty() {
        println!("You aren't carrying anything.\n");
    } else {
        println!("You have: {}.\n", invent_list(world, &player.inventory));
    }
    Ok(())
}

/// Describe a thing in the current location.
fn cmd_examine(world: &World, player: &PlayerView, name: &str) -> CmdResult {
    if let Some(id) = find_visible_thing(world, player, name) {
        describe_thing(world, id);
        Ok(())
    } else {
        Err("You don't see any such thing.".into())
    }
}

/// Read a thing in the current location.
fn cmd_read(world: &World, player: &PlayerView, name: &str) -> CmdResult {
    if let Some(id) = find_visible_thing(world, player, name) {
        let thing = world.get(id).as_thing();

        // If it has no prose, it can't be read
        if !world.get(id).is_book() {
            return Err("You can't read that.".into());
        }

        // If he's holding it, or it's scenery, then he can read it.
        if thing.location == player.id || thing.vars.contains(&Scenery) {
            let book = world.get(id).as_book();
            println!("{}\n", book.text);
            Ok(())
        } else {
            Err("You don't have it.".into())
        }
    } else {
        // It isn't here.
        Err("You don't see any such thing.".into())
    }
}

/// Describe a thing in the current location.
fn cmd_examine_self(_world: &World, player: &PlayerView) -> CmdResult {
    let mut msg = String::new();

    msg.push_str(&player.visual);

    if player.vars.contains(&DirtyHands) {
        msg.push_str(" Your hands are kind of dirty, though.");
    } else {
        msg.push_str(" Plus, they're clean bits!");
    }
    println!("{}\n", msg);

    Ok(())
}

// TODO: As currently implemented, this should be a scenario command, not a
// built-in command.
fn cmd_wash_hands(world: &mut World, player: &mut PlayerView) -> CmdResult {
    let room = world.get(player.location).as_room();

    if !room.vars.contains(&HasWater) {
        return Err("That'd be a neat trick.".into());
    }

    let mut msg = String::new();
    msg.push_str("You wash your hands in the water.");

    if player.vars.contains(&DirtyHands) {
        msg.push_str(" They look much cleaner.");
        player.vars.remove(&DirtyHands);
    }

    println!("{}\n", msg);

    Ok(())
}

/// Gets a thing from the location's inventory.
fn cmd_get(world: &mut World, player: &mut PlayerView, name: &str) -> CmdResult {
    let room = &mut world.get(player.location).as_room();

    // Does he already have it?
    if find_in_inventory(world, &player.inventory, name).is_some() {
        return Err("You already have it.".into());
    }

    if let Some(id) = find_in_inventory(world, &room.inventory, name) {
        let thing = &mut world.get(id).as_thing();
        if thing.vars.contains(&Scenery) {
            return Err("You can't take that!".into());
        }

        // Get the thing.
        room.inventory.remove(&thing.id);
        player.inventory.insert(thing.id);
        thing.location = player.id;

        room.save(world);
        thing.save(world);

        println!("Taken.\n");
        Ok(())
    } else {
        Err("You don't see any such thing.".into())
    }
}

/// Drops a thing you're carrying
fn cmd_drop(world: &mut World, player: &mut PlayerView, name: &str) -> CmdResult {
    let room = &mut world.get(player.location).as_room();

    if let Some(id) = find_in_inventory(world, &player.inventory, name) {
        let thing = &mut world.get(id).as_thing();

        player.inventory.remove(&thing.id);
        room.inventory.insert(thing.id);
        thing.location = room.id;

        room.save(world);
        thing.save(world);

        println!("Dropped.\n");
        Ok(())
    } else if find_in_inventory(world, &room.inventory, name).is_some() {
        Err("You aren't carrying that.".into())
    } else {
        Err("You don't see any such thing.".into())
    }
}

/// Quit the game.
fn cmd_quit(_world: &World) -> CmdResult {
    println!("Bye, then.");
    ::std::process::exit(0);
}

//------------------------------------------------------------------------------
// Debugging commands

/// Parse a token as an entity tag or ID, return an ID on success and
/// an error result on failure.
fn parse_id(world: &World, token: &str) -> Result<ID, String> {
    // FIRST, is the token a tag?
    if let Some(id) = world.lookup_id(token) {
        return Ok(id);
    }

    // NEXT, is it an explicit ID?
    let id = match token.parse() {
        Ok(id) => id,
        Err(_) => {
            return Err(format!("Not an ID: {}", token));
        }
    };

    if id >= world.entities.len() {
        return Err(format!("Out of range: {}", token));
    }

    Ok(id)
}


/// List all of the available entities.
fn cmd_debug_list(world: &World) -> CmdResult {
    debug::list_world(world);
    Ok(())
}

/// Dump information about the given entity, provided the ID string is valid.
fn cmd_debug_dump(world: &World, id_arg: &str) -> CmdResult {
    let id = parse_id(world, id_arg)?;
    debug::dump_entity(world, id);
    Ok(())
}

/// Describe the room as though the player were in it.
fn cmd_debug_look(world: &World, id_arg: &str) -> CmdResult {
    let id = parse_id(world, id_arg)?;
    if world.get(id).is_room() {
        describe_room(world, id, Full);
        Ok(())
    } else {
        Err(format!("Entity {} is not a room.", id))
    }
}

/// Examine the thing fully, as though the player could see it.
fn cmd_debug_examine(world: &World, id_arg: &str) -> CmdResult {
    let id = parse_id(world, id_arg)?;
    if world.get(id).is_thing() {
        describe_thing(world, id);
        Ok(())
    } else {
        Err(format!("Entity {} is not a thing.", id))
    }
}

/// Take the player to the location.
fn cmd_debug_go(world: &World, player: &mut PlayerView, id_arg: &str) -> CmdResult {
    let id = parse_id(world, id_arg)?;
    if world.get(id).is_room() {
        player.location = id;
        describe_room(world, id, Full);
        Ok(())
    } else {
        Err(format!("Entity {} is not a room.", id))
    }
}

//-------------------------------------------------------------------------
// Actions
//
// These functions are used to implement the above commands.

/// Describe the room.
pub fn describe_room(world: &World, id: ID, detail: Detail) {
    let room = world.get(id).as_room();

    // FIRST, display the room's description
    if detail == Full {
        println!("{}\n{}\n", room.name, room.visual);
    } else {
        println!("{}\n", room.name);
    }

    // NEXT, list any objects in the room's inventory.  (We don't list
    // scenary; presumably that's in the description.)
    let list = invent_list(world, &room.inventory);

    if !list.is_empty() {
        println!("You see: {}.\n", list);
    }
}

/// Describe the location.
pub fn describe_thing(world: &World, id: ID) {
    let thing = world.get(id).as_thing();

    // FIRST, display the thing's description
    println!("{}\n", thing.visual);

    // TODO: eventually we will want to describe its contents, if it has
    // contents and its open.
}

//-------------------------------------------------------------------------
// Parsing Tools

/// Looks for a thing with the given name in the given inventory list.
fn find_in_inventory(world: &World, inventory: &Inventory, name: &str) -> Option<ID> {
    for id in inventory {
        let thing = world.get(*id).as_thing();
        if thing.name == name {
            return Some(thing.id);
        }
    }

    None
}

/// Find a visible thing: something you're carrying, or that's here in this location.
fn find_visible_thing(world: &World, player: &PlayerView, name: &str) -> Option<ID> {
    // FIRST, does the player have it?
    if let Some(id) = find_in_inventory(world, &player.inventory, name) {
        return Some(id);
    }

    // NEXT, is it in this room?
    let room = &world.get(player.location).as_room();

    if let Some(id) = find_in_inventory(world, &room.inventory, name) {
        return Some(id);
    }

    None
}

//-------------------------------------------------------------------------
// Display Tools

/// List the names of the entities, separated by commas.  Omits scenery.
fn invent_list(world: &World, inventory: &Inventory) -> String {
    let mut list = String::new();

    for id in inventory {
        let thing = world.get(*id).as_thing();

        if !thing.vars.contains(&Scenery) {
            if !list.is_empty() {
                list.push_str(", ");
            }
            list.push_str(&thing.name);
        }
    }

    list
}
