# Bonaventure: A simple text adventure in Rust

A simple text adventure, written as a way to learn the Rust language,
using something like the ECS architecture.

## To Do

Also, see docs/journal.txt.

* Complete the physical system, phys::, and use it through out.
  * And then remove the relevant methods out of World.
* Move rule_component::Action to types::
* Debug commands shouldn't advance the clock.
* Define Event guards and hooks
  * The dirty-note rule should be a Get hook.
  * The note Book prose hook could be a Read guard.
* The Game
  * Add emotional comfort sword.
    * Kills you if you draw it when your hands are dirty.
      * But the fairy godmother will revive you.
  * Main loop should halt if you're dead at the end of it.
  * Rules map should be BTreeMap, so that the order of
    rule execution is predictable.
  * Add puzzle to make water flow in the stream.
  * Extend the world.
  * Add some more things and more puzzles.
  * Add more story.
  * Add NPCs, monsters.
    * Maybe less of a standard text adventure, more of a text-based
      RPG?
  * Add winning condition.
* Add save/restore
* Add `stuff` system
  * For finding and listing things in the local environment.
  * Can support queries of various kinds.
  * Goal is to simplify the command handlers.
  * Don't try to do too much too quickly.
* Improve vocabulary and grammar management
  * Consider design where verbs (operations) depend on
    species, i.e., each thing knows what verbs can be
    used on it.
  * Support inputs containing multiple commands, with command queue.
  * Allow two-word synonyms for verbs as part of basic parsing.
  * Allow scenario to define verbs and custom command handlers.
  * Support simple patterns, e.g.,
    * ["drop", noun]
    * ["give", noun,"to", npc]
    * ["hit", npc, "with", noun]
  * Possibly, compile commands to action lists.
* Extend `visual` system
  * `visual::room()` should include descriptions of portable things as
    prose in the basic description, and maybe of some scenery items as
    well.
    * Need articles for things.

## Background

I wrote Bonaventure as an exercise to familiarize myself with the
Rust language.  I chose to implement a simple text adventure game because
that's been my chosen "hello, world" project since the early 80's; it's
complicated enough to require real understanding of the chosen language,
but nicely self-contained.

Jonathan Castello suggested I look at the [keynote](https://kyren.github.io/2018/09/14/rustconf-talk.html) from the Rust 2018 conference, which talked about the
[entity-component-system (ECS)](https://en.wikipedia.org/wiki/Entity–component–system) architecture as applied to game implementation in Rust.  Of particular interest to me was
the section where the author explains why OOP fails for game programming
(which it does, as I've experienced myself in the past) and how ECS solves
the same problems more cleanly.  It's a brilliant paper, and got
me headed the right direction.

As a text adventure, Bonaventure is (at present) dirt simple.  It doesn't
have a fancy natural language parser; it has only a very few rooms
(and few objects); and there are few puzzles and no way to win the game.

What it does have is a data model that would support a real game and
could be extended cleanly in all sorts of ways. At present, the player can:

* Move from room to room
* Pick up and drop objects
* Query his inventory
* Examine the room he's in

There are also a couple of debugging commands.

## The Internals

The game world consists of entities, each of which is made up of
components.  Each entity has an integer ID; an entity's components are
stored in a set of hash tables.  An entity may have the following
components:

* The TagComponent, which contains the entity's ID and a string tag
  used for lookups and as a debugging aid.  Every entity has a
  TagComponent
* A PlayerComponent, for the player entity
* A RoomComponent, for places a player can be
* A ThingComponent, for things a player can interact with
* Etc.

We build up complex
entity types not by class-based inheritance, but by composing the entity
out of components, e.g., a vehicle is a thing in a room AND a room the
player can be in; it will have both a ThingComponent and a RoomComponent.

The entities themselves have very little logic attached to them.

* The World struct (which contains the entities) provides convenience
  methods for querying and mutating the game world at a very low level.

* The World struct allows for acquiring "view" objects focussed on a
  particular role, e.g., `as_room()` and `as_thing()`.  Views can be
  mutated and the result saved back into the World.  

* The bulk of the logic is in the game's "systems", which define how
  the entities appear to the player (the `visual` system), how the
  player's commands are processed (the `player_control` system), etc.

## Ideas for the Future

### Fancy Undo

At present Bonaventure supports undoing the very last command.  This is
problematic, as it doesn't distinguish between commands that mutate the world
and commands that don't; and in fact, it's difficult to distinguish between
the two.  Even "look" takes time and so updates the clock; and a rule might
fire or an NPC move during that time.

Consequently, we may want a multi-level undo; and if so, we certainly need to
tell the user what was undone.

### NPCs and Monsters

These would be entities with behavior.  Behavior could be implemented as
a BehaviorComponent that takes a closure or functional pointer as a value;
but it's probably easier to define an Enum with values for each kind of
behavior the NPC or monster might have: sneaking, fighting, running away in
fear, patrolling.  These behaviors could be configured using Enum constant
fields, e.g., the monster might fight until its health decreases past
a threshold at which point it runs.  The threshold could be part of the
Fight enum:

```
enum Behavior {
    Fight(run_threshold),
    ...
}
```

### Commands with duration

At present the clock increments for each user input, regardless of what it
is.  Ideally, different commands should have different durations.  Errors
should have no duration.  Some commands, like checking your inventory,
should have no duration as well.  In principle, it's possible that some
commands should take longer than one turn.

### Multiple Commands

A command line can have multiple commands separated by periods.  Since
commands should have durations, we need to manage that carefully.
Probably entered commands should get pushed into a queue; and the
player_control::system() should process commands until it hits an error
or time has passed.

Of course, once we add monsters/NPCs it's possible that they can interrupt
the command queue as well.

### Fancy Destinations

Consider making the links map be `HashMap<Dir,Dest>` instead of
`HashMap<Dir,ID>`, where `Dest` is an Enum:

```
enum Dest {
    Go(location_entity_id),
    DeadEnd(prose_entity_id),
    Guarded(location_entity_id, predicate, prose_entity_id),
    ...
}
```

Here, `Go` means just link there; `DeadEnd` means you can't go that way,
but there's a special message; `Guarded` means you can only go there if
a predicate condition is met, and you'll get a `DeadEnd` message otherwise.

### Dictionary Content

A number of inform games have "dictionaries", in which the user needs
to look things up.  This could be a dictionary or encyclopedia, a set
of mail slots, a corridor with lots of numbered offices, a phone book,
or what have you. The essence is that the user isn't allowed to simply
search through all the possibilities; he has to know what he's looking
for, e.g., Prof. Plum's mail slot is number 47.

### Expression Syntax

At present Rules take a closure |&World| -> bool as the predicate.  If
I were reading the game from a scenario file, though, I'd need some
kind of expression Syntax, probably translated to a syntax tree
represented by enum values.  It could be useful anyway.

There might be some crate that provides this.

* There doesn't seem to be any good crate providing basic boolean/arithmetic
  expression parsing and evaluation.
  * `calculate` doesn't seem to offer boolean expressions, and the
    documentation is lacking.
  * `pupil` is only arithmetic

There are some scripting language possibilities.  These would also allow
writing rule and command actions in the scenario.

* `rhai`
* gluon-lang/gluon
* PistonDevelopers/dyon
