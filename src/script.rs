//! Scripts that mutate the world

use crate::phys;
use self::Action::*;
use crate::types::Flag;
use crate::visual;
use crate::world::World;
use crate::world_builder;

/// Actions taken by rules (and maybe other things)
#[allow(dead_code)]
#[derive(Clone, Debug)]
enum Action {
    /// Print the entity's visual
    Print(String),

    /// SetFlag(tag,flag): Set the flag on the tagged entity
    SetFlag(String, Flag),

    /// UnsetFlag(tag,flag): Unset the flag on the tagged entity
    UnsetFlag(String, Flag),

    /// PutIn(thing, inv): Put the tagged thing in the tagged
    /// entity's inventory
    PutIn(String, String),

    /// Swap(thing1, thing2) Swap a tagged thing in the world for one in LIMBO
    Swap(String, String),

    /// Drop(player,thing): Drop a held item into the current location.
    Drop(String, String),

    /// Kill(player): Kill the tagged player/NPC (currently, only the player)
    Kill(String),

    /// Revive(player): Revive the tagged player/NPC (currently, only the player)
    Revive(String),
}

/// A script of actions for execution.  Scripts can be pre-defined and executed
/// later, or created and executed immediately.
#[derive(Clone, Debug, Default)]
pub struct Script {
    actions: Vec<Action>,
}

impl Script {
    /// Creates a new, empty script.
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    /// Dumps the script.  Each line is preceded by the leader.
    pub fn dump(&self, leader: &str) {
        for action in &self.actions {
            println!("{}Action: {:?}", leader, action);
        }
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
                SetFlag(tag, flag) => {
                    world.set_flag(world.lookup(tag), *flag);
                }

                // Clear the flag on the entity's flag set
                UnsetFlag(tag, flag) => {
                    world.unset_flag(world.lookup(tag), *flag);
                }

                // Moves a thing to a given place.
                PutIn(thing, inv) => {
                    phys::put_in(world, world.lookup(thing), world.lookup(inv));
                }

                // Player/NPC drops thing into its current location.
                Drop(player, thing) => {
                    let loc = phys::loc(world, world.lookup(player));
                    phys::put_in(world, world.lookup(thing), loc);
                }

                // Swap a, in a place, with b, in LIMBO
                Swap(a, b) => {
                    let aid = world.lookup(a);
                    let bid = world.lookup(b);
                    let loc = phys::loc(world, aid);
                    phys::take_out(world, aid);
                    phys::put_in(world, bid, loc);
                }

                // Kill the player/NPC
                Kill(player) => {
                    world.set_flag(world.lookup(player), Flag::Dead);
                    visual::act("*** You have died. ***");
                }

                // Revive the player/NPC
                Revive(player) => {
                    world.unset_flag(world.lookup(player), Flag::Dead);
                    visual::act("*** You are alive! ***");
                }
            }
        }
    }

    //-------------------------------------------------------------------------------------------
    // Script Building Methods

    /// Adds an action to a script.
    fn add(&mut self, action: Action) {
        self.actions.push(action);
    }


    /// Adds an action to print the given text string.
    pub fn print(&mut self, text: &str) {
        self.add(Print(text.into()));
    }

    /// Adds an action to set the given flag on the tagged entity.
    pub fn set_flag(&mut self, tag: &str, flag: Flag) {
        self.add(SetFlag(tag.into(), flag));
    }

    /// Adds an action to move the tagged entity to LIMBO.
    pub fn forget(&mut self, thing: &str) {
        self.add(PutIn(thing.into(), world_builder::LIMBO.into()));
    }

    /// Adds an action to kill the given entity (i.e., set its Dead flag).
    /// At present the only thing that can be killed is the player.
    pub fn kill(&mut self, player: &str) {
        self.add(Action::Kill(player.into()));
    }

    /// Adds an action to revive the given entity (i.e., clear its Dead flag).
    /// At present the only thing that can be killed is the player.
    pub fn revive(&mut self, player: &str) {
        self.add(Action::Revive(player.into()));
    }
}
