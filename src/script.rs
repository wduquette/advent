//! Scripts that mutate the world

use crate::phys;
use crate::types::Action;
use crate::types::Action::*;
use crate::types::Flag;
use crate::visual;
use crate::world::World;
use crate::world;

/// A script of actions for execution.  Scripts can be pre-defined and executed
/// later, or created and executed immediately.
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

                // Moves a thing to a given place.
                PutIn(thing, inv) => {
                    phys::put_in(world, *thing, *inv);
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

/// A builder for scripts, for use especially in rule and command hooks.
/// Create a ScriptBuilder, add actions, and then call get() to retrieve the
/// script.
pub struct ScriptBuilder<'a> {
    world: &'a World,
    script: Script
}

impl<'a> ScriptBuilder<'a> {
    //------------------------------------------------------------------------------------------
    // Script Management

    /// Creates a new script builder relative to the given world.
    pub fn new(world: &'a World) -> Self {
        Self {
            world,
            script: Script::new(),
        }
    }

    /// Retrieves the finished script.
    pub fn get(self) -> Script {
        self.script
    }

    //------------------------------------------------------------------------------------------
    // Script Actions

    /// Adds an action to print the given text string.
    pub fn print(&mut self, text: &str) {
        self.script.add(Print(text.into()));
    }

    /// Adds an action to set the given flag on the tagged entity.
    /// Panics if the entity does not exist or doesn't allow flags.
    pub fn set_flag(&mut self, tag: &str, flag: Flag) {
        if let Some(id) = self.world.lookup_id(tag) {
            if self.world.has_flags(id) {
                self.script.add(SetFlag(id, flag));
            }
        } else {
            panic!("set_flag: not an entity with a flag set: {}", tag);
        }
    }

    /// Adds an action to move the tagged entity to LIMBO.
    /// Panics if the entity does not exist or has no location component.
    pub fn forget(&mut self, tag: &str) {
        if let Some(id) = self.world.lookup_id(tag) {
            if self.world.has_location(id) {
                self.script.add(PutIn(id, world::LIMBO));
            }
        } else {
            panic!("forget: not an entity with location: {}", tag);
        }
    }

    /// Adds an action to kill the given entity (i.e., set its Dead flag).
    /// At present the only thing that can be killed is the player, so
    /// this call panics if it is called for anything but the player.
    pub fn kill(&mut self, tag: &str) {
        if let Some(id) = self.world.lookup_id(tag) {
            if self.world.is_player(id) {
                self.script.add(Action::Kill(id));
            }
        } else {
            panic!("forget: not the player: {}", tag);
        }
    }

    /// Adds an action to revive the given entity (i.e., clear its Dead flag).
    /// At present the only thing that can be killed is the player, so
    /// this call panics if it is called for anything but the player.
    pub fn revive(&mut self, tag: &str) {
        if let Some(id) = self.world.lookup_id(tag) {
            if self.world.is_player(id) {
                self.script.add(Action::Revive(id));
            }
        } else {
            panic!("forget: not the player: {}", tag);
        }
    }
}
