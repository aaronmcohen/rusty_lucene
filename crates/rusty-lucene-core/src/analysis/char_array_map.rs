use crate::analysis::{ASCII_A_MIN, ASCII_Z_MAX, A_TO_Z_LOWER_OFFSET};

pub struct CharArrayMap<V: Default + Clone> {
    /// `None` means empty slot; `Some(Vec<u8>)` holds key bytes for occupied slots.
    keys: Vec<Option<Vec<u8>>>,
    /// Value for each slot.
    values: Vec<V>,
    /// Number of occupied slots.
    pub count: usize,
    /// `true` when case should be ignored for key matching and insertion.
    pub ignore_case: bool,
}

impl<V: Default + Clone> CharArrayMap<V> {
    /// Minimum capacity starting at 8 and doubling while `startSize + startSize/4` exceeds capacity.
    fn min_capacity(start_size: usize) -> usize {
        let mut size = 8;
        while start_size + (start_size >> 2) > size {
            size <<= 1;
        }
        size
    }

    /// Lowercases only ASCII letters ('A'-'Z') by adding 32 when `ignore_case` is true.
    /// Non-letters pass through unchanged. Matches original `cmp` behavior which only transforms letters.
    fn normalize_key(&self, key: &[u8]) -> Vec<u8> {
        if self.ignore_case {
            let mut out = key.to_vec();
            for i in 0..out.len() {
                let b = out[i];
                if b >= ASCII_A_MIN && b <= ASCII_Z_MAX {
                    out[i] = b + A_TO_Z_LOWER_OFFSET;
                }
            }
            out
        } else {
            key.to_vec()
        }
    }

    /// Compares key text (at `off`, length `len`) with stored key `key2`. When `ignore_case`,
    /// only ASCII letters are compared in lowercase form; exact match for non-letters. Matches original `cmp`.
    fn equals_key(&self, text: &[u8], off: usize, len: usize, key2: &Vec<u8>) -> bool {
        if len != key2.len() {
            return false;
        }
        if self.ignore_case {
            for i in 0..len {
                let b1 = text[off + i];
                let nb1 = if b1 >= ASCII_A_MIN && b1 <= ASCII_Z_MAX {
                    b1 + A_TO_Z_LOWER_OFFSET
                } else {
                    b1
                };
                let b2 = key2[i];
                let nb2 = if b2 >= ASCII_A_MIN && b2 <= ASCII_Z_MAX {
                    b2 + A_TO_Z_LOWER_OFFSET
                } else {
                    b2
                };
                if nb1 != nb2 {
                    return false;
                }
            }
            return true;
        } else {
            for i in 0..len {
                if text[off + i] != key2[i] {
                    return false;
                }
            }
            return true;
        }
    }

    /// Computes hash code for key text at offset `off` with length `len`. Uses 31 multiplier.
    /// When `ignore_case`, letters are lowercased before hashing so case-insensitive maps have
    /// consistent hashes.
    fn hash_key(&self, text: &[u8], off: usize, len: usize) -> usize {
        if self.ignore_case {
            let mut code = 0;
            for i in off..off + len {
                let b = text[i];
                let nb = if b >= ASCII_A_MIN && b <= ASCII_Z_MAX {
                    b + A_TO_Z_LOWER_OFFSET
                } else {
                    b
                };
                code = code * 31 + nb as usize;
            }
            code
        } else {
            let mut code = 0;
            for i in 0..len {
                let b = text[off + i];
                code = code * 31 + b as usize;
            }
            code
        }
    }

    /// Finds the slot for key text starting at `off` with length `len` by probing.
    /// Probe increment is `((hash>>8)+hash)|1`, always odd. Odd increments minimize clustering
    /// in hash tables and keep average probe lengths small. `|1` ensures the increment is odd.
    fn find_slot(&self, key_bytes: &[u8], off: usize, len: usize) -> usize {
        let mut code = self.hash_key(key_bytes, off, len);
        let mask = self.keys.len().wrapping_sub(1);
        let mut pos = code & mask;

        loop {
            if self.keys[pos] == None {
                return pos;
            }
            let key2 = self.keys[pos].as_ref().unwrap();
            if self.equals_key(key_bytes, off, len, key2) {
                return pos;
            }
            let inc = ((code >> 8) + code) | 1; // always odd to minimize clustering
            code = code + inc;
            pos = code & mask;
        }
    }

