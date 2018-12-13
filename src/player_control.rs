//! The Player Control System

use crate::debug;
use crate::types::Dir::*;
use crate::types::*;
use crate::world::*;

/// An error result
type CmdResult = Result<(), String>;

/// The Player Control system.  Processes player commands.
pub fn system(world: &mut World, command: &str) {
    let tokens: Vec<&str> = command.split_whitespace().collect();

    let result = match tokens.as_slice() {
        ["n"] => cmd_go(world, &North),
        ["north"] => cmd_go(world, &North),
        ["s"] => cmd_go(world, &South),
        ["south"] => cmd_go(world, &South),
        ["e"] => cmd_go(world, &East),
        ["east"] => cmd_go(world, &East),
        ["w"] => cmd_go(world, &West),
        ["west"] => cmd_go(world, &West),
        ["help"] => cmd_help(world),
        ["look"] => cmd_look(world),
        ["i"] => cmd_inventory(world),
        ["invent"] => cmd_inventory(world),
        ["inventory"] => cmd_inventory(world),
        ["x", name] => cmd_examine(world, name),
        ["examine", name] => cmd_examine(world, name),
        ["get", name] => cmd_get(world, name),
        ["drop", name] => cmd_drop(world, name),
        ["exit"] => cmd_quit(world),
        ["quit"] => cmd_quit(world),

        // Debugging
        ["dump", id_arg] => cmd_dump(world, id_arg),
        ["dump"] => cmd_dump_world(world),
        ["list"] => cmd_list(world),

        // Error
        _ => Err("I don't understand.".into()),
    };

    if let Err(msg) = result {
        println!("{}\n", msg);
    }
}

// User Commands

/// Move the player in the given direction
fn cmd_go(world: &mut World, dir: &Dir) -> CmdResult {
    let here = world.loc(world.pid);
    if let Some(dest) = world.follow(here, &dir) {
        world.set_location(world.pid, dest);
        let seen = world.attrs.contains(&Attr::Seen(dest));
        describe_player_location(world, seen);
        world.attrs.insert(Attr::Seen(dest));
        Ok(())
    } else {
        Err("You can't go that way.".into())
    }
}

/// Display basic help, i.e., what commands are available.
fn cmd_help(_world: &World) -> CmdResult {
    println!(
        "\
You've got the usual commands: n, s, e, w, look, get, drop, quit.
You know.  Like that.
    "
    );

    Ok(())
}

/// Re-describe the current location.
fn cmd_look(world: &World) -> CmdResult {
    describe_player_location(world, false);
    Ok(())
}

/// Re-describe the current location.
fn cmd_inventory(world: &World) -> CmdResult {
    let pid = world.pid;
    let inv = &world.entities[pid].inventory.as_ref().unwrap();

    if inv.things.is_empty() {
        println!("You aren't carrying anything.\n");
    } else {
        println!("You have: {}.\n", invent_list(world, pid));
    }
    Ok(())
}

/// Describe a thing in the current location.
fn cmd_examine(world: &World, name: &str) -> CmdResult {
    if let Some(id) = find_visible_thing(world, name) {
        println!("{}\n", world.prose(id));
        Ok(())
    } else {
        Err("You don't see any such thing.".into())
    }
}

/// Gets a thing from the location's inventory.
fn cmd_get(world: &mut World, name: &str) -> CmdResult {
    let loc = here(world);
    if find_in_inventory(world, world.pid, name).is_some() {
        Err("You already have it.".into())
    } else if find_in_scenery(world, loc, name).is_some() {
        Err("You can't take that!".into())
    } else if let Some(id) = find_in_inventory(world, loc, name) {
        world.take_out(id, loc);
        world.put_in(id, world.pid);
        println!("Taken.\n");
        Ok(())
    } else {
        Err("You don't see any such thing.".into())
    }
}

/// Drops a thing you're carrying
fn cmd_drop(world: &mut World, name: &str) -> CmdResult {
    let loc = here(world);
    if let Some(id) = find_in_inventory(world, world.pid, name) {
        world.take_out(id, world.pid);
        world.put_in(id, loc);
        println!("Dropped.\n");
        Ok(())
    } else if find_visible_thing(world, name).is_some() {
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

// Debugging commands

/// Dump information about the given entity, provided the ID string is valid.
fn cmd_dump(world: &World, id_arg: &str) -> CmdResult {
    let id = parse_id(world, id_arg)?;
    debug::dump_entity(world, id);
    Ok(())
}

/// Dump information about all entities.
fn cmd_dump_world(world: &World) -> CmdResult {
    debug::dump_world(world);
    Ok(())
}

/// List all of the available entities.
fn cmd_list(world: &World) -> CmdResult {
    debug::list_world(world);
    Ok(())
}

//-------------------------------------------------------------------------
// Actions
//
// These functions are used to implement the above commands.

/// Describe the player's current location.
pub fn describe_player_location(world: &World, brief: bool) {
    let loc = world.loc(world.pid);

    // FIRST, display the room's description
    if brief {
        println!("{}\n", world.name(loc));
    } else {
        println!("{}\n{}\n", world.name(loc), world.prose(loc));
    }

    // NEXT, list any objects in the room's inventory.  (We don't list
    // scenary; presumably that's in the description.)
    if let Some(inv) = &world.entities[loc].inventory {
        if !inv.things.is_empty() {
            println!("You see: {}.\n", invent_list(world, loc));
        }
    }
}

//-------------------------------------------------------------------------
// Parsing Tools

/// Parse a token as an entity ID, return an error result on failure.
fn parse_id(world: &World, token: &str) -> Result<ID, String> {
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

/// Find a visible thing: something you're carrying, or that's here in this location.
fn find_visible_thing(world: &World, name: &str) -> Option<ID> {
    let loc = here(world);

    if let Some(id) = find_in_inventory(world, world.pid, name) {
        return Some(id);
    }

    if let Some(id) = find_in_inventory(world, loc, name) {
        return Some(id);
    }

    if let Some(id) = find_in_scenery(world, loc, name) {
        return Some(id);
    }

    None
}

fn find_in_inventory(world: &World, loc: ID, name: &str) -> Option<ID> {
    if let Some(inv) = &world.entities[loc].inventory {
        for id in &inv.things {
            if world.name(*id) == name {
                return Some(*id);
            }
        }
    }

    None
}

fn find_in_scenery(world: &World, loc: ID, name: &str) -> Option<ID> {
    for id in 1..world.entities.len() {
        if world.is_scenery(id) && world.loc(id) == loc && world.name(id) == name {
            return Some(id);
        }
    }

    None
}

fn here(world: &World) -> ID {
    world.loc(world.pid)
}

//-------------------------------------------------------------------------
// Display Tools

/// List the names of the entities, separated by commas.
/// TODO: This could probably be done with map and some kind of join function.
/// However, it seems that "join" is available in the nightly.
fn invent_list(world: &World, loc: ID) -> String {
    let mut list = String::new();

    if let Some(inv) = &world.entities[loc].inventory {
        for id in &inv.things {
            if !list.is_empty() {
                list.push_str(", ");
            }
            list.push_str(world.name(*id));
        }
    }

    list
}
