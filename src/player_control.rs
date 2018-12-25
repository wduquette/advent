//! The Player Control System

use self::Status::*;
use crate::debug;
use crate::entity::PlayerView;
use crate::visual;
use crate::types::Dir::*;
use crate::types::flags::Flag::*;
use crate::types::*;
use crate::world::*;
use crate::command;
use crate::command::Command;
use crate::Game;

/// A status result on Ok(Normal)
#[derive(Copy,Clone,Debug)]
enum Status {
    Normal,
    Restart
}

/// An error result
type CmdResult = Result<Status, String>;

/// The Player Control system.  Processes player commands.
pub fn system(game: &mut Game, input: &str) {
    // FIRST, get the player.  We'll save any changes at the end.
    let player = &mut game.world.player();

    // NEXT, handle the input
    let result = handle_input(game, player, input);
    match result {
        Err(msg) => visual::error(&msg),
        Ok(Normal) => player.save(&mut game.world),
        Ok(Restart) => (),
    }
}

fn handle_input(game: &mut Game, player: &mut PlayerView, input: &str) -> CmdResult {
    // FIRST, parse the input.
    let cmd = command::parse(&game.world, input)?;

    if cmd.is_debug {
        handle_debug_command(game, player, &cmd)
    } else {
        handle_normal_command(game, player, &cmd)
    }
}

fn handle_normal_command(game: &mut Game, player: &mut PlayerView, cmd: &Command) -> CmdResult {
    let words: Vec<&str> = cmd.words.iter().map(|s| s.as_ref()).collect();
    let world = &mut game.world;

    // TODO: parser should handle two-word verb synonyms.
    match words.as_slice() {
        ["go", "north"] => cmd_go(world, player, North),
        ["north"] => cmd_go(world, player, North),
        ["go", "south"] => cmd_go(world, player, South),
        ["south"] => cmd_go(world, player, South),
        ["go", "east"] => cmd_go(world, player, East),
        ["east"] => cmd_go(world, player, East),
        ["go", "west"] => cmd_go(world, player, West),
        ["west"] => cmd_go(world, player, West),
        ["help"] => cmd_help(),
        ["look"] => cmd_look(world, player),
        ["inventory"] => cmd_inventory(world),
        ["examine", "self"] => cmd_examine_self(world),
        ["examine", "me"] => cmd_examine_self(world),
        ["examine", name] => cmd_examine(world, player, name),
        ["read", name] => cmd_read(world, player, name),
        ["get", name] => cmd_get(world, player, name),
        ["pick", "up", name] => cmd_get(world, player, name),
        ["drop", name] => cmd_drop(world, player, name),
        ["wash", "hands"] => cmd_wash_hands(world, player),
        ["wash", _] => Err("Whatever for?".into()),
        ["restart"] => cmd_restart(game),
        ["quit"] => cmd_quit(world),

        // Error
        _ => Err("I don't understand.".into()),
    }
}

fn handle_debug_command(game: &mut Game, player: &mut PlayerView, cmd: &Command) -> CmdResult {
    let words: Vec<&str> = cmd.words.iter().map(|s| s.as_ref()).collect();
    let world = &mut game.world;

    match words.as_slice() {
        ["list"] => cmd_debug_list(world),
        ["dump", id_arg] => cmd_debug_dump(world, id_arg),
        ["look", id_arg] => cmd_debug_look(world, id_arg),
        ["examine", id_arg] => cmd_debug_examine(world, id_arg),
        ["go", id_arg] => cmd_debug_go(world, player, id_arg),

        // Error
        _ => Err("I don't understand.".into()),
    }
}


// User Commands

/// Move the player in the given direction
fn cmd_go(world: &mut World, player: &mut PlayerView, dir: Dir) -> CmdResult {
    if let Some(dest) = world.follow(player.location, dir) {
        player.location = dest;

        if !player.flags.has(Seen(dest)) {
            visual::room(world, dest);
        } else {
            visual::room_brief(world, dest);
        }

        player.flags.set(Seen(dest));
        Ok(Normal)
    } else {
        Err("You can't go that way.".into())
    }
}

