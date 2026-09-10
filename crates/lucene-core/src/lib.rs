//! Lucene core crate
//! Simple placeholder that compiles

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
