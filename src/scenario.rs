//! Scenario definition

use crate::types::Dir::*;
use crate::types::*;

// Build the initial state of the game world.
pub fn build_world() -> World {
    // FIRST, make the empty world
    let mut world_map = World::new();
    let world = &mut world_map;

    // NEXT, make the rooms.

    // Rooms
    let clearing = make_room(
        world,
        "Clearing",
        "A wide spot in the woods.  You can go east.",
    );
    let trail = make_room(
        world,
        "Trail",
        "A trail from hither to yon.  You can go east or west.",
    );

    // Links
    link(world, East, clearing, trail);
    link(world, West, trail, clearing);

    // Stories
    make_story(world, "Story-1",|world| world.clock == 2, "\
You don't know where you are.  You don't even know where you want to
be.  All you know is that your feet are wet, your hands are dirty,
and gosh, this doesn't look like anything like the toy aisle.
    ");

    // NEXT, initialize the player
    initialize_player(world, clearing);

    // NEXT, return the world.
    world_map
}

fn initialize_player(world: &mut World, start: ID) {
    let pid = world.player;

    world.entities[pid].prose = Some(ProseComponent {
        name: String::from("You"),
        description: String::from("You've got all the usual bits."),
    });

    world.entities[pid].loc = Some(start);
}

fn make_room(world: &mut World, name: &str, description: &str) -> ID {
    let rid = world.make_entity();

    world.entities[rid].prose = Some(ProseComponent {
        name: String::from(name),
        description: String::from(description),
    });

    world.entities[rid].links = Some(LinksComponent::new());

    rid
}

fn make_story<F: 'static>(world: &mut World, name: &str, predicate: F, description: &str)
    where F: Fn(&World) -> bool
{
    let tid = world.make_entity();
    world.entities[tid].prose = Some(ProseComponent {
        name: format!("Trigger {}",name),
        description: String::from(description),
    });

    world.entities[tid].trigger = Some(TriggerComponent{
        predicate: Box::new(predicate),
        action: Action::Print,
        once_only: true,
        fired: false,
    });
}

fn link(world: &mut World, dir: Dir, from: ID, to: ID) {
    if let Some(links) = &mut world.entities[from].links {
        links.map.insert(dir, to);
    }
}
