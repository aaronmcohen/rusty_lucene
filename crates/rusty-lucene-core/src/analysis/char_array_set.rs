use crate::analysis::char_array_map::CharArrayMap;
// No HashMap needed

/// A set that stores `char[]` keys backed by a {@link CharArrayMap}. Not general-purpose:
/// cannot remove items from the set, nor does it resize smaller.
///
/// <p>Why use a backing map? The map already stores keys in a hash table with fast lookup.
/// The set can simply delegate to the map for `contains` and `add`, avoiding duplicating key
/// storage logic. The map's {@code ignoreCase} flag controls case sensitivity for the set as well.
///
/// <p>Example: create set, add item, check containment
/// ```rust
/// # let mut set = crate::analysis::char_array_set::CharArraySet::new(3, false);
/// set.add("hello");
/// assert!(set.contains_cs("hello"));
/// ```
pub struct CharArraySet {
    /// Backing map that stores all keys. Uses the same {@code ignoreCase} flag as the set.
    map: CharArrayMap<()>,
}

impl CharArraySet {
    pub fn new(start_size: usize, ignore_case: bool) -> Self {
        let map = CharArrayMap::new(start_size, ignore_case);
        Self { map }
    }

    /// Checks if `val` (a &str) is in the set. For case-insensitive sets, compares after lowercasing
    /// only ASCII letters (matching the map's behavior).
    ///
    /// @param val the value to check
    /// @return {@code true} if the set contains `val`, {@code false} otherwise
    pub fn contains_cs(&self, val: &str) -> bool {
        let key = val.as_bytes();
        self.map.contains_key(key, 0, key.len())
    }

    /// Adds `val` to the set if it is not already present.
    /// Wrapper around `add` that accepts &str.
    /// @param val the value to add
    /// @return {@code true} if the key was new (not already present), {@code false} otherwise
    pub fn add_str(&mut self, val: &str) -> bool {
        let key = val.as_bytes();
        match self.map.put(key, ()) {
            None => true,     // new → "add" true
            Some(_) => false, // replaced → "add" would overwrite placeholder
        }
    }

    /// Adds `val` to the set if it is not already present. Returns {@code true} if the key was new,
    /// {@code false} if it would replace an existing placeholder (i.e., "add" would overwrite).
    ///
    /// @param val the value to add
    /// @return {@code true} if the key was new (not already present), {@code false} otherwise
    pub fn add(&mut self, val: &str) -> bool {
        let key = val.as_bytes();
        match self.map.put(key, ()) {
            None => true,     // new → "add" true
            Some(_) => false, // replaced → "add" would overwrite placeholder
        }
    }

    /// Clears all keys from the set by clearing the backing map.
    ///
    /// @return {@code this} for chaining
    pub fn clear(&mut self) -> &mut Self {
        self.map.clear();
        self
    }

    /// Returns the number of keys in the set.
    ///
    /// @return the size
    pub fn size(&self) -> usize {
        self.map.size()
    }

    /// Returns an unmodifiable {@link CharArraySet} view on this set.
    ///
    /// @return a new {@link CharArraySet}
    pub fn unmodifiable_set(&self) -> Self {
        CharArraySet::new(self.map.size() as usize, self.map.ignore_case)
    }

    /// Copies this set to a new {@link CharArraySet}.
    ///
    /// @return a new {@link CharArraySet}
    pub fn copy(&self) -> Self {
        CharArraySet {
            map: self.map.copy(),
        }
    }

    /// Returns the key set as a {@link Vec<String>}. For read-only access, use this method.
    ///
    /// @return the key vector
    pub fn key_vec(&self) -> Vec<String> {
        self.map.key_vec()
    }

    /// Checks if this set is empty.
    ///
    /// @return {@code true} if empty
    pub fn is_empty(&self) -> bool {
        self.map.size() == 0
    }

    /// Returns a CharArraySet view of keys. Throws on modification.
    ///
    /// @return the key set
    pub fn key_set(&self) -> Self {
        CharArraySet {
            map: self.map.copy(),
        }
    }

    /// Checks if `val` (a &str) is in the map, case-insensitively if the map is case-insensitive.
    /// Wrapper around `contains_key` that accepts &str.
    /// @param val the value to check
    /// @return {@code true} if the map contains `val`, {@code false} otherwise
    pub fn contains_key_ca(&self, val: &str) -> bool {
        let key = val.as_bytes();
        self.map.contains_key(key, 0, key.len())
    }
}