/// Display basic help, i.e., what commands are available.
fn cmd_help() -> CmdResult {
    visual::info(
        "\
You've got the usual commands: n, s, e, w, look, get, drop, quit.
You know.  Like that.
    "
    );

    Ok(Normal)
}

/// Re-describe the current location.
fn cmd_look(world: &World, player: &PlayerView) -> CmdResult {
    visual::room(world, player.location);
    Ok(Normal)
}

/// Re-describe the current location.
fn cmd_inventory(world: &World) -> CmdResult {
    visual::player_inventory(world);
    Ok(Normal)
}

/// Describe a thing in the current location.
fn cmd_examine(world: &World, player: &PlayerView, name: &str) -> CmdResult {
    if let Some(id) = find_visible_thing(world, player, name) {
        visual::thing(world, id);
        Ok(Normal)
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
        if thing.location == player.id || thing.flags.has(Scenery) {
            visual::book(world, id);
            Ok(Normal)
        } else {
            Err("You don't have it.".into())
        }
    } else {
        // It isn't here.
        Err("You don't see any such thing.".into())
    }
}

/// Describe a thing in the current location.
fn cmd_examine_self(world: &World) -> CmdResult {
    visual::player(world);

    Ok(Normal)
}

// TODO: As currently implemented, this should be a scenario command, not a
// built-in command.
fn cmd_wash_hands(world: &mut World, player: &mut PlayerView) -> CmdResult {
    let room = world.get(player.location).as_room();

    if !room.flags.has(HasWater) {
        return Err("That'd be a neat trick.".into());
    }

    visual::prose("You wash your hands in the water.")
        .when(player.flags.has(DirtyHands), "They look much cleaner now.")
        .para();
    player.flags.unset(DirtyHands);

    Ok(Normal)
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
        if thing.flags.has(Scenery) {
            return Err("You can't take that!".into());
        }

        // Get the thing.
        room.inventory.remove(&thing.id);
        player.inventory.insert(thing.id);
        thing.location = player.id;

        room.save(world);
        thing.save(world);

        visual::act("Taken.");
        Ok(Normal)
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

        visual::act("Dropped.");
        Ok(Normal)
    } else if find_in_inventory(world, &room.inventory, name).is_some() {
        Err("You aren't carrying that.".into())
    } else {
        Err("You don't see any such thing.".into())
    }
}

/// Restart the game
fn cmd_restart(game: &mut Game) -> CmdResult {
    visual::act("Restarting...");
    game.restart();
    Ok(Restart)
}

/// Quit the game.
fn cmd_quit(_world: &World) -> CmdResult {
    visual::act("Bye, then.");
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
    Ok(Normal)
}

/// Dump information about the given entity, provided the ID string is valid.
fn cmd_debug_dump(world: &World, id_arg: &str) -> CmdResult {
    let id = parse_id(world, id_arg)?;
    debug::dump_entity(world, id);
    Ok(Normal)
}

/// Describe the room as though the player were in it.
fn cmd_debug_look(world: &World, id_arg: &str) -> CmdResult {
    let id = parse_id(world, id_arg)?;
    if world.get(id).is_room() {
        visual::room(world, id);
        Ok(Normal)
    } else {
        Err(format!("Entity {} is not a room.", id))
    }
}

/// Examine the thing fully, as though the player could see it.
fn cmd_debug_examine(world: &World, id_arg: &str) -> CmdResult {
    let id = parse_id(world, id_arg)?;
    if world.get(id).is_thing() {
        visual::thing(world, id);
        Ok(Normal)
    } else {
        Err(format!("Entity {} is not a thing.", id))
    }
}

/// Take the player to the location.
fn cmd_debug_go(world: &World, player: &mut PlayerView, id_arg: &str) -> CmdResult {
    let id = parse_id(world, id_arg)?;
    if world.get(id).is_room() {
        player.location = id;
        visual::room(world, id);
        Ok(Normal)
    } else {
        Err(format!("Entity {} is not a room.", id))
    }
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
