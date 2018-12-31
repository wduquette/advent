//! Entity Flags

use crate::types::Flag;
use std::collections::HashSet;

/// Inventories of Things
#[derive(Debug, Clone, Default)]
pub struct FlagSetComponent {
    /// A set of things in the inventory.  We use a BTreeSet so that we preserve the order
    /// in which things were added.
    pub set: HashSet<Flag>,
}

impl FlagSetComponent {
    /// Create a new inventory component
    pub fn new() -> FlagSetComponent {
        FlagSetComponent {
            set: HashSet::new(),
        }
    }

    /// Gets the number of flags in the set.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.set.len()
    }

    /// Determines whether the set has the given flag setting.
    pub fn has(&self, flag: Flag) -> bool {
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

    pub fn iter(&self) -> std::collections::hash_set::Iter<'_, Flag> {
        self.set.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Flag::*;

    #[test]
    fn new() {
        let set = FlagSetComponent::new();
        assert_eq!(set.len(), 0);
        assert!(!set.has(Seen(1)));
    }

    #[test]
    fn set() {
        let mut set = FlagSetComponent::new();

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
        let mut set = FlagSetComponent::new();
        set.set(Seen(1));
        set.set(Seen(2));

        set.unset(Seen(1));
        assert_eq!(set.len(), 1);
        assert!(!set.has(Seen(1)));
        assert!(set.has(Seen(2)));
    }

    #[test]
    fn iter() {
        let mut set = FlagSetComponent::new();
        set.set(Seen(1));
        set.set(Seen(2));

        let copy: HashSet<Flag> = set.iter().cloned().collect();
        assert_eq!(copy.len(), 2);
        assert!(copy.contains(&Seen(1)));
        assert!(copy.contains(&Seen(2)));
    }
}
