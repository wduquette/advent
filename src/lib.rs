//! The Main Application Library

mod console;
mod debug;
mod player_control;
mod scenario;
mod trigger_control;
mod types;

use crate::types::*;

/// Runs the program.
pub fn run() {
    // FIRST, Print the introduction
    print_introduction();

    // NEXT, create the game world.
    let mut world_map: World = scenario::build_world();
    let world = &mut world_map;

    player_control::describe_player_location(world);

    // NEXT, enter the game loop.
    loop {
        // FIRST, get the user's input
        let cmd = console::get_command(">");

        // NEXT, let the player do what he does.
        player_control::system(world, &cmd);

        // NEXT, handle triggered events
        trigger_control::system(world);
        
        // NEXT, Increment the clock
        // TODO: Probably don't want to do this here.  Some commands should
        // take time, and some shouldn't.  This should probably be in the
        // player_control system.
        world.clock += 1;
    }
}

fn print_introduction() {
    println!("Welcome to Advent!\n");
}
