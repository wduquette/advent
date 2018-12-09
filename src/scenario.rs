//! Scenario definition

use crate::types::Dir::*;
use crate::types::*;
use crate::world::*;

/// Build the initial state of the game world.
pub fn build() -> World {
    // FIRST, make the empty world
    let mut the_world = World::new();
    let world = &mut the_world;

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

    // Stories: Triggers that supply backstory to the player.
    make_story(
        world,
        "Story-1",
        |world| world.clock == 2,
        "\
You don't know where you are.  You don't even know where you want to
be.  All you know is that your feet are wet, your hands are dirty,
and gosh, this doesn't look like anything like the toy aisle.
    ",
    );

    // NEXT, initialize the player
    initialize_player(world, clearing);

    // NEXT, return the world.
    the_world
}

/// Initializes the player's details
fn initialize_player(world: &mut World, start: ID) {
    let pid = world.player;

    world.entities[pid].name = "You".into();
    world.entities[pid].prose = Some(ProseComponent {
        text: "You've got all the usual bits.".into(),
    });

    world.entities[pid].loc = Some(start);
}

/// Makes a room with the given name and prose, and an empty set of links.
/// Returns the room's ID.
fn make_room(world: &mut World, name: &str, text: &str) -> ID {
    let rid = world.alloc();

    world.entities[rid].name = name.into();
    world.entities[rid].prose = Some(ProseComponent {
        text: text.into(),
    });

    world.entities[rid].links = Some(LinksComponent::new());

    rid
}

/// Adds a bit of backstory to be revealed when the conditions are right.
/// Backstory will appear only once.
fn make_story<F: 'static>(world: &mut World, name: &str, predicate: F, text: &str)
where
    F: Fn(&World) -> bool,
{
    let tid = world.alloc();
    world.entities[tid].name = format!("Trigger {}", name);
    world.entities[tid].prose = Some(ProseComponent {
        text: text.trim().into(),
    });

    world.entities[tid].trigger = Some(TriggerComponent {
        predicate: Box::new(predicate),
        action: Action::Print,
        once_only: true,
        fired: false,
    });
}

/// Links one room to another in the given direction.
/// Links are not bidirectional.  If you want links both ways, you
/// have to add them.
fn link(world: &mut World, dir: Dir, from: ID, to: ID) {
    let links = &mut world.entities[from].links.as_mut()
        .expect(&format!("Entity has no link component: {}", from));

    links.map.insert(dir, to);
}
