//! The event component.  It stores the entity's event guards and hooks

use crate::types::EntityEventHook;
use std::collections::HashMap;
use crate::entity::ID;
use crate::world::World;
use crate::types::EventType;
use std::fmt;

/// A hook to modify the world based on an event occuring to an entity.
/// We define this struct because we can't add traits to EntityEventHook.
#[derive(Clone)]
pub struct EventHook {
    pub hook: EntityEventHook,
}

impl EventHook {
    /// Creates the hook
    pub fn new(hook: EntityEventHook) -> Self {
        Self { hook }
    }
}

impl fmt::Debug for EventHook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EventHook(...)")
    }
}

/// Information specific to entity event guards and hooks
#[derive(Debug, Clone, Default)]
pub struct EventComponent {
    pub hooks: HashMap<EventType,EventHook>,
}

impl EventComponent {
    /// Create a new room with a name, noun, visual, and related info.
    pub fn new() -> EventComponent {
        EventComponent {
            hooks: HashMap::new(),
        }
    }

    /// Call the hook
    pub fn call_hook(&mut self, world: &mut World, id: ID, event_type: EventType) {
        if let Some(event_hook) = self.hooks.get(&event_type) {
            (event_hook.hook)(world, id, event_type);
        }
    }
}

//------------------------------------------------------------------------------------------------
// Event View

/// Event view: A view of an entity as a read-only collection of event
pub struct EventView {
    pub id: ID,
    pub tag: String,
    pub event: EventComponent,
}

impl EventView {
    /// Creates a EventView for the entity.
    pub fn from(world: &World, id: ID) -> Self {
        let tc = &world.tags[&id];

        assert!(world.is_event(id), "Not event: [{}] {}", tc.id, tc.tag,);

        Self {
            id: tc.id,
            tag: tc.tag.clone(),
            event: world.events[&id].clone(),
        }
    }
}
