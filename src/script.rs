//! Scripts that mutate the world

use crate::phys;
use crate::types::Action;
use crate::types::Action::*;
use crate::types::Flag;
use crate::visual;
use crate::world::World;

/// A script of actions to be executed at a later time.
#[derive(Clone, Debug, Default)]
pub struct Script {
    pub actions: Vec<Action>,
}

impl Script {
    /// Creates a new, empty script.
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    /// Adds an action to a script.
    pub fn add(&mut self, action: Action) {
        self.actions.push(action);
    }

    /// Executes a script on the world.
    pub fn execute(&self, world: &mut World) {
        for action in &self.actions {
            match action {
                // Print the rule's visual
                Print(visual) => {
                    visual::info(&visual);
                }

                // Set the flag on the entity's flag set
                SetFlag(id, flag) => {
                    world.set_flag(*id, *flag);
                }

                // Clear the flag on the entity's flag set
                UnsetFlag(id, flag) => {
                    world.unset_flag(*id, *flag);
                }

                // Player/NPC drops thing into its current location.
                Drop(pid, thing) => {
                    let loc = phys::loc(world, *pid);
                    phys::put_in(world, *thing, loc);
                }

                // Swap a, in a place, with b, in LIMBO
                Swap(a, b) => {
                    let loc = phys::loc(world, *a);
                    phys::take_out(world, *a);
                    phys::put_in(world, *b, loc);
                }

                // Kill the player/NPC
                Kill(pid) => {
                    world.set_flag(*pid, Flag::Dead);
                    visual::act("*** You have died. ***");
                }

                // Revive the player/NPC
                Revive(pid) => {
                    world.unset_flag(*pid, Flag::Dead);
                    visual::act("*** You are alive! ***");
                }
            }
        }
    }
}
