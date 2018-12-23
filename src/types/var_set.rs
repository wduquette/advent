//! Flags and FlagSets
use std::collections::HashSet;
use super::ID;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
#[allow(dead_code)]
/// A game variable
pub enum Flag {
    /// Has this entity been seen by the player?  Used mostly for locations.
    Seen(ID),

    /// Does the player have dirty hands?
    DirtyHands,

    /// Does the location have clean water?
    HasWater,

    /// Is the thing Scenery?
    Scenery,

    /// Is the thing dirty?
    Dirty,
}

/// A set of flag values.
#[derive(Debug, Clone)]
pub struct FlagSet {
    set: HashSet<Flag>
}

impl FlagSet {
    pub fn new() -> FlagSet {
        FlagSet {
            set: HashSet::new(),
        }
    }
    
    /// Determines whether the set has the given flag setting.
    pub fn has(&self, flag: Flag) -> bool{
        self.set.contains(&flag)
    }

    /// Sets the flag.
    pub fn set(&mut self, flag: Flag) {
        self.set.insert(flag);
    }

    /// Unsets the flag
    pub fn unset(&mut self, flag: Flag) {
        self.set.remove(&flag);
    }

    /// Replaces all other occurrences of the same flag variant
    /// with this flag.
    pub fn replace(&mut self, flag: Flag) {
        // FIRST, remove all other occurrences of the same variant, e.g., Seen(_).
        let disc = std::mem::discriminant(&flag);
        self.set = self
            .set
            .iter()
            .map(|f| *f)
            .filter(|f| std::mem::discriminant(f) != disc)
            .collect();

        // NEXT, add the new one.
        self.set(flag);
    }

    pub fn iter(&self) -> std::collections::hash_set::Iter<'_, Flag> {
        self.set.iter()
    }
}
