# Bonaventure: A simple text adventure in Rust

A simple text adventure, written as a way to learn the Rust language,
using the ECS architecture.

## To Do

* Extend the world.
* Implement objects.
* Implement inventories.
  * The player has one, but objects can have them as well.

## Questions

* Should the player be contained in a location's inventory?
  * For multiple players, probably.  For a single player game, probably not.

## Ideas for the Future

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

Of course, once we add monsters/npcs it's possible that they can interrupt
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
