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
        &["n"] => cmd_go(world, North),
        &["north"] => cmd_go(world, North),
        &["s"] => cmd_go(world, South),
        &["south"] => cmd_go(world, South),
        &["e"] => cmd_go(world, East),
        &["east"] => cmd_go(world, East),
        &["w"] => cmd_go(world, West),
        &["west"] => cmd_go(world, West),
        &["help"] => cmd_help(world),
        &["look"] => cmd_look(world),
        &["exit"] => cmd_quit(world),
        &["quit"] => cmd_quit(world),

        // Debugging
        &["dump", id_arg] => cmd_dump(world, id_arg),
        &["dump"] => cmd_dump_world(world),
        &["list"] => cmd_list(world),

        // Error
        _ => Err("I don't understand.".into()),
    };

    if let Err(msg) = result {
        println!("{}\n", msg);
    }
}

// User Commands

/// Move the player in the given direction
fn cmd_go(world: &mut World, dir: Dir) -> CmdResult {
    let here = world.loc(world.player);
    if let Some(dest) = world.follow(here, dir) {
        world.set_location(world.player, dest);
        describe_player_location(world);
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
    describe_player_location(world);
    Ok(())
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
pub fn describe_player_location(world: &World) {
    let loc = world.loc(world.player);

    // FIRST, display the room's description
    println!("{}\n{}\n", world.name(loc), world.prose(loc));

    // NEXT, list any objects in the room's inventory.  (We don't list
    // scenary; presumably that's in the description.)
    if let Some(inv) = &world.entities[loc].inventory {
        if inv.things.len() > 0 {
            println!("You see: {}.\n", comma_list(world, &inv.things));
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

//-------------------------------------------------------------------------
// Display Tools

/// List the names of the entities, separated by commas.
/// TODO: This could probably be done with map and some kind of join function.
/// However, it seems that "join" is available in the nightly.
fn comma_list(world: &World, ids: &[ID]) -> String {
    let mut list = String::new();

    for id in ids {
        if !list.is_empty() {
            list.push_str(", ");
        }
        list.push_str(world.name(*id));
    }

    list
}
