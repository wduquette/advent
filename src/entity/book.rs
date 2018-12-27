//! Books: Things that can be read.

use crate::entity::flag::FlagSetComponent;
use crate::entity::thing::ThingComponent;
use crate::entity::ID;
use crate::world::World;

/// Books: things that can be read.
#[derive(Debug, Clone)]
pub struct BookComponent {
    pub text: String,
}

impl BookComponent {
    /// Creates a new component.
    pub fn new(text: &str) -> Self {
        Self { text: text.into() }
    }
}

//------------------------------------------------------------------------------------------------
// Book View

/// Book view: A view of an entity as a Book
pub struct BookView {
    pub id: ID,
    pub tag: String,
    pub thing: ThingComponent,
    pub flag_sets: FlagSetComponent,
    pub book: BookComponent,
}

impl BookView {
    /// Creates a BookView for the entity.
    pub fn from(world: &World, id: ID) -> BookView {
        let tc = &world.tags[&id];

        assert!(world.is_book(id), "Not a book: [{}] {}", tc.id, tc.tag,);

        BookView {
            id: tc.id,
            tag: tc.tag.clone(),
            thing: world.things[&id].clone(),
            flag_sets: world.flag_sets[&id].clone(),
            book: world.books[&id].clone(),
        }
    }

    /// Save the book back to the world.  Replaces the links and inventory.
    pub fn save(&self, world: &mut World) {
        world.things.insert(self.id, self.thing.clone());
        world.flag_sets.insert(self.id, self.flag_sets.clone());
        world.books.insert(self.id, self.book.clone());
    }
}
