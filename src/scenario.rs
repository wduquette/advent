//! Scenario definition

use crate::entity::ID;
use crate::phys;
use crate::types::Action::*;
use crate::types::Dir::*;
use crate::types::Event::*;
use crate::types::Flag;
use crate::types::Flag::*;
use crate::types::ProseType::*;
use crate::visual::Buffer;
use crate::world::World;
use crate::world::LIMBO;

// Constant entity tags, for lookup
const NOTE: &str = "note";
const SWORD: &str = "sword";
const STONE: &str = "stone";

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
    // FIRST, make the empty world
    let mut the_world = World::new();
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

    // The player
    world.pid = world
        .add("self")
        .player()
        .prose_hook(Thing, &|world, id| player_visual(world, id))
        .flag(DIRTY_HANDS)
        .id();

    let pid = world.pid;

    // Room: Clearing
    let clearing = world
        .add("clearing")
        .room("Clearing")
        .prose(Room, "\
A wide spot in the woods.  The trees are dense, but there seem to be paths
heading to the north, south, and east.
        ")
        .dead_end(North, "\
You feel a chill as you approach the edge of the clearing, and after a few more steps are
overcome with a vague but horrifying sense of deja vu.  You don't remember
what's back under the trees to the north, but you're pretty sure you didn't like it
and that you don't want to go find it again.
        ")
        .id();

    // The note
    let note = world
        .add(NOTE)
        .thing("note", "note")
        .prose_hook(Thing, &|world, id| note_thing_prose(world, id))
        .prose(Book, "\
If you ever wish to see your toy aisle alive again, put $10,000 dollars
under the statue in the castle courtyard before nine o'clock tomorrow morning.
||   -- Your host.
||Well.  That's a bit alarming.  Where are you going to find $10,000 at this time of day?
         ")
        .put_in(clearing)
        .id();

    world
        .add("guard-dirty-note")
        .before(ReadThing(pid, note), &|w, _| !w.tag_has(NOTE, DIRTY))
        .action(Print("You've gotten it too dirty to read.".into()));

    world
        .add("rule-dirty-note")
        .always(GetThing(pid, note),
            &|w, _| w.has(w.pid, DIRTY_HANDS) && !w.tag_has(NOTE, DIRTY))
        .action(Print(
            "The dirt from your hands got all over the note.".into(),
        ))
        .action(SetFlag(note, DIRTY));

    // Room: Grotto
    let grotto = world
        .add("grotto")
        .room("Grotto")
        .prose(Room, "\
Nestled in a grotto among the trees you find a pool of water.
A path leads west.
        ")
        .flag(HAS_WATER)
        .id();

        world
            .add("pool")
            .thing("pool", "pool")
            .prose(
                Thing,
                "\
Moss grows on the stones around the edge, but the water is clear and
deep and cold.
            ",
            )
            .flag(Scenery)
            .put_in(grotto)
            .id();

    // Room: Hilltop
    let hilltop = world
        .add("hilltop")
        .room("Hilltop")
        .prose(
            Room,
            "\
The path has led you to the top of a hill, where there is a broad open
space.  Trails lead to the north and south.
            ",
        )
        .id();

    // The stone
    let stone = world
        .add(STONE)
        .thing("stone", "stone")
        .prose(
            Thing,
            "\
It's a massive block of marble, four feet wide and three feet high.  The top is flat, and the
four sides slope inward.  There's a sword sticking out of the top.  These words are chiseled
into one side:
||   * Only The Pure *
            "
        )
        .flag(Scenery)
        .put_in(hilltop)
        .id();

    // The sword
    let sword = world
        .add(SWORD)
        .thing("sword", "sword")
        .prose_hook(Thing, &|w, id| sword_thing_prose(w, id))
        .put_in(hilltop)
        .id();

    world
        .add("before-sword-get")
        .before(GetThing(pid, sword), &|w,_| {
            !w.has(w.pid, DIRTY_HANDS)
        })
        .action(Print(
            "\
Oh, you so didn't want to touch the sword with dirty hands.
Weren't you paying attention? Only the pure may touch this sword.
            "
            .into(),
        ))
        .action(Kill(pid));

    world
        .add("rule-sword-get")
        .once(GetThing(pid, sword), &|w,_| !w.tag_has(SWORD, TAKEN))
        .action(Print(
            "\
The sword almost seems to leap into your hands.  As you marvel at it
(and, really, there's something odd about it), the marble block dissolves
into white mist and blows away.
            "
            .into(),
        ))
        .action(PutIn(stone, LIMBO))
        .action(SetFlag(sword, TAKEN));

    // Room: Mouth of Cave
    let cave_mouth = world
        .add("cave-mouth")
        .room("Mouth of Cave")
        .prose(
            Room,
            "\
The trail ends at the mouth of a dark and forbidding cave.  You just
know that if you go any closer, a stream of bats will fly out and
scare you silly.  If you choose, you can enter the cave to the east, or
go back up the trail to the west.
            ",
        )
        .id();

    let cave_1 = world
        .add("cave-1")
        .room("In Cave")
        .prose(
            Room,
            "\
You're in a damp, muddy cave, dimly lit by patches of the glowing fungus
that indicates that game designer didn't want to be bothered with providing
you a light source. The entrance is to the west, and a narrow passage continues
to the east.
        ")
        .dead_end(East, "\
At least, it would if the developer had implemented it yet.
        ")
        .id();

    world
        .add("before-cave-1")
        .before(EnterRoom(pid, cave_1), &|w,_| w.tag_owns(w.pid, SWORD))
        .action(Print("\
Oh, hell, no, you're not going in there empty handed.  You'd better go back
and get that sword.
        ".into()));

    world
        .add("rule-enter-cave-1")
        .once(EnterRoom(pid, cave_1), &|_,_| true)
        .action(Print("\
It's an unpleasant place but your sword gives you confidence and warm fuzzies.
        ".into()));

    // UNUSED! Room: Bridge
    let bridge = world
        .add("bridge")
        .room("Bridge")
        .prose(
            Room,
            "The trail crosses a small stream here.  You can go east or west.",
        )
        .flag(HAS_WATER)
        .id();

    world
        .add("stream")
        .thing("stream", "stream")
        .prose(
            Thing,
            "\
The stream comes from the north, down a little waterfall, and runs
away under the bridge.  It looks surprisingly deep, considering
how narrow it is.
        ",
        )
        .flag(Scenery)
        .put_in(bridge)
        .id();

    // Links
    world.twoway(clearing, East, West, grotto);
    world.twoway(clearing, South, North, hilltop);
    world.twoway(hilltop, South, West, cave_mouth);
    world.twoway(cave_mouth, East, West, cave_1);

    // Other Rules

    // The fairy-godmother revives you if you die.
    world
        .add("fairy-godmother-rule")
        .always(Turn, &|w, _| w.has(w.pid, Dead))
        .action(Print(
            "\
A fairy godmother hovers over your limp body.  She frowns;
then, apparently against her better judgment, she waves
her wand.  There's a flash, and she disappears.
            "
            .into(),
        ))
        .action(Revive(pid));

    // NEXT, set the starting location.
    phys::put_in(world, world.pid, clearing);
    world.set(world.pid, Seen(clearing));

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
