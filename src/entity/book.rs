//! Books: Things that can be read.

/// Books: things that can be read.
#[derive(Debug, Clone)]
pub struct BookComponent {
    pub text: String,
}

impl BookComponent {
    /// Creates a new component.
    pub fn new(text: &str) -> Self {
        Self {
            text: text.into(),
        }
    }
}
