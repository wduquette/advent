//! Scenario definition

use crate::entity::prose::ProseType::*;
use crate::entity::ID;
use crate::entity::rule::Action::*;
use crate::types::Dir::*;
use crate::types::Flag::*;
use crate::world::World;
use crate::visual::Buffer;

// Important Constants
const NOTE: &str = "note.clean";

/// Build the initial state of the game world.
pub fn build() -> World {
    // FIRST, make the empty world
    let mut the_world = World::new();
    let world = &mut the_world;

    // // NEXT, Make the player
    world.pid = world
        .add("self")
        .player()
        .prose_hook(Thing, &|world, id| player_visual(world, id))
        .flag(DirtyHands)
        .id();

    // NEXT, make the rooms.

    // Room: Clearing
    let clearing = world
        .add("clearing")
        .room("Clearing")
        .prose(Room, "A wide spot in the woods.  You can go east.")
        .id();

    // Room: Trail
    let trail = world
        .add("trail")
        .room("Trail")
        .prose(Room, "A trail from hither to yon.  You can go east or west.")
        .id();

    // Room: Bridge
    let bridge = world
        .add("bridge")
        .room("Bridge")
        .prose(Room, "The trail crosses a small stream here.  You can go east or west.")
        .flag(HasWater)
        .id();

    let stream = world
        .add("stream")
        .thing("stream", "stream")
        .prose(Thing,
            "\
The stream comes from the north, down a little waterfall, and runs
away under the bridge.  It looks surprisingly deep, considering
how narrow it is.
        ",
        )
        .flag(Scenery)
        .id();
    world.put_in(stream, bridge);

    // Links
    world.twoway(clearing, East, West, trail);
    world.twoway(trail, East, West, bridge);

    // The note
    let clean_note = world
        .add(NOTE)
        .thing("note", "note")
        .prose(Thing, "A note, on plain paper.")
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
        .add("note.dirty")
        .thing("note", "note",)
        .prose(Thing, "A note, on plain paper.  It looks pretty grubby.")
        .book("You've gotten it too dirty to read.")
        .id();

    world
        .add("rule-dirty-note")
        .always(&|world| player_gets_note_dirty(world))
        .action(Print(
            "The dirt from your hands got all over the note.".into(),
        ))
        .action(Swap(clean_note, dirty_note));

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
    world.set_flag(world.pid, Seen(clearing));

    // NEXT, return the world.
    the_world
}

/// Returns the player's current appearance.
fn player_visual(world: &World, _id: ID) -> String {
    let playerv = world.player();

    Buffer::new()
        .add("You've got all the usual bits.")
        .when(
            playerv.flag_set.has(DirtyHands),
            "Your hands are kind of dirty, though.",
        )
        .when(
            !playerv.flag_set.has(DirtyHands),
            "Plus, they're clean bits!",
        )
        .get()
}

fn player_gets_note_dirty(world: &World) -> bool {
    let playerv = world.player();
    let id = world.lookup_id(NOTE).unwrap();
    let notev = world.as_thing(id);

    playerv.inventory.has(id) && playerv.flag_set.has(DirtyHands) && !notev.flag_set.has(Dirty)
}
