//! The Main Application Library

mod conmark;
#[macro_use]
mod console;
mod debug;
mod entity;
mod player_control;
mod rule;
mod scenario;
mod types;
mod visual;
mod world;

use crate::world::*;

/// Runs the program.
pub fn run() {
    // FIRST, create the game world.
    let mut the_world: World = scenario::build();
    let world = &mut the_world;

    // NEXT, Print the introduction
    print_introduction(world);

    // NEXT, enter the game loop.
    loop {
        // FIRST, get the user's input
        let cmd = console::get_command(">");

        // NEXT, let the player do what he does.
        player_control::system(world, &cmd);

        // NEXT, handle rules
        rule::system(world);

        // NEXT, Increment the clock
        // TODO: Probably don't want to do this here.  Some commands should
        // take time, and some shouldn't.  This should probably be in the
        // player_control system.
        world.clock += 1;
    }
}

fn print_introduction(world: &World) {
    println!("Welcome to Advent!\n");

    let player = world.player();

    visual::room(world, player.location);
}
