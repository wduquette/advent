# Bonaventure: A simple text adventure in Rust

A simple text adventure, written as a way to learn the Rust language,
using something like the ECS architecture.

## To Do

Also, see docs/journal.txt.

* Move flag methods to flag::has, flag::set, flag::unset, following the
  component architecture.
* Ponder how to define custom commands:
  * e.g., "wash hands".
* The Game
  * Add more story.
  * Main loop should halt if you're dead at the end of it.
  * Add NPCs, monsters.
    * Maybe less of a standard text adventure, more of a text-based
      RPG?
  * Add winning condition.
* Debug commands shouldn't advance the clock.
* Add save/restore
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
have a fancy natural language parser; it has only a few rooms, and
objects, and a couple of puzzles; and there's no way to win the game.

What it does have is a data model that would support a real game and
could be extended cleanly in all sorts of ways. At present, the player can:

* Move from room to room
* Pick up and drop objects
* Query his inventory
* Examine the room he's in and the things he sees.

The engine also includes a WorldBuilder API that allows the scenario author
to add rooms, things, etc., easily, and to customize their behavior
and visuals using hooks.  See scenario.rs for the example.

## The Internals

The game world consists of entities, each of which is made up of
components.  Each entity has a text tag and an integer ID; an
entity's components are stored in a set of hash tables index on the
ID.  An entity may have the following components:

* The TagComponent, which contains the entity's tag and ID.
  Every entity has a TagComponent
* A PlayerComponent, for the player entity
* A RoomComponent, for places a player can be
* A ThingComponent, for things a player can interact with
* A RuleComponent, for rules that change the default behavior
* A LocationComponent, for where a thing or player is
* An InventoryComponent, for things that a room or player contains.
* A FlagSetComponent, for flags that can be set on the entity
* A ProseComponent, for prose (or hooks) used to describe the entity

We build up complex entity types not by class-based inheritance, but by
composing the entity out of components, e.g., a vehicle is a thing in a
room AND a room the player can be in; it will have both a ThingComponent
and a RoomComponent.

The entities themselves have very little logic attached to them.
The bulk of the logic is in the game's "systems":

* The `visual` system, which controls how entities appear to the
  player, along with other visual output.
* The `phys` (physical) system, which is responsible for managing how
  entities are related to each other (i.e., where they are located
  and moved)
* The `rule` system, which allows the scenario to define special
  rules that are triggered by various events.
* The `player_control` system which processes the player's commands.

## Ideas for the Future

### Multiple Commands

A command line can have multiple commands separated by periods.  Since
commands should have durations, we need to manage that carefully.
Probably entered commands should get pushed into a queue; and the
player_control::system() should process commands until it hits an error
or time has passed.

Of course, once we add monsters/NPCs it's possible that they can interrupt
the command queue as well.

### Commands with duration

At present the clock increments for each user input, regardless of what it
is.  Ideally, different commands should have different durations.  Errors
should have no duration.  Some commands, like checking your inventory,
should have no duration as well.  In principle, it's possible that some
commands should take longer than one turn.

### Ambient Sound

It would be cool to manage sources of sound (conveyed, as always, by prose).
Sounds could be Quiet, Medium, or Loud.  Quiet sounds you can only hear if
you "listen"; others appear as part of the normal room description.  
Volume decreases with distance; Loud sounds can be heard in
adjoining rooms without explicitly listening, and Medium sounds can be heard
if you "listen".

To do this properly we would need a notion of the distance between two rooms;
just because they are adjacent in the link map doesn't mean they are close
to each other.  A link could be a long road, for example.

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

### Fancy Undo

At present Bonaventure supports undoing the very last command.  This is
problematic, as it doesn't distinguish between commands that mutate the world
and commands that don't; and in fact, it's difficult to distinguish between
the two.  Even "look" takes time and so updates the clock; and a rule might
fire or an NPC move during that time.

Consequently, we may want a multi-level undo; and if so, we certainly need to
tell the user what was undone.

Alternatively, we can design the game so that undo isn't needed.  And for
some games, undo is undesirable (i.e., if combat is a real thing)

### Dictionary Content

A number of inform games have "dictionaries", in which the user needs
to look things up.  This could be a dictionary or encyclopedia, a set
of mail slots, a corridor with lots of numbered offices, a phone book,
or what have you. The essence is that the user isn't allowed to simply
search through all the possibilities; he has to know what he's looking
for, e.g., Prof. Plum's mail slot is number 47.

### Expression Syntax

At present rules take a closure |&WorldQuery| -> bool as the predicate.  If
I were reading the game from a scenario file, though, I'd need some
kind of expression syntax, probably translated to a syntax tree
represented by enum values.

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
