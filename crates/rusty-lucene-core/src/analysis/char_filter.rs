use rusty_lucene_common::reader::{Reader, StringReader};
use std::io;

/// A character filter that wraps a `Reader` and can optionally correct the offset.
///
/// <p>This class is abstract: at a minimum you must implement {@link #read(char[], int, int)},
/// transforming the input in some way from {@link #input}, and {@link #correct(int)} to adjust
/// the offsets to match the originals.
///
/// <p>You can optionally provide more efficient implementations of additional methods like
/// {@link #read()}, {@link #read(char[])}, {@link #read(java.nio.CharBuffer)}, but this is not required.
///
/// @param input a Reader, can also be a CharFilter for chaining.
///
/// <p>For use with {@link Analyzer}, see the {@link org.apache.lucene.analysis Analysis}
/// package documentation.
pub struct CharFilter {
    input: Box<dyn Reader>,
}

impl CharFilter {
    fn new<R: Reader + 'static>(input: R) -> Self {
        Self {
            input: Box::new(input),
        }
    }

    fn correct(&self, current_off: usize) -> usize {
        current_off
    }
    fn close(&self) -> io::Result<()> {
        self.input.close()
    }

    pub fn correct_offset(&self, current_off: usize) -> usize {
        let corrected = self.correct(current_off);
        self.input.correct_offset(corrected)
    }
}

impl Reader for CharFilter {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        self.input.read(buf)
    }

    fn available(&self) -> usize {
        self.input.available()
    }

    fn close(&self) -> io::Result<()> {
        self.close()
    }

    fn correct_offset(&self, current_off: usize) -> usize {
        let corrected = self.correct(current_off);
        self.input.correct_offset(corrected)
    }
}

/// A simple char filter that adds 1 to the offset. Mirrors Java's `CharFilter1`.
pub struct CharFilter1 {
    filter: CharFilter,
}

impl CharFilter1 {
    fn new<R: Reader + 'static>(input: R) -> Self {
        Self {
            filter: CharFilter::new(input),
        }
    }

    fn correct(&self, current_off: usize) -> usize {
        current_off + 1
    }
    fn close(&self) -> io::Result<()> {
        self.filter.close()
    }

    pub fn correct_offset(&self, current_off: usize) -> usize {
        let corrected = self.correct(current_off);
        self.filter.correct_offset(corrected)
    }
}

impl Reader for CharFilter1 {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        self.filter.input.read(buf)
    }

    fn available(&self) -> usize {
        self.filter.available()
    }

    fn close(&self) -> io::Result<()> {
        self.close()
    }

    fn correct_offset(&self, current_off: usize) -> usize {
        let corrected = self.correct(current_off);
        self.filter.correct_offset(corrected)
    }
}

/// A simple char filter that adds 2 to the offset. Mirrors Java's `CharFilter2`.
pub struct CharFilter2 {
    filter: CharFilter,
}

impl CharFilter2 {
    fn new<R: Reader + 'static>(input: R) -> Self {
        Self {
            filter: CharFilter::new(input),
        }
    }

    fn correct(&self, current_off: usize) -> usize {
        current_off + 2
    }
    fn close(&self) -> io::Result<()> {
        self.filter.close()
    }

    pub fn correct_offset(&self, current_off: usize) -> usize {
        let corrected = self.correct(current_off);
        self.filter.correct_offset(corrected)
    }
}

impl Reader for CharFilter2 {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        self.filter.input.read(buf)
    }

    fn available(&self) -> usize {
        self.filter.available()
    }

    fn close(&self) -> io::Result<()> {
        self.close()
    }

    fn correct_offset(&self, current_off: usize) -> usize {
        let corrected = self.correct(current_off);
        self.filter.correct_offset(corrected)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_filter1() {
        let input = StringReader::new(String::new());
        let cs = CharFilter1::new(input);
        assert_eq!(cs.correct_offset(0), 1);
    }

    #[test]
    fn test_char_filter2() {
        let input = StringReader::new(String::new());
        let cs = CharFilter2::new(input);
        assert_eq!(cs.correct_offset(0), 2);
    }

    #[test]
    fn test_char_filter12() {
        let input = StringReader::new(String::new());
        let inner = CharFilter1::new(input);
        let cs = CharFilter2::new(inner);
        assert_eq!(cs.correct_offset(0), 3);
    }

    #[test]
    fn test_char_filter11() {
        let input = StringReader::new(String::new());
        let inner = CharFilter1::new(input);
        let cs = CharFilter1::new(inner);
        assert_eq!(cs.correct_offset(0), 2);
    }

    #[test]
    fn test_correct_offset_chain_with_non_empty_data() {
        let input = StringReader::new("abc".to_string());
        let inner = CharFilter1::new(input);
        let mut outer = CharFilter2::new(inner);
        assert_eq!(outer.correct_offset(0), 3);
        // Check that read works correctly with chaining
        let mut buf = [0; 5];
        let n = outer.read(&mut buf);
        assert_eq!(n.unwrap(), 3); // reads "abc"
    }
}
