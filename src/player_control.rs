//! The Player Control System

use crate::entity::inventory::InventoryComponent;
use crate::entity::ID;
use self::Status::*;
use crate::command;
use crate::command::Command;
use crate::debug;
use crate::entity::player::PlayerView;
use crate::types::Flag::*;
use crate::types::Dir::*;
use crate::types::*;
use crate::visual;
use crate::world::*;
use crate::Game;

/// A status result.  Indicates the general category of the change.
#[derive(Copy, Clone, Debug)]
enum Status {
    /// Normal response: the world has been updated, and the change can be undone.
    Normal,

    /// Restart response; the game should be restarted from scratch.
    Restart,

    /// Undo the last command (plus anything that happened after, e.g., rule firings)
    Undo,
}

/// An error result
type CmdResult = Result<Status, String>;

/// The Player Control system.  Processes player commands.
pub fn system(game: &mut Game, input: &str) {
    // FIRST, get the current game state, for later undo.
    let undo_info = game.world.clone();

    // NEXT, get the player.  We'll save any changes at the end.
    let player = &mut game.world.player();

    // NEXT, handle the input
    let result = handle_input(game, player, input);
    match result {
        Err(msg) => visual::error(&msg),
        Ok(Normal) => {
            player.save(&mut game.world);
            game.save_for_undo(undo_info);
        }
        Ok(Restart) => game.restart(),
        Ok(Undo) => game.undo(),
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
        ["undo"] => cmd_undo(game),
        ["restart"] => cmd_restart(),
        ["quit"] => cmd_quit(),

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
    if let Some(dest) = world.follow(player.location(), dir) {
        player.set_location(dest);

        if !player.flag_set.has(Seen(dest)) {
            visual::room(world, dest);
        } else {
            visual::room_brief(world, dest);
        }

        player.flag_set.set(Seen(dest));
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
    ",
    );

    Ok(Normal)
}

/// Re-describe the current location.
fn cmd_look(world: &World, player: &PlayerView) -> CmdResult {
    visual::room(world, player.location());
    Ok(Normal)
}

/// Display the player's inventory.
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
        let thingv = world.as_thing(id);

        // If it has no prose, it can't be read
        if !world.is_book(id) {
            return Err("You can't read that.".into());
        }

        // If he's holding it, or it's scenery, then he can read it.
        if thingv.thing.location == player.id || thingv.flag_set.has(Scenery) {
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
    let roomv = world.as_room(player.location());

    if !roomv.flag_set.has(HasWater) {
        return Err("That'd be a neat trick.".into());
    }

    visual::prose("You wash your hands in the water.")
        .when(player.flag_set.has(DirtyHands), "They look much cleaner now.")
        .para();
    player.flag_set.unset(DirtyHands);

    Ok(Normal)
}

/// Gets a thing from the location's inventory.
fn cmd_get(world: &mut World, player: &mut PlayerView, name: &str) -> CmdResult {
    let roomv = &mut world.as_room(player.location());

    // Does he already have it?
    if find_in_inventory(world, &player.inventory, name).is_some() {
        return Err("You already have it.".into());
    }

    if let Some(id) = find_in_inventory(world, &roomv.inventory, name) {
        let thingv = &mut world.as_thing(id);
        if thingv.flag_set.has(Scenery) {
            return Err("You can't take that!".into());
        }

        // Get the thing.
        // TODO: use take_out, put_in
        roomv.inventory.remove(thingv.id);
        player.inventory.add(thingv.id);
        thingv.thing.location = player.id;

        roomv.save(world);
        thingv.save(world);

        visual::act("Taken.");
        Ok(Normal)
    } else {
        Err("You don't see any such thing.".into())
    }
}

/// Drops a thing you're carrying
fn cmd_drop(world: &mut World, player: &mut PlayerView, name: &str) -> CmdResult {
    let roomv = &mut world.as_room(player.location());

    if let Some(id) = find_in_inventory(world, &player.inventory, name) {
        let thingv = &mut world.as_thing(id);

        // TODO: use take_out and put_int
        player.inventory.remove(thingv.id);
        roomv.inventory.add(thingv.id);
        thingv.thing.location = roomv.id;

        roomv.save(world);
        thingv.save(world);

        visual::act("Dropped.");
        Ok(Normal)
    } else if find_in_inventory(world, &roomv.inventory, name).is_some() {
        Err("You aren't carrying that.".into())
    } else {
        Err("You don't see any such thing.".into())
    }
}

/// Undo the last command the game
fn cmd_undo(game: &mut Game) -> CmdResult {
    if game.has_undo() {
        visual::act("Undone.");
        Ok(Undo)
    } else {
        Err("Nothing to undo.".into())
    }
}

/// Restart the game
fn cmd_restart() -> CmdResult {
    visual::act("Restarting...");
    Ok(Restart)
}

/// Quit the game.
fn cmd_quit() -> CmdResult {
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

    if !world.tags.contains_key(&id) {
        return Err(format!("Not an ID: {}", token));
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
    if world.is_room(id) {
        visual::room(world, id);
        Ok(Normal)
    } else {
        Err(format!("Entity {} is not a room.", id))
    }
}

/// Examine the thing fully, as though the player could see it.
fn cmd_debug_examine(world: &World, id_arg: &str) -> CmdResult {
    let id = parse_id(world, id_arg)?;
    if world.is_thing(id) {
        visual::thing(world, id);
        Ok(Normal)
    } else {
        Err(format!("Entity {} is not a thing.", id))
    }
}

/// Take the player to the location.
fn cmd_debug_go(world: &World, player: &mut PlayerView, id_arg: &str) -> CmdResult {
    let id = parse_id(world, id_arg)?;
    if world.is_room(id) {
        player.set_location(id);
        visual::room(world, id);
        Ok(Normal)
    } else {
        Err(format!("Entity {} is not a room.", id))
    }
}

//-------------------------------------------------------------------------
// Parsing Tools

/// Looks for a thing with the given name in the given inventory list.
fn find_in_inventory(world: &World, inventory: &InventoryComponent, noun: &str) -> Option<ID> {
    for id in inventory.iter() {
        let thingv = world.as_thing(*id);
        if thingv.thing.noun == noun {
            return Some(thingv.id);
        }
    }

    None
}

/// Find a visible thing: something you're carrying, or that's here in this location.
fn find_visible_thing(world: &World, player: &PlayerView, noun: &str) -> Option<ID> {
    // FIRST, does the player have it?
    if let Some(id) = find_in_inventory(world, &player.inventory, noun) {
        return Some(id);
    }

    // NEXT, is it in this room?
    let roomv = &world.as_room(player.location());

    if let Some(id) = find_in_inventory(world, &roomv.inventory, noun) {
        return Some(id);
    }

    None
}