    /// Checks if a key exists in the map.
    ///
    /// @param key the key bytes [u8]
    /// @param off the offset
    /// @param len the length
    /// @return {@code true} if key exists
    pub fn contains_key(&self, key: &[u8], off: usize, len: usize) -> bool {
        let slot = self.find_slot(key, off, len);
        self.keys[slot] != None
    }

    /// Convenience checks if `key` (a &[u8]) is in the map, case-insensitively if the map is case-insensitive.
    /// Wraps `contains_key` that accepts &[u8].
    /// @param key the key bytes [u8]
    /// @return {@code true} if the map contains `key`, {@code false} otherwise
    pub fn contains_key_ca(&self, key: &[u8]) -> bool {
        self.contains_key(key, 0, key.len())
    }

    /// Inserts or replaces a key-value pair. Returns `None` if the key was new (slot was empty),
    /// `Some(old)` if it would replace an existing placeholder (i.e., "add" would overwrite).
    ///
    /// @param key the key bytes [u8]
    /// @param value the value to store (e.g., `()`` for sets)
    /// @return {@code None} if new, {@code Some(old)} if replaced
    pub fn put(&mut self, key: &[u8], value: V) -> Option<V> {
        let norm_key = self.normalize_key(key);
        let slot = self.find_slot(&norm_key, 0, norm_key.len());

        if self.keys[slot] == None {
            // New key: insert
            self.keys[slot] = Some(norm_key);
            self.values[slot] = value;
            self.count += 1;
            None
        } else {
            // Existing key: replace
            let old = self.values[slot].clone();
            self.values[slot] = value;
            Some(old)
        }
    }

    /// Convenience put that accepts `&[u8]` for the key.
    /// @param key the key bytes [u8]
    /// @param value the value to store
    /// @return {@code None} if new, {@code Some(old)} if replaced
    pub fn put_ca(&mut self, key: &[u8], value: V) -> Option<V> {
        self.put(key, value)
    }

    /// Clears all occupied slots. Sets all to `None` and resets `count`.
    ///
    /// @return {@code this} for chaining
    pub fn clear(&mut self) -> &mut Self {
        self.keys.iter_mut().for_each(|opt| *opt = None);
        self.count = 0;
        self
    }

    /// Returns the number of occupied slots (keys present in the map).
    ///
    /// @return the size (number of occupied slots)
    pub fn size(&self) -> usize {
        self.count
    }

    /// Returns keys as strings.
    ///
    /// @return the key vector
    pub fn key_vec(&self) -> Vec<String> {
        self.keys
            .iter()
            .filter(|o| o.is_some())
            .map(|o| {
                let key: &Vec<u8> = o.as_ref().unwrap();
                String::from_utf8(key.to_vec()).unwrap()
            })
            .collect()
    }

    /// Copies this map to a new {@link CharArrayMap}.
    ///
    /// @return a new {@link CharArrayMap}
    pub fn copy(&self) -> Self {
        let mut new_map = Self::new(self.keys.len() as usize, self.ignore_case);
        for i in 0..self.keys.len() {
            if let Some(ref key) = self.keys[i] {
                new_map.put(&key, self.values[i].clone());
            }
        }
        new_map
    }

    /// Creates a new map with given start size and ignore_case flag.
    /// Starts with empty vectors (capacity grows as needed).
    pub fn new(start_size: usize, ignore_case: bool) -> Self {
        let capacity = Self::min_capacity(start_size);
        let mut keys = Vec::with_capacity(capacity);
        let mut values = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            keys.push(None);
            values.push(V::default());
        }
        Self {
            keys,
            values,
            count: 0,
            ignore_case,
        }
    }
}

// Helper to create an empty map; no constant EMPTY_MAP
pub fn new_empty_map<V: Default + Clone>() -> CharArrayMap<V> {
    CharArrayMap::new(8, false)
}
