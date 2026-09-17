//! Lucene core crate
//! Simple placeholder that compiles

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_float};

mod analysis;

pub struct IndexableField {
    pub name: String,
    pub value: String,
    pub stored: bool,
    pub indexed: bool,
}

impl IndexableField {
    pub fn new(name: String, value: String, stored: bool, indexed: bool) -> Self {
        Self {
            name,
            value,
            stored,
            indexed,
        }
    }
}

pub struct Document {
    pub fields: Vec<IndexableField>,
}

impl Document {
    pub fn new() -> Self {
        Self { fields: vec![] }
    }

    pub fn add(&mut self, field: IndexableField) {
        self.fields.push(field);
    }
}

#[cfg(test)]
mod tests {
    use crate::analysis::char_array_map::CharArrayMap;
    use crate::analysis::char_array_set::CharArraySet;

    #[test]
    fn test_basic_case_insensitive() {
        let mut set = CharArraySet::new(8, true);
        set.add_str("HELLO");
        assert!(set.contains_cs("hello"));
        assert!(set.contains_cs("HELLO"));
        assert!(!set.contains_cs("World"));
        assert!(set.size() == 1);
    }

    #[test]
    fn test_add_and_contains_str() {
        let mut set = CharArraySet::new(8, true);
        set.add_str("test");
        assert!(set.contains_cs("Test"));
        assert!(set.contains_cs("test"));
        set.add_str("world");
        assert!(set.size() == 2);
    }

    #[test]
    fn test_normalize_key() {
        let mut map = CharArrayMap::new(8, true);
        map.put_ca(b"HELLO", "v1");
        assert!(map.contains_key_ca(b"hello"));
        assert!(map.contains_key_ca(b"HELLO"));
    }
}

// Native functions for JNI
#[no_mangle]
pub extern "C" fn search(query: *const c_char, len: c_int) -> c_int {
    // Simple implementation: return number of matching fields (mock)
    // query is a C string (null-terminated), len is length in bytes
    if query.is_null() { return 0; }
    let c_str = unsafe { CStr::from_ptr(query) };
    let query_str = c_str.to_string_lossy().to_string();
    // Very simple mock: always return 1 for non-empty query
    if !query_str.is_empty() { 1 } else { 0 }
}

#[no_mangle]
pub extern "C" fn rank(id: c_int, score: c_float) -> c_float {
    // Simple implementation: return normalized score (mock)
    score * 0.5
}

// JNI-incompatible type example (will be documented, not exported)
#[no_mangle]
pub extern "C" fn get_complex_enum(state: *const c_char) -> c_int {
    // This function uses a Rust enum with data payload which has no natural Java equivalent
    // It will be documented as JNI-incompatible in the Java wrapper README
    42
}
