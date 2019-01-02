//! Scenario definition

use crate::entity::ID;
use crate::phys;
use crate::types::Action::*;
use crate::types::Dir::*;
use crate::types::Event::*;
use crate::types::Flag;
use crate::types::Flag::*;
use crate::visual::Buffer;
use crate::world::World;
use crate::world;
use crate::world_builder::WorldBuilder;

// Constant entity tags, for lookup
const NOTE: &str = "note";
const SWORD: &str = "sword";

// User-defined flags
// TODO: These constants should only be used in the scenario itself; but at present they
// are still used by the "wash hands" command code in player_control.rs.  Once that's
// implemented clearly, they should no longer be "pub".
const DIRTY: Flag = User("DIRTY");
pub const DIRTY_HANDS: Flag = User("DIRTY_HANDS");
pub const HAS_WATER: Flag = User("HAS_WATER");
pub const TAKEN: Flag = User("TAKEN");

/// Build the initial state of the game world.
pub fn build() -> World {
    // FIRST, create the world builder
    let mut wb = WorldBuilder::new();

    wb.player()
        .location("clearing")
        .flag(DIRTY_HANDS)
        .prose_hook(&|w, id| player_visual(w, id)); // TODO: put in-line.

    // Room: Clearing
    wb.room("clearing", "A Dreary Clearing")
        .prose("\
A wide spot in the woods.  The trees are dense, but there seem to be paths
heading to the north, south, and east.
        ")
        .dead_end(North, "\
You feel a chill as you approach the edge of the clearing, and after a few more steps are
overcome with a vague but horrifying sense of deja vu.  You don't remember
what's back under the trees to the north, but you're pretty sure you didn't like it
and that you don't want to go find it again.
        ")
        .link(East, "grotto")
        .link(South, "hilltop");

    // Thing: A ransom note, found in the clearing
    // TODO: the method names prose_hook and book_prose can be better named.
    // examine(), read(), examine_hook(), read_hook()?
    wb.thing("note", "note", "note")
        .location("clearing")
        .prose_hook(&|world, id| note_thing_prose(world, id))
        .book_prose("\
If you ever wish to see your toy aisle alive again, put $10,000 dollars
under the statue in the castle courtyard before nine o'clock tomorrow morning.
||   -- Your host.
||Well.  That's a bit alarming.  Where are you going to find $10,000 at this time of day?
         ");

    // Room: Grotto
    wb.room("grotto", "A Grotto in the Woods")
        .link(West, "clearing")
        .prose("\
Nestled in a grotto among the trees you find a pool of water.
A path leads west.
        ")
        .flag(HAS_WATER);

    // Thing: Pool, a pool in the Grotto
    wb.thing("pool", "pool", "pool")
        .location("grotto")
        .flag(Scenery)
        .prose("\
Moss grows on the stones around the edge, but the water is clear and
deep and cold.
        ");

    // Room: Hilltop
    wb.room("hilltop", "A Windy Hilltop")
        .link(North, "clearing")
        .link(South, "cave-mouth")
        .prose("\
The path has led you to the top of a hill, where there is a broad open
space.  Trails lead to the north and south.
        ");

    // Thing: The Stone on the Hilltop
    wb.thing("stone", "stone", "stone")
        .location("hilltop")
        .flag(Scenery)
        .prose("\
It's a massive block of marble, four feet wide and three feet high.  The top is flat, and the
four sides slope inward.  There's a sword sticking out of the top.  These words are chiseled
into one side:
||   * Only The Pure *
        ");

    // Thing: The Sword in the Stone on the Hilltop
    wb.thing("sword", "sword", "sword")
        .location("hilltop")
        .prose_hook(&|w, id| sword_thing_prose(w, id));  // TODO: Put in-line

    // Room: Mouth of Cave
    wb.room("cave-mouth", "The Mouth of a Forbidding Cave")
        .link(West, "hilltop")
        .link(East, "cave-1")
        .prose("\
The trail ends at the mouth of a dark and forbidding cave.  You just
know that if you go any closer, a stream of bats will fly out and
scare you silly.  If you choose, you can enter the cave to the east, or
go back up the trail to the west.
        ");

    // Room: The Cave, First Chamber
    wb.room("cave-1", "In the Cave")
        .link(West, "cave-mouth")
        .dead_end(East, "\
At least, it would if the developer had implemented it yet.
        ")
        .prose("\
You're in a damp, muddy cave, dimly lit by patches of the glowing fungus
that indicates that game designer didn't want to be bothered with providing
you a light source. The entrance is to the west, and a narrow passage continues
to the east.
        ");

    // NEXT, retrieve the world.
    // TODO: Ultimately, this will be at the end of the function, to return the newly
    // build World.
    let mut the_world = wb.world();
    let world = &mut the_world;

    // Story 1
    world
        .add("rule-story-1")
        .once(Turn, &|w,_| w.clock == 0)
        .action(Print(
            "\
You don't know where you are.  You don't even know where you want to
be.  All you know is that your feet are wet, your hands are dirty,
and gosh, this doesn't look anything like the toy aisle.
        "
            .into(),
        ));

    crate::debug::list_world(world);
    crate::debug::dump_world(world);

    // Temporary
    let pid = world.pid;
    let clearing = world.lookup("clearing");
    let note = world.lookup("note");
    let stone = world.lookup("stone");
    let sword = world.lookup("sword");
    let cave_1 = world.lookup("cave-1");

//     world
//         .add("guard-dirty-note")
//         .before(ReadThing(pid, note), &|w, _| !w.tag_has(NOTE, DIRTY))
//         .action(Print("You've gotten it too dirty to read.".into()));
//
//     world
//         .add("rule-dirty-note")
//         .always(GetThing(pid, note),
//             &|w, _| w.has(w.pid, DIRTY_HANDS) && !w.tag_has(NOTE, DIRTY))
//         .action(Print(
//             "The dirt from your hands got all over the note.".into(),
//         ))
//         .action(SetFlag(note, DIRTY));
//
//
//
//     world
//         .add("before-sword-get")
//         .before(GetThing(pid, sword), &|w,_| {
//             !w.has(w.pid, DIRTY_HANDS)
//         })
//         .action(Print(
//             "\
// Oh, you so didn't want to touch the sword with dirty hands.
// Weren't you paying attention? Only the pure may touch this sword.
//             "
//             .into(),
//         ))
//         .action(Kill(pid));
//
//     world
//         .add("rule-sword-get")
//         .once(GetThing(pid, sword), &|w,_| !w.tag_has(SWORD, TAKEN))
//         .action(Print(
//             "\
// The sword almost seems to leap into your hands.  As you marvel at it
// (and, really, there's something odd about it), the marble block dissolves
// into white mist and blows away.
//             "
//             .into(),
//         ))
//         .action(PutIn(stone, world::LIMBO))
//         .action(SetFlag(sword, TAKEN));
//
//     world
//         .add("before-cave-1")
//         .before(EnterRoom(pid, cave_1), &|w,_| w.tag_owns(w.pid, SWORD))
//         .action(Print("\
// Oh, hell, no, you're not going in there empty handed.  You'd better go back
// and get that sword.
//         ".into()));
//
//     world
//         .add("rule-enter-cave-1")
//         .once(EnterRoom(pid, cave_1), &|_,_| true)
//         .action(Print("\
// It's an unpleasant place but your sword gives you confidence and warm fuzzies.
//         ".into()));
//
//     // Other Rules
//
//     // The fairy-godmother revives you if you die.
//     world
//         .add("fairy-godmother-rule")
//         .always(Turn, &|w, _| w.has(w.pid, Dead))
//         .action(Print(
//             "\
// A fairy godmother hovers over your limp body.  She frowns;
// then, apparently against her better judgment, she waves
// her wand.  There's a flash, and she disappears.
//             "
//             .into(),
//         ))
//         .action(Revive(pid));

    // NEXT, return the world.
    the_world
}

/// Returns the player's current appearance.
fn player_visual(world: &World, pid: ID) -> String {
    Buffer::new()
        .add("You've got all the usual bits.")
        .when(
            world.has(pid, DIRTY_HANDS),
            "Your hands are kind of dirty, though.",
        )
        .when(
            !world.has(pid, DIRTY_HANDS),
            "Plus, they're clean bits!",
        )
        .get()
}

fn note_thing_prose(world: &World, id: ID) -> String {
    if world.has(id, DIRTY) {
        "A note, on plain paper.  It looks pretty grubby; someone's been mishandling it.".into()
    } else {
        "A note, on plain paper".into()
    }
}

fn sword_thing_prose(world: &World, id: ID) -> String {
    if world.has(id, TAKEN) {
        "\
The sword, if you want to call it that, is a three-foot length of dark hardwood
with a sharkskin hilt on one end.  It's polished so that it gleams, and it has no
sharp edges anywhere.  Carved along the length of it are the words
\"Emotional Support Sword (TM)\".
        ".into()
    } else {
        "All you can really see is the hilt; the rest is embedded in the stone.".into()
    }
}
