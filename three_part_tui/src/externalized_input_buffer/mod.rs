// src/externalized_input_buffer.rs

use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;

/// Manages an input buffer that writes its state to a file for external reading
pub struct ExternalizedInputBuffer {
    /// Current content of input buffer
    buffer: String,
    /// Path to file where buffer content is written
    buffer_file_path: PathBuf,
    /// Whether to show cursor marker at end
    show_cursor: bool,
}

impl ExternalizedInputBuffer {
    pub fn new(buffer_file_path: PathBuf, show_cursor: bool) -> io::Result<Self> {
        // Ensure file exists and is empty
        fs::write(&buffer_file_path, "")?;
        
        Ok(ExternalizedInputBuffer {
            buffer: String::new(),
            buffer_file_path,
            show_cursor,
        })
    }

    /// Handles a single character of input
    /// Returns true if Enter was pressed
    pub fn handle_char(&mut self) -> io::Result<bool> {
        let mut char_buf = [0u8; 1];
        if io::stdin().read_exact(&mut char_buf).is_ok() {
            match char_buf[0] {
                // Enter key
                13 | 10 => {
                    let completed_line = self.buffer.clone();
                    self.buffer.clear();
                    self.write_to_file()?;
                    Ok(true)
                },
                // Backspace
                127 | 8 => {
                    self.buffer.pop();
                    self.write_to_file()?;
                    Ok(false)
                },
                // Regular character
                c if c.is_ascii_graphic() || c == b' ' => {
                    self.buffer.push(c as char);
                    self.write_to_file()?;
                    Ok(false)
                },
                _ => Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    /// Writes current buffer content to file
    fn write_to_file(&self) -> io::Result<()> {
        let mut content = self.buffer.clone();
        if self.show_cursor {
            content.push_str("[]");
        }
        fs::write(&self.buffer_file_path, content)
    }

    /// Gets current buffer content
    pub fn get_buffer(&self) -> &str {
        &self.buffer
    }

    /// Clears the buffer and file
    pub fn clear(&mut self) -> io::Result<()> {
        self.buffer.clear();
        self.write_to_file()
    }
}
