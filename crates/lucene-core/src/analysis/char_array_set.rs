pub struct CharArraySet {
    values: Vec<String>,
    case_insensitive: bool,
}

#[allow(dead_code)]
impl CharArraySet {
    pub fn new(_size: usize, case_insensitive: bool) -> Self {
        Self {
            values: Vec::new(),
            case_insensitive,
        }
    }

    pub fn add_str(&mut self, val: &str) {
        if self.case_insensitive {
            let lc = val.to_lowercase().to_owned();
            self.values.push(lc);
        } else {
            self.values.push(val.to_owned());
        }
    }

    pub fn contains_cs(&self, val: &str) -> bool {
        if self.case_insensitive {
            let val_lc = val.to_lowercase().to_owned();
            self.values.iter().any(|s| {
                let s_lc = s.to_lowercase().to_owned();
                // compare String with String
                s_lc == val_lc
            })
        } else {
            self.values.iter().any(|s| s.eq(val))
        }
    }

    pub fn size(&self) -> usize {
        self.values.len()
    }
}
