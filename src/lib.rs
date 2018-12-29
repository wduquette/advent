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
#[allow(dead_code)] // Temporary
mod phys;
mod player_control;
mod rule_monitor;
mod scenario;
mod script;
mod types;
mod visual;
mod world;

use crate::world::*;

/// The main game object.  The Game contains the world, and any other data that
/// change when the world changes.
/// TODO: Ultimately, this will live somewhere else; and it will contain data that persists
/// from scene to scene.
pub struct Game {
    // THe current world
    world: World,

    // Undo information
    undo_info: Option<World>,
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
            world: scenario::build(),
            undo_info: None,
        }
    }

    /// Introduce the game: print a welcome message, and visualize the initial location
    pub fn introduce(&self) {
        println!("Welcome to Bonaventure!\n");

        visual::room(&self.world, phys::loc(&self.world, self.world.pid));
    }

    /// Restart the game: recreate the initial scenario.
    pub fn restart(&mut self) {
        self.world = scenario::build();
        self.undo_info = None;
        self.introduce();
    }

    /// Saves the world state for later undo.
    pub fn save_for_undo(&mut self, undo_info: World) {
        // At present, we save only one turn.
        self.undo_info = Some(undo_info);
    }

    /// Is there any undo info?
    pub fn has_undo(&self) -> bool {
        self.undo_info.is_some()
    }

    pub fn undo(&mut self) {
        assert!(self.has_undo(), "Cannot undo; no undo info");
        self.world = self.undo_info.take().unwrap();
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
        rule_monitor::system(&mut game.world);

        // NEXT, Increment the clock
        // TODO: Probably don't want to do this here.  Some commands should
        // take time, and some shouldn't.  This should probably be in the
        // player_control system.
        game.world.clock += 1;
    }
}
