// A simple helper struct to write into a byte slice buffer
struct BufferWriter<'a> {
    buffer: &'a mut [u8],
    cursor: usize,
}

impl<'a> BufferWriter<'a> {
    fn new(buffer: &'a mut [u8]) -> Self {
        BufferWriter { buffer, cursor: 0 }
    }

    // Get the portion of the buffer that has been written to
    fn as_str(&self) -> Option<&str> {
        core::str::from_utf8(&self.buffer[..self.cursor]).ok()
    }

    // Get the raw bytes written and their length
    fn as_bytes(&self) -> &[u8] {
        &self.buffer[..self.cursor]
    }
}

// Implement the core::fmt::Write trait for our buffer writer
impl<'a> core::fmt::Write for BufferWriter<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        let remaining_buf = &mut self.buffer[self.cursor..];

        // Check if it fits
        if remaining_buf.len() < bytes.len() {
            // Optional: Write as much as fits, or just return Err
            // let available_len = remaining_buf.len();
            // remaining_buf[..available_len].copy_from_slice(&bytes[..available_len]);
            // self.cursor += available_len;
            Err(core::fmt::Error) // Indicate out of space
        } else {
            remaining_buf[..bytes.len()].copy_from_slice(bytes);
            self.cursor += bytes.len();
            Ok(())
        }
    }
}
