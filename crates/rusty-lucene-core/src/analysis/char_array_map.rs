use crate::analysis::char_array_set::CharArraySet;
use crate::analysis::{ASCII_A_MIN, ASCII_Z_MAX, A_TO_Z_LOWER_OFFSET};
use std::collections::HashMap;

#[allow(dead_code)]
pub struct CharArrayMap {
    entries: Vec<(Vec<u8>, String)>,
    ignore_case: bool,
}

#[allow(dead_code)]
pub(crate) struct UnmodMap<'a> {
    delegate: &'a CharArrayMap,
}

impl<'a> UnmodMap<'a> {
    #[allow(dead_code)]
    pub fn new(delegate: &'a CharArrayMap) -> Self {
        Self { delegate }
    }

    #[allow(dead_code)]
    pub fn put(&self, _key: &[u8], _value: &str) -> Result<(), &'static str> {
        Err("UnsupportedOperationException")
    }

    #[allow(dead_code)]
    pub fn clear(&self) -> Result<(), &'static str> {
        Err("UnsupportedOperationException")
    }

    #[allow(dead_code)]
    pub fn key_set(&self) -> CharArraySet {
        self.delegate.key_set()
    }
}

#[allow(dead_code)]
impl CharArrayMap {
    fn cmp(&self, key1: &[u8], off1: usize, len1: usize, key2: &Vec<u8>) -> bool {
        if self.ignore_case {
            for i in 0..len1 {
                let b1 = key1[off1 + i];
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
            for i in 0..len1 {
                if key1[off1 + i] != key2[i] {
                    return false;
                }
            }
            return true;
        }
    }

    pub fn new(size: usize, ignore_case: bool) -> Self {
        Self {
            entries: Vec::with_capacity(size),
            ignore_case,
        }
    }

    pub fn put_all(&mut self, map: &HashMap<&str, &str>) {
        for (&key_str, value) in map.iter() {
            self.put_ca(&key_str.as_bytes(), value);
        }
    }

    pub fn put(&mut self, key: &[u8], value: &str) {
        for (entry_key, _entry_value) in &self.entries {
            if self.cmp(key, 0, key.len(), &entry_key) {
                // overwrite existing entry's value
                let idx = self
                    .entries
                    .iter()
                    .position(|(k, _)| k == entry_key)
                    .unwrap();
                self.entries[idx].1 = value.to_string();
                return;
            }
        }
        let stored_key = if self.ignore_case {
            let mut res = vec![0; key.len()];
            for i in 0..key.len() {
                let b = key[i];
                let nb = if b >= ASCII_A_MIN && b <= ASCII_Z_MAX {
                    b + A_TO_Z_LOWER_OFFSET
                } else {
                    b
                };
                res[i] = nb;
            }
            res
        } else {
            key.to_vec()
        };
        self.entries.push((stored_key, value.to_string()));
    }

    pub fn put_ca(&mut self, key: &[u8], value: &str) {
        let stored_key = if self.ignore_case {
            let mut res = vec![0; key.len()];
            for i in 0..key.len() {
                let b = key[i];
                let nb = if b >= ASCII_A_MIN && b <= ASCII_Z_MAX {
                    b + A_TO_Z_LOWER_OFFSET
                } else {
                    b
                };
                res[i] = nb;
            }
            res
        } else {
            key.to_vec()
        };
        self.entries.push((stored_key, value.to_string()));
    }

    pub fn get(&self, key: &[u8], off: usize, len: usize) -> Option<&String> {
        for (entry_key, entry_value) in &self.entries {
            if self.cmp(key, off, len, &entry_key) {
                return Some(&entry_value);
            }
        }
        None
    }

    #[allow(dead_code)]
    pub fn get_value(&self, key: &[u8], off: usize, len: usize) -> Option<&String> {
        self.get(key, off, len)
    }

    pub fn contains_key(&self, key: &[u8], off: usize, len: usize) -> bool {
        self.get(key, off, len).is_some()
    }

    #[allow(dead_code)]
    pub fn contains_key_ca(&self, key: &[u8]) -> bool {
        for (entry_key, _) in &self.entries {
            if self.cmp(key, 0, key.len(), entry_key) {
                return true;
            }
        }
        false
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.entries.len() == 0
    }

    #[allow(dead_code)]
    pub fn unmodifiable_map(&self) -> UnmodMap<'_> {
        UnmodMap::new(self)
    }

    pub fn copy(&self) -> Self {
        let mut new_map = Self::new(self.entries.len() as usize, self.ignore_case);
        for (entry_key, entry_value_str) in &self.entries {
            new_map.put_ca(&entry_key, &entry_value_str);
        }
        new_map
    }

    pub fn key_set(&self) -> CharArraySet {
        let mut cs = CharArraySet::new(self.entries.len(), self.ignore_case);
        for (entry_key, _) in &self.entries {
            let key_str = String::from_utf8(entry_key.clone()).unwrap_or_default();
            cs.add_str(&key_str);
        }
        cs
    }
}

impl Clone for CharArrayMap {
    fn clone(&self) -> Self {
        Self {
            entries: self.entries.clone(),
            ignore_case: self.ignore_case,
        }
    }
}
