//! Scenario definition

use crate::types::Dir::*;
use crate::types::Var::*;
use crate::types::*;
use crate::world::*;

// Important Constants
const NOTE: &str = "note-1";

/// Build the initial state of the game world.
pub fn build() -> World {
    // FIRST, make the empty world
    let mut the_world = World::new();
    let world = &mut the_world;

    // NEXT, make the rooms.

    //Room: Clearing
    let clearing = make_room(
        world,
        "clearing-1",
        "Clearing",
        "A wide spot in the woods.  You can go east.",
    );

    // Room: Trail
    let trail = make_room(
        world,
        "trail-1",
        "Trail",
        "A trail from hither to yon.  You can go east or west.",
    );

    // Room: Bridge
    let bridge = make_room(
        world,
        "bridge-1",
        "Bridge",
        "\
The trail crosses a small stream here.  You can go east or west.
        ",
    );

    world.set_var(bridge, HasWater);

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

    // Links
    connect(world, East, clearing, West, trail);
    connect(world, East, trail, West, bridge);

    // NEXT, make the things
    // The note
//     let visual_clean_note = world
//         .make("visual-clean-note")
//         .visual("\
// Welcome, dear friend.  Your mission, should you choose to
// accept it, is to figure out how to get to the end of
// the trail.  You've already taken the first big
// step!
//          ")
//         .build();
//
//     let visual_dirty_note = world
//         .make("visual-dirty-note")
//         .visual("It's so dirty it's illegible.")
//         .build();

    let note = world.make(NOTE)
        .name("note")
        .visual("A note, on plain paper.")
        .vars()
        .build();
    put_in(world, note, clearing);

    world
        .make("rule-dirty-note")
        .always(
            &|world| player_gets_note_dirty(world),
            vec![Action::PrintVisual,
                Action::SetVar(note, Dirty)],
        )
        .visual("The dirt from your hands got all over the note.")
        .build();

    // Stories: Rules that supply backstory to the player.
    world
        .make("rule-story-1")
        .once(&|world| world.clock == 2, vec![Action::PrintVisual])
        .visual(
            "\
You don't know where you are.  You don't even know where you want to
be.  All you know is that your feet are wet, your hands are dirty,
and gosh, this doesn't look anything like the toy aisle.
        ",
        )
        .build();

    // NEXT, Make the player
    make_player(world, clearing);

    // NEXT, return the world.
    the_world
}

fn player_gets_note_dirty(world: &World) -> bool {
    let player = world.player();
    let note = world.lookup(NOTE).as_thing();

    player.inventory.contains(&note.id) &&
    player.vars.contains(&DirtyHands) &&
    !note.vars.contains(&Dirty)
}

/// Initializes the player's details
fn make_player(world: &mut World, start: ID) {
    world.pid = world
        .make("self")
        .name("self")
        .visual("You've got all the usual bits.")
        .location(start)
        .inventory()
        .var(DirtyHands)
        .var(Seen(start))
        .build();
}

/// Makes a room with the given name and visual, and an empty set of links.
/// Returns the room's ID.
fn make_room(world: &mut World, tag: &str, name: &str, text: &str) -> ID {
    world
        .make(tag)
        .name(name)
        .visual(text)
        .links()
        .inventory()
        .vars()
        .build()
}

/// Makes a portable object, and returns its ID.
fn make_thing(world: &mut World, tag: &str, name: &str, text: &str) -> ID {
    world.make(tag).name(name).visual(text).vars().build()
}

/// Makes a scenery object, and returns its ID.
fn make_scenery(world: &mut World, loc: ID, tag: &str, name: &str, text: &str) -> ID {
    let id = world.make(tag).name(name).visual(text).var(Scenery).build();

    put_in(world, id, loc);

    id
}

/// Links one room to another in the given direction.
/// Links are not bidirectional.  If you want links both ways, you
/// have to add them.
fn oneway(world: &mut World, dir: Dir, from: ID, to: ID) {
    let room = &mut world.get(from).as_room();
    room.links.insert(dir, to);
    room.save(world);
}

/// Establishes a bidirectional link between two rooms.
fn connect(world: &mut World, from_dir: Dir, from: ID, to_dir: Dir, to: ID) {
    oneway(world, from_dir, from, to);
    oneway(world, to_dir, to, from);
}

/// Puts the thing in the container's inventory, and sets the thing's location.
/// No op if the thing is already in the location.
pub fn put_in(world: &mut World, thing: ID, container: ID) {
    if let Some(inv) = &mut world.entities[container].inventory {
        if !inv.contains(&thing) {
            inv.insert(thing);
            world.entities[thing].loc = Some(container);
        }
    }
}
