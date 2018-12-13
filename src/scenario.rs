//! Scenario definition

use crate::types::Var::*;
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
        "clearing-1",
        "Clearing",
        "A wide spot in the woods.  You can go east.",
    );
    let trail = make_room(
        world,
        "trail-1",
        "Trail",
        "A trail from hither to yon.  You can go east or west.",
    );
    let bridge = make_room(
        world,
        "bridge-1",
        "Bridge",
        "\
The trail crosses a small stream here.  You can go east or west.
        ",
    );
    world.set(HasWater(bridge));

    // Links
    connect(world, East, clearing, West, trail);
    connect(world, East, trail, West, bridge);

    // NEXT, make the things
    let note = make_portable(world, "note-1", "note", "It's illegible.");
    world.put_in(note, clearing);

    make_scenery(
        world,
        bridge,
        "stream-1",
        "stream",
        "\
The stream comes from the north, down a little waterfall, and runs
away under the bridge.  It looks surprisingly deep, considering
how narrow it is.
        ",
    );

    // Stories: Rules that supply backstory to the player.
    make_story(
        world,
        "story-1",
        |world| world.clock == 2,
        "\
You don't know where you are.  You don't even know where you want to
be.  All you know is that your feet are wet, your hands are dirty,
and gosh, this doesn't look anything like the toy aisle.
    ",
    );

    // NEXT, Make the player
    make_player(world, clearing);

    // NEXT, return the world.
    the_world
}

/// Initializes the player's details
fn make_player(world: &mut World, start: ID) {
    world.pid = world.make("self")
        .prose("You've got all the usual bits.")
        .location(start)
        .inventory()
        .thing(false)
        .build();

    world.set(Seen(start));
    world.set(DirtyHands);
}

/// Makes a room with the given name and prose, and an empty set of links.
/// Returns the room's ID.
fn make_room(world: &mut World, tag: &str, name: &str, text: &str) -> ID {
    world.make(tag)
        .name(name)
        .prose(text)
        .links()
        .inventory()
        .build()
}

/// Makes a portable object, and returns its ID.
fn make_portable(world: &mut World, tag: &str, name: &str, text: &str) -> ID {
    world
        .make(tag)
        .name(name)
        .prose(text)
        .thing(true) // TODO: Obscure; needs improvement.
        .build()
}

/// Makes a scenery object, and returns its ID.
fn make_scenery(world: &mut World, loc: ID, tag: &str, name: &str, text: &str) -> ID {
    world
        .make(tag)
        .name(name)
        .prose(text)
        .location(loc)
        .thing(false) // TODO: Obscure; needs improvement.
        .build()
}

/// Adds a bit of backstory to be revealed when the conditions are right.
/// Backstory will appear only once.
fn make_story<F: 'static>(world: &mut World, tag: &str, predicate: F, text: &str)
where
    F: Fn(&World) -> bool,
{
    world
        .make(tag)
        .prose(text)
        .rule(predicate, Action::Print, true)
        .build();
}

/// Links one room to another in the given direction.
/// Links are not bidirectional.  If you want links both ways, you
/// have to add them.
fn oneway(world: &mut World, dir: Dir, from: ID, to: ID) {
    let links = &mut world.entities[from]
        .links
        .as_mut()
        .unwrap_or_else(|| panic!("Entity has no link component: {}", from));

    links.insert(dir, to);
}

/// Establishes a bidirectional link between two rooms.
fn connect(world: &mut World, from_dir: Dir, from: ID, to_dir: Dir, to: ID) {
    oneway(world, from_dir, from, to);
    oneway(world, to_dir, to, from);
}
