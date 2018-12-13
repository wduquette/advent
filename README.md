# Bonaventure: A simple text adventure in Rust

A simple text adventure, written as a way to learn the Rust language,
using the ECS architecture.

## To Do

* Add dictionary: preferred words with synonyms.
* Convert input from user's words to preferred words before pattern
  matching.
* Rename RuleComponent to Rule
* Rename ThingComponent to ThingInfo
* Add puzzle to make water flow
* Get note dirty and illegible.
* Review current code.
* Extend the world.
* Add some puzzles.
* Add more story.
* Add winning condition.

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
(and few objects); and there are no puzzles and no way to win the game.

What it does have is a data model that would support a real game and
could be extended cleanly in all sorts of ways. At present, the player can:

* Move from room to room
* Pick up and drop objects
* Query his inventory
* Examine the room he's in

There are also a couple of debugging commands.

## The Internals

The game world consists of a vector of "entities".  Each entity has a tag,
which is used for debugging, and can also be used to look up the entity's
ID; entities may also have the following components:

* A name, for display
* A descriptive text string
* A location (for when it is contained in another entity)
* A set of links to other entities (i.e., the trail is East of the clearing)
* Details of things with which the player interacts (i.e., can he
  pick it up)
* An inventory of entities contained within this entity
* A Rule: under certain conditions, do something automatically.

Thus,

* The player has a name, a location, and an inventory.
* A room has a name a description, links, and an inventory.
* A thing has a name, a description, a location, is marked portable, and
  is found in some other entity's inventory.
* Scenery (an immobile thing) has a name, a description, a location, and
  is marked non-portable.
* A back-story element has a description and is printed when the conditions
  are right (e.g., on the third turn)

The point is, these categories of things are not classes in the OOP sense.
The Entity struct has Option<T> fields for each of the above components;
and by mixing and matching the components you can create almost any kind of
of object, e.g., a magical car that is an object that can appear in a room
(or be put in the player's pocket) but which the player can also get into
and drive.  

The Entity struct and its components are almost pure data; some of the
component structs have `new()` methods, but otherwise all of the logic
is in two places.

* The World struct provides convenience methods for querying and mutating
  the game world at a very low level.  

* The game's "systems" define the game logic that determines how the game
  world mutates each turn, e.g., how to process the player's commands.  

## Ideas for the Future

### NPCs and Monsters

These would be entities with behavior.  Behavior could be implemented as
a BehaviorComponent that takes a closure as a value; but it's probably
easier to define an Enum with values for each kind of behavior the NPC
or monster might have: sneaking, fighting, running away in fear,
patrolling.  These behaviors could be configured using Enum constant
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

### Book/Note Content

A "Thing" can have additional prose, as the ID of a prose-only component,
e.g., so you can examine a book and then read it.  Alternatively, just
add another text component, description vs. prose.
