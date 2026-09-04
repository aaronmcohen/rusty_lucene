//! Lucene core crate
//! Simple placeholder that compiles

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
