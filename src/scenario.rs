//! Scenario definition

use crate::types::Dir::*;
use crate::types::Var::*;
use crate::types::*;
use crate::world::*;

// Important Constants
const NOTE: &str = "note-clean";

/// Build the initial state of the game world.
pub fn build() -> World {
    // FIRST, make the empty world
    let mut the_world = World::new();
    let world = &mut the_world;

    // NEXT, make the rooms.

    // Room: Clearing
    let clearing = world
        .make("clearing")
        .room("Clearing",
            "A wide spot in the woods.  You can go east.")
        .build();

    // Room: Trail
    let trail = world
        .make("trail")
        .room("Trail", "A trail from hither to yon.  You can go east or west.")
        .build();

    // Room: Bridge
    let bridge = world
        .make("bridge")
        .room("Bridge", "\
The trail crosses a small stream here.  You can go east or west.
        ")
        .var(HasWater)
        .build();

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

    // The note
    let clean_note = world
        .make(NOTE)
        .name("note")
        .visual("A note, on plain paper.")
        .prose(
            "\
Welcome, dear friend.  Your mission, should you choose to
accept it, is to figure out how to get to the end of
the trail.  You've already taken the first big
step!
         ",
        )
        .vars()
        .build();
    world.put_in(clean_note, clearing);

    let dirty_note = world
        .make("note-dirty")
        .name("note")
        .visual("A note, on plain paper.  It looks pretty grubby.")
        .prose("You've gotten it too dirty to read.")
        .vars()
        .limbo()
        .build();

    world
        .make("rule-dirty-note")
        .always(
            &|world| player_gets_note_dirty(world),
            vec![Action::PrintVisual, Action::Swap(clean_note, dirty_note)],
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

    player.inventory.contains(&note.id)
        && player.vars.contains(&DirtyHands)
        && !note.vars.contains(&Dirty)
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

/// Makes a scenery object, and returns its ID.
fn make_scenery(world: &mut World, loc: ID, tag: &str, name: &str, text: &str) -> ID {
    let id = world.make(tag).name(name).visual(text).var(Scenery).build();

    world.put_in(id, loc);

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
