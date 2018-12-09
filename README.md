# Advent in Rust

A simple text adventure, written as a way to learn the Rust language,
using the ECS architecture.

## To Do

* Review the code, looking for utilities to factor out or move.
* Extend the world.
* Implement objects.
* Implement inventories.
  * The player has one, but objects can have them as well.


## Ideas

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
