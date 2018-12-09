//! Scenario definition

use crate::types::Dir::*;
use crate::types::*;
use crate::world::*;

/// Build the initial state of the game world.
pub fn build() -> World {
    // FIRST, make the empty world
    let mut the_world = World::new();
    let world = &mut the_world;

    // NEXT, make the rooms.

    // Rooms
    let clearing = make_room(
        world,
        "Clearing",
        "A wide spot in the woods.  You can go east.",
    );
    let trail = make_room(
        world,
        "Trail",
        "A trail from hither to yon.  You can go east or west.",
    );
    let bridge = make_room(
        world,
        "Bridge",
        "\
The trail crosses a small stream here.  You can go east or west.
        ",
    );

    // Links
    connect(world, East, clearing, West, trail);
    connect(world, East, trail, West, bridge);

    // NEXT, make the things
    let note = make_portable(world, "note", "It's illegible.");
    world.put_in(note, clearing);

    make_scenery(
        world,
        bridge,
        "stream",
        "\
The stream comes from the north, down a little waterfall, and runs
away under the bridge.  It looks surprisingly deep, considering
how narrow it is.
        ",
    );

    // Stories: Triggers that supply backstory to the player.
    make_story(
        world,
        "Story-1",
        |world| world.clock == 2,
        "\
You don't know where you are.  You don't even know where you want to
be.  All you know is that your feet are wet, your hands are dirty,
and gosh, this doesn't look anything like the toy aisle.
    ",
    );

    // NEXT, initialize the player
    initialize_player(world, clearing);

    // NEXT, return the world.
    the_world
}

/// Initializes the player's details
fn initialize_player(world: &mut World, start: ID) {
    let pid = world.player;
    let player = &mut world.entities[pid];

    player.name = "You".into();
    player.prose = Some(ProseComponent {
        text: "You've got all the usual bits.".into(),
    });
    player.loc = Some(start);
    player.inventory = Some(InventoryComponent::new());
}

/// Makes a room with the given name and prose, and an empty set of links.
/// Returns the room's ID.
fn make_room(world: &mut World, name: &str, text: &str) -> ID {
    let rid = world.alloc();
    let room = &mut world.entities[rid];

    room.name = name.into();
    room.prose = Some(ProseComponent {
        text: text.trim().into(),
    });
    room.links = Some(LinksComponent::new());
    room.inventory = Some(InventoryComponent::new());

    rid
}

/// Makes a portable object, and returns its ID.
fn make_portable(world: &mut World, name: &str, text: &str) -> ID {
    let id = world.alloc();
    let thing = &mut world.entities[id];

    thing.name = name.into();
    thing.prose = Some(ProseComponent {
        text: text.trim().into(),
    });
    thing.thing = Some(ThingComponent { portable: true });

    id
}

/// Makes a scenery object, and returns its ID.
fn make_scenery(world: &mut World, loc: ID, name: &str, text: &str) -> ID {
    let id = world.alloc();
    let thing = &mut world.entities[id];

    thing.name = name.into();
    thing.prose = Some(ProseComponent {
        text: text.trim().into(),
    });
    thing.loc = Some(loc);
    thing.thing = Some(ThingComponent { portable: false });

    id
}

/// Adds a bit of backstory to be revealed when the conditions are right.
/// Backstory will appear only once.
fn make_story<F: 'static>(world: &mut World, name: &str, predicate: F, text: &str)
where
    F: Fn(&World) -> bool,
{
    let tid = world.alloc();
    world.entities[tid].name = format!("Trigger {}", name);
    world.entities[tid].prose = Some(ProseComponent {
        text: text.trim().into(),
    });

    world.entities[tid].trigger = Some(TriggerComponent {
        predicate: Box::new(predicate),
        action: Action::Print,
        once_only: true,
        fired: false,
    });
}

/// Links one room to another in the given direction.
/// Links are not bidirectional.  If you want links both ways, you
/// have to add them.
fn oneway(world: &mut World, dir: Dir, from: ID, to: ID) {
    let links = &mut world.entities[from]
        .links
        .as_mut()
        .expect(&format!("Entity has no link component: {}", from));

    links.map.insert(dir, to);
}

/// Establishes a bidirectional link between two rooms.
fn connect(world: &mut World, from_dir: Dir, from: ID, to_dir: Dir, to: ID) {
    oneway(world, from_dir, from, to);
    oneway(world, to_dir, to, from);
}
