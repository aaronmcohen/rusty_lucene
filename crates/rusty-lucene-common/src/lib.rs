//! rusty-lucene-common crate
//!
//! This crate provides the Rust equivalents of Java's `java.io.Reader` and `java.io.IOException`.
//! The public API consists of the `Reader` trait and `StringReader` from the `reader` module,
//! all re-exported from this crate root.

pub mod reader;

pub use reader::Reader;
pub use reader::StringReader;

#[cfg(test)]
mod tests {
    use crate::reader::{Reader, StringReader};

    #[test]
    fn string_reader_read_works() {
        // Test that StringReader can read from "hello world" (11 bytes, not including '\n')
        let mut reader = StringReader::from("hello world");
        assert_eq!(reader.available(), 11);

        let mut buf = [0; 5];
        let n = reader.read(&mut buf);
        assert_eq!(n.unwrap(), 5);
        assert_eq!(reader.available(), 6);

        let mut buf2 = [0; 7];
        let n2 = reader.read(&mut buf2);
        assert_eq!(n2.unwrap(), 6);
        assert_eq!(reader.available(), 0);
    }

    #[test]
    fn string_reader_empty() {
        let mut reader = StringReader::from("");
        assert_eq!(reader.available(), 0);
        let n = reader.read(&mut [0]);
        assert_eq!(n.unwrap(), 0);
    }
}
