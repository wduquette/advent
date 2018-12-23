//! Flags and FlagSets
use std::collections::HashSet;
use super::ID;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
#[allow(dead_code)]
/// A game flag
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
    /// Creates a new flag set.
    pub fn new() -> FlagSet {
        FlagSet {
            set: HashSet::new(),
        }
    }

    /// Gets the number of flags in the set.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.set.len()
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
    #[allow(dead_code)]
    pub fn replace(&mut self, flag: Flag) {
        // FIRST, remove all other occurrences of the same variant, e.g., Seen(_).
        let disc = std::mem::discriminant(&flag);
        self.set = self
            .set
            .iter()
            .cloned()
            .filter(|f| std::mem::discriminant(f) != disc)
            .collect();

        // NEXT, add the new one.
        self.set(flag);
    }

    pub fn iter(&self) -> std::collections::hash_set::Iter<'_, Flag> {
        self.set.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Flag::*;

    #[test]
    fn new() {
        let set = FlagSet::new();
        assert_eq!(set.len(), 0);
        assert!(!set.has(Seen(1)));
    }

    #[test]
    fn set() {
        let mut set = FlagSet::new();

        set.set(Seen(1));
        assert_eq!(set.len(), 1);
        assert!(set.has(Seen(1)));
        assert!(!set.has(Seen(2)));

        set.set(Seen(2));
        assert_eq!(set.len(), 2);
        assert!(set.has(Seen(1)));
        assert!(set.has(Seen(2)));
    }

    #[test]
    fn unset() {
        let mut set = FlagSet::new();
        set.set(Seen(1));
        set.set(Seen(2));

        set.unset(Seen(1));
        assert_eq!(set.len(), 1);
        assert!(!set.has(Seen(1)));
        assert!(set.has(Seen(2)));
    }

    #[test]
    fn replace() {
        let mut set = FlagSet::new();
        set.set(Seen(1));
        set.set(Seen(2));

        set.replace(Seen(3));

        assert_eq!(set.len(), 1);
        assert!(!set.has(Seen(1)));
        assert!(!set.has(Seen(2)));
        assert!(set.has(Seen(3)));
    }

    #[test]
    fn iter() {
        let mut set = FlagSet::new();
        set.set(Seen(1));
        set.set(Seen(2));

        let copy: HashSet<Flag> = set.iter().cloned().collect();
        assert_eq!(copy.len(), 2);
        assert!(copy.contains(&Seen(1)));
        assert!(copy.contains(&Seen(2)));
    }
}
