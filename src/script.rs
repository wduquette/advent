//! Scripts that mutate the world

use crate::phys;
use crate::visual;
use crate::types::Action;
use crate::types::Action::*;
use crate::world::World;

/// A script of actions to be executed at a later time.
#[derive(Clone,Debug,Default)]
pub struct Script {
    pub actions: Vec<Action>,
}

impl Script {
    pub fn add(&mut self, action: Action) {
        self.actions.push(action);
    }

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
                ClearFlag(id, flag) => {
                    world.unset_flag(*id, *flag);
                }

                // Swap a, in a place, with b, in LIMBO
                Swap(a, b) => {
                    let loc = phys::loc(world, *a);
                    phys::take_out(world, *a);
                    phys::put_in(world, *b, loc);
                }
            }
        }
    }
}
