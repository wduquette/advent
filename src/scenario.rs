//! Scenario definition

use crate::entity::ID;
use crate::types::Flag::*;
use crate::types::Dir::*;
use crate::types::*;
use crate::world::*;
use crate::entity::rule::Action::*;

// Important Constants
const NOTE: &str = "note-clean";

/// Build the initial state of the game world.
pub fn build() -> World {
    // FIRST, make the empty world
    let mut the_world = World::new();
    let world = &mut the_world;

    // // NEXT, Make the player
    world.pid = world
        .add("self")
        .player("You've got all the usual bits.")
        .flag(DirtyHands)
        .id();

    // NEXT, make the rooms.

    // Room: Clearing
    let clearing = world
        .add("clearing")
        .room("Clearing", "A wide spot in the woods.  You can go east.")
        .id();

    // Room: Trail
    let trail = world
        .add("trail")
        .room(
            "Trail",
            "A trail from hither to yon.  You can go east or west.",
        )
        .id();

    // Room: Bridge
    let bridge = world
        .add("bridge")
        .room(
            "Bridge",
            "\
The trail crosses a small stream here.  You can go east or west.
        ",
        )
        .flag(HasWater)
        .id();

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
        .add(NOTE)
        .thing("note", "note", "A note, on plain paper.")
        .book(
            "\
Welcome, dear friend.  Your mission, should you choose to
accept it, is to figure out how to get to the end of
the trail.  You've already taken the first big
step!
         ",
        )
        .id();
    world.put_in(clean_note, clearing);

    let dirty_note = world
        .add("note-dirty")
        .thing(
            "note",
            "note",
            "A note, on plain paper.  It looks pretty grubby.",
        )
        .book("You've gotten it too dirty to read.")
        .id();

    // TODO: Not working yet
    // world
    //     .add("rule-dirty-note")
    //     .always(&|world| player_gets_note_dirty(world))
    //     .action(Print(
    //         "The dirt from your hands got all over the note.".into(),
    //     ))
    //     .action(Swap(clean_note, dirty_note));

    // Stories: Rules that supply backstory to the player.
    world
        .add("rule-story-1")
        .once(&|world| world.clock == 2)
        .action(Print(
            "\
You don't know where you are.  You don't even know where you want to
be.  All you know is that your feet are wet, your hands are dirty,
and gosh, this doesn't look anything like the toy aisle.
        "
            .into(),
        ));


    // NEXT, set the starting location.
    world.set_room(world.pid, clearing);

    // NEXT, return the world.
    the_world
}

fn player_gets_note_dirty(world: &World) -> bool {
    let playerv = world.player();
    let id = world.lookup_id(NOTE).unwrap();
    let notev = world.as_thing(id);

    // TODO: consider adding methods to InventoryComponent!
    playerv.inventory.things.contains(&id) &&
    playerv.flag_set.has(DirtyHands) &&
    !notev.flag_set.has(Dirty)
}

/// Makes a scenery object, and returns its ID.
fn make_scenery(world: &mut World, loc: ID, tag: &str, name: &str, text: &str) -> ID {
    let id = world
        .add(tag)
        .thing(name, name, text)
        .flag(Scenery)
        .id();
    world.put_in(id, loc);

    id
}

// TODO: oneway and connect should be world methods link() and bilink(); we should also have
// unlink() and unbilink().

/// Links one room to another in the given direction.
/// Links are not bidirectional.  If you want links both ways, you
/// have to add them.
fn oneway(world: &mut World, dir: Dir, from: ID, to: ID) {
    let roomv = &mut world.as_room(from);
    roomv.room.links.insert(dir, to);
    roomv.save(world);
}

/// Establishes a bidirectional link between two rooms.
fn connect(world: &mut World, from_dir: Dir, from: ID, to_dir: Dir, to: ID) {
    oneway(world, from_dir, from, to);
    oneway(world, to_dir, to, from);
}
