//! Scenario definition

use crate::types::Dir::*;
use crate::types::Flag;
use crate::types::Flag::*;
use crate::types::ProseBuffer;
use crate::world::World;
use crate::world_builder::*;
use crate::world_builder::WBEvent::*;

// User-defined flags
const DIRTY: Flag = User("DIRTY");
const HAS_WATER: Flag = User("HAS_WATER");
const TAKEN: Flag = User("TAKEN");

/// Build the initial state of the game world.
pub fn build() -> World {
    // FIRST, create the world builder
    let mut wb = WorldBuilder::new();

    // NEXT, configure the player
    wb.player()
        .location("clearing")
        .on_examine("You've got all the usual bits.");

    wb.feature("hands", "hands", "hands")
        .location(PLAYER)
        .flag(DIRTY)
        .on_examine_hook(&|w,e,buff| {
            if w.has(e, DIRTY) {
                buff.puts("You don't remember what you were doing, but it must have been messy.");
            } else {
                buff.puts("Fresh and clean.");
            }
        })
        .on_scenery_hook(&|w,e,buff| {
            if w.has(e, DIRTY) {
                buff.puts("Your hands are kind of dirty, though.");
            }
        });

    // NEXT, create and configure the things in the world.

    // Rule: Story 1
    wb.rule("rule-story-1")
        .when(&|w| w.clock() == 0)
        .print("\
You don't know where you are.  You don't even know where you want to
be.  All you know is that your feet are wet, your hands are dirty,
and gosh, this doesn't look anything like the toy aisle.
        ");

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
    wb.thing("note", "note", "note")
        .location("clearing")
        .on_examine_hook(&|w,e,buff| {
            buff.puts("A note, on plain paper.");
            if w.has(e, DIRTY) {
                buff.puts("It looks pretty grubby; someone's been mishandling it.");
            }
        })
        .on_read("\
If you ever wish to see your toy aisle alive again, put $10,000 dollars
under the statue in the castle courtyard before nine o'clock tomorrow morning.
||   -- Your host.
||Well.  That's a bit alarming.  Where are you going to find $10,000 at this time of day?
         ");

    // You can't read the note if it's dirty.
    wb.allow(&ReadThing("note"))
        .unless(&|w| w.has("note", DIRTY))
        .print("You've gotten it too dirty to read.");

    // The note gets dirty if the player picks it up with dirty hands.
    wb.on(&GetThing("note"))
        .when(&|w| w.has("hands", DIRTY) && !w.has("note", DIRTY))
        .print("The dirt from your hands got all over the note.")
        .set_flag("note", DIRTY);

    // Room: Grotto
    wb.room("grotto", "A Grotto in the Woods")
        .link(West, "clearing")
        .prose("\
Nestled in a grotto among the trees you find a pool of water.
A path leads west.
        ")
        .flag(HAS_WATER);

    // Feature: Pool, a pool in the Grotto
    wb.feature("pool", "pool", "pool")
        .location("grotto")
        .on_examine("\
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
    wb.feature("stone", "stone", "stone")
        .location("hilltop")
        .on_scenery("\
A massive block of stone squats on the crest of the hill.  There seems to be a sword hilt
poking out of the top, and there's something on one of the sides.
        ")
        .on_examine("\
It's a massive block of marble, four feet wide and three feet high.  The top is flat, and the
four sides slope inward.  There's a sword sticking out of the top.  These words are chiseled
into one side:
||   * Only The Pure *
        ");

    // Thing: The Sword in the Stone on the Hilltop
    wb.thing("sword", "sword", "sword")
        .location("hilltop")
        .flag(Scenery) // It will appear as part of the stone until removed.
        .on_examine_hook(&|w,e,buff| {
            if w.has(e, TAKEN) {
                buff.puts("\
The sword, if you want to call it that, is a three-foot length of dark hardwood
with a sharkskin hilt on one end.  It's polished so that it gleams, and it has no
sharp edges anywhere.  Carved along the length of it are the words
\"Emotional Support Sword (TM)\".
                ");
            } else {
                buff.puts("All you can really see is the hilt; the rest is embedded in the stone.");
            }
        });

    // If the player tries to pick up the sword with dirty hands, it kills him.
    wb.allow(&GetThing("sword"))
        .unless(&|w| w.has("hands", DIRTY))
        .print("\
Oh, you so didn't want to touch the sword with dirty hands.
Weren't you paying attention? Only the pure may touch this sword.
        ")
        .kill(PLAYER);

    // When the player takes the sword successfully, magic stuff happens.
    wb.on(&GetThing("sword"))
        .once_only()
        .forget("stone") // Move it to LIMBO
        .set_flag("sword", TAKEN)
        .unset_flag("sword", Scenery)
        .print("\
The sword almost seems to leap into your hands.  As you marvel at it
(and, really, there's something odd about it), the marble block dissolves
into white mist and blows away.
        ");

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

    // The player can't enter the cave without the sword.
    wb.allow(&EnterRoom("cave-1"))
        .unless(&|w| !w.owns(PLAYER, "sword"))
        .print("\
Oh, hell, no, you're not going in there empty handed.  You'd better go back
and get that sword.
        ");

    // The first time the player enters the cave, magic happens.
    wb.on(&EnterRoom("cave-1"))
        .once_only()
        .print("\
It's an unpleasant place but your sword gives you confidence and warm fuzzies.
        ");

    // If the player dies, the fairy godmother revives him.
    wb.rule("fairy-godmother-rule")
        .when(&|w| w.has(PLAYER, Dead))
        .print("\
A fairy godmother hovers over your limp body.  She frowns;
then, apparently against her better judgment, she waves
her wand.  There's a flash, and she disappears.
        ")
        .revive(PLAYER);

    // NEXT, add custom commands.
    // NOTE: Order is important!

    wb.verb_noun("wash", "hands", &|w,_,script| {
        if !w.has(&w.loc(PLAYER), HAS_WATER) {
            return Err("That'd be a neat trick, since there's no water here.".into());
        }

        // TODO: Provide actions that build up paragraphs?
        let mut buff = ProseBuffer::new();
        buff.puts("You wash your hands in the water.");
        if w.has("hands", DIRTY) {
            buff.puts("They look much cleaner now.");
        }

        script.print(&buff.get());
        script.unset_flag("hands", DIRTY);

        Ok(())
    });

    wb.verb_visible("wash", &|_,_,script| {
        script.print("You can't wash that.");
        Ok(())
    });


    // NEXT, return the world.
    wb.world()
}
