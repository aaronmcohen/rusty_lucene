use std::io;

/// A character reader that reads from an input source.
///
/// This replaces `java.io.Reader` in the Rust port.
///
/// <p>Readers provide a way to read character data. `Reader` is the basic interface
/// for character input streams. `StringReader` is a concrete implementation that
/// reads from a `String`.
pub trait Reader {
    /// Reads bytes from the reader into the buffer.
    ///
    /// @param buf the buffer to read into (0-based offset assumed; caller ensures bounds)
    /// @param off the start offset in buf (normally 0)
    /// @param len the maximum number of bytes to read
    /// @return the number of bytes actually read (0 if nothing left); -1 is not used
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error>;

    /// Returns the number of bytes still available to be read from the reader.
    ///
    /// @return the number of bytes still available (> 0 means more can be read)
    fn available(&self) -> usize;

    /// Closes the reader.
    ///
    /// <p><b>NOTE:</b> When closing a `Reader`, it is recommended to call
    /// `<code>super.close()</code>` if extending `Reader` or `CharFilter`.
    /// For in-memory readers like `StringReader`, closing has no effect.
    fn close(&self) -> io::Result<()>;

    /// Returns the corrected offset after applying the filter's offset correction.
    ///
    /// <p>This method is used by {@link CharFilter}'s {@link #correctOffset} to
    /// chain offset corrections through nested `CharFilter` instances.
    ///
    /// @param currentOff the current offset (0-based, e.g., position in the input)
    /// @return the corrected offset
    fn correct_offset(&self, current_off: usize) -> usize;
}

/// A simple in-memory string reader that implements `Reader` and `std::io::Read`.
///
/// <p>`StringReader` reads bytes from a `String` sequence. It is commonly used
/// for testing and small in-memory character inputs.
///
/// @param data the string data
#[derive(Debug)]
pub struct StringReader {
    data: String,
    pos: usize,
}

impl StringReader {
    /// Creates a new `StringReader` with the given string data.
    ///
    /// @param data the string data
    pub fn new(data: String) -> Self {
        Self { data, pos: 0 }
    }

    /// Creates a new `StringReader` from a string buffer.
    ///
    /// @param input a Reader, can also be a CharFilter for chaining.
    pub fn from(data: impl Into<String>) -> Self {
        Self::new(data.into())
    }
}

impl std::io::Read for StringReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        // Read up to `len` bytes from `data` starting at `pos`.
        let remaining = self.data.len() - self.pos;
        if remaining == 0 {
            return Ok(0);
        }
        let to_read = buf.len().min(remaining);
        let slice = &self.data.as_bytes()[self.pos..self.pos + to_read];
        buf[..to_read].copy_from_slice(slice);
        self.pos += to_read;
        Ok(to_read)
    }
}

impl Reader for StringReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        // Delegate to std::io::Read impl to avoid code duplication.
        let read_result = std::io::Read::read(self, buf);
        match read_result {
            Ok(n) => Ok(n),
            Err(e) => Err(e.into()),
        }
    }

    fn available(&self) -> usize {
        self.data.len() - self.pos
    }

    fn close(&self) -> io::Result<()> {
        // `StringReader` has no underlying resource to close; simply return success.
        Ok(())
    }

    fn correct_offset(&self, current_off: usize) -> usize {
        // `StringReader` does not perform any offset correction, so it returns
        // the current offset unchanged.
        current_off
    }
}
