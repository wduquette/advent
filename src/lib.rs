//! # Bonaventure: A Text Adventure Framework
/// Bonaventure is a simple text adventure framework.  At present, it is used to
/// implement a single game; see src/scenario.rs.  Eventually it might support
/// multiple games.

mod command;
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

/// The main game object.  The Game contains the world, and any other data that
/// change when the world changes.
/// TODO: Ultimately, this will live somewhere else; and it will contain data that persists
/// from scene to scene.
pub struct Game {
    world: World,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    /// Create the game object
    pub fn new() -> Game {
        Game {
            world: scenario::build()
        }
    }

    /// Introduce the game: print a welcome message, and visualize the initial location
    pub fn introduce(&self) {
        println!("Welcome to Bonaventure!\n");

        let player = self.world.player();

        visual::room(&self.world, player.location);
    }

    /// Restart the game: recreate the initial scenario.
    pub fn restart(&mut self) {
        self.world = scenario::build();
        self.introduce();
    }
}

/// Runs the program.
pub fn run() {
    // FIRST, create the game world.
    let mut game = Game::new();
    game.introduce();

    // NEXT, enter the game loop.
    let mut con = console::Console::new();

    loop {
        // FIRST, get the user's input
        let cmd = con.readline("> ");

        // NEXT, let the player do what he does.
        player_control::system(&mut game, &cmd);

        // NEXT, handle rules
        rule::system(&mut game.world);

        // NEXT, Increment the clock
        // TODO: Probably don't want to do this here.  Some commands should
        // take time, and some shouldn't.  This should probably be in the
        // player_control system.
        game.world.clock += 1;
    }
}
