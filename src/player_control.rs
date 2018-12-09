//! The Player Control System

use crate::debug;
use crate::types::Dir::*;
use crate::types::*;
use crate::world::*;

type CmdResult = Result<(), String>;

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

fn cmd_go(world: &mut World, dir: Dir) -> CmdResult {
    let here = world.loc(world.player);
    if let Some(dest) = world.follow(here, dir) {
        set_player_location(world, dest);
        describe_player_location(world);
        Ok(())
    } else {
        Err("You can't go that way.".into())
    }
}

fn cmd_help(_world: &World) -> CmdResult {
    println!(
        "\
You've got the usual commands: n, s, e, w, look, get, drop, quit.
You know.  Like that.
    "
    );

    Ok(())
}

fn cmd_look(world: &World) -> CmdResult {
    describe_player_location(world);
    Ok(())
}

fn cmd_quit(_world: &World) -> CmdResult {
    println!("Bye, then.");
    ::std::process::exit(0);
}

// Debugging commands

fn cmd_dump(world: &World, id_arg: &str) -> CmdResult {
    let id = parse_id(world, id_arg)?;
    debug::dump_entity(world, id);
    Ok(())
}

fn cmd_dump_world(world: &World) -> CmdResult {
    debug::dump_world(world);
    Ok(())
}

fn cmd_list(world: &World) -> CmdResult {
    debug::list_world(world);
    Ok(())
}

// Tools

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

    println!("{}\n{}\n", world.entities[loc].name, prose.text);
}

fn set_player_location(world: &mut World, dest: ID) {
    world.entities[world.player].loc = Some(dest);
}
