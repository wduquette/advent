//! The Player Control System

use crate::debug;
use crate::types::Dir::*;
use crate::types::*;
use crate::world::*;

type CmdResult = Result<(), String>;

pub fn system(world: &mut World, command: &str) {
    let tokens: Vec<&str> = command.split_whitespace().collect();

    let result = match tokens[0] {
        "n" => cmd_go_dir(world, &tokens, North),
        "s" => cmd_go_dir(world, &tokens, South),
        "e" => cmd_go_dir(world, &tokens, East),
        "w" => cmd_go_dir(world, &tokens, West),
        "help" => cmd_help(world, &tokens),
        "look" => cmd_look(world, &tokens),
        "quit" => cmd_quit(world, &tokens),

        // Debugging
        "dump" => cmd_dump(world, &tokens),
        "list" => cmd_list(world, &tokens),
        _ => Err("I don't understand.".into()),
    };

    if let Err(msg) = result {
        println!("{}", msg);
    }
}

// User Commands

fn cmd_go_dir(world: &mut World, tokens: &[&str], dir: Dir) -> CmdResult {
    require_args(tokens, 0, 0)?;
    let here = world.loc(world.player);
    if let Some(dest) = world.follow(here, dir) {
        set_player_location(world, dest);
        describe_player_location(world);
        Ok(())
    } else {
        Err("You can't go that way.".into())
    }
}

fn cmd_help(_world: &World, _tokens: &[&str]) -> CmdResult {
    println!(
        "\
You've got the usual commands: n, s, e, w, look, get, drop, quit.
You know.  Like that.
    "
    );

    Ok(())
}

fn cmd_look(world: &World, tokens: &[&str]) -> CmdResult {
    require_args(tokens, 0, 0)?;
    describe_player_location(world);
    Ok(())
}

fn cmd_quit(_world: &World, _tokens: &[&str]) -> CmdResult {
    println!("Bye, then.");
    ::std::process::exit(0);
}

// Debugging commands

fn cmd_dump(world: &World, tokens: &[&str]) -> CmdResult {
    require_args(tokens, 0, 1)?;

    // FIRST, handle general case
    if tokens.len() == 1 {
        debug::dump_world(world);
        return Ok(());
    }

    // NEXT, get the entity ID
    let id = parse_id(world, tokens[1])?;
    debug::dump_entity(world, id);
    Ok(())
}

fn cmd_list(world: &World, _tokens: &[&str]) -> CmdResult {
    debug::list_world(world);
    Ok(())
}

// Tools

fn require_args(tokens: &[&str], min: usize, max: usize) -> CmdResult {
    let n = tokens.len() - 1;

    if n < min || n > max {
        Err("I don't understand".into())
    } else {
        Ok(())
    }
}

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

pub fn describe_player_location(world: &World) {
    describe_location(world, world.loc(world.player));
}

fn describe_location(world: &World, loc: ID) {
    let prose = world.entities[loc]
        .prose
        .as_ref()
        .expect(&format!("Entity has no prose: {}", loc));

    println!("{}\n{}\n",
        world.entities[loc].name,
        prose.text);
}

fn set_player_location(world: &mut World, dest: ID) {
    world.entities[world.player].loc = Some(dest);
}
