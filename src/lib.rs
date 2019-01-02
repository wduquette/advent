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
mod rule;
mod scenario;
mod script;
mod types;
mod visual;
mod world;
#[allow(dead_code)] // Games won't use all features.
mod world_builder;

use crate::types::Event;
use crate::world::*;

/// The main game object.  It owns the world as it currently is, and supports restart
/// and undo, etc.
/// TODO: Possibly, this should live elsewhere.
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
    pub fn introduce(&mut self) {
        println!("Welcome to Bonaventure!\n");

        // The first turn is always an implicit "look at the current setting".
        // This will also give everything else a chance to move.
        self.turn("look");
    }

    /// Execute one game turn.
    pub fn turn(&mut self, cmd: &str) {
        // FIRST, let the player do what he does.
        player_control::system(self, &cmd);

        // NEXT, handle rules
        rule::fire_event(&mut self.world, &Event::Turn);

        // NEXT, Increment the clock
        // TODO: Probably don't want to do this here.  Some commands should
        // take time, and some shouldn't.  This should probably be in the
        // player_control system.
        self.world.clock += 1;
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
        game.turn(&con.readline("> "));
    }
}
