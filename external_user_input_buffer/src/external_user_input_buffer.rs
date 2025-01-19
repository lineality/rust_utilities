//! externalized_input_buffer.rs
//! Non-blocking keyboard input handler for TUI applications
//! Provides immediate character-by-character access without streams
//! Characters are stored and accessible before any "flush" operation

use std::fs::File;
use std::io::{self, Read, Write};
use std::fs;

/// Raw keyboard event structure for Linux input system
#[repr(C)]
struct InputEvent {
    tv_sec: i64,
    tv_usec: i64,
    type_: u16,
    code: u16,
    value: i32,
}

/// Non-blocking keyboard input handler for TUI applications
/// Stores characters both in memory and file for immediate access
/// Characters are available before any flush operation
#[derive(Debug)]
pub struct ExternalizedInputBuffer {
    /// Current characters in memory buffer
    chars: Vec<char>,
    /// Path to persistent character storage
    temp_file_path: String,
    /// Raw keyboard device file
    keyboard_device: File,
}

impl ExternalizedInputBuffer {
    /// Creates new keyboard input handler with empty buffers
    /// Opens raw keyboard device for immediate character access
    pub fn new() -> io::Result<Self> {
        let temp_file_path = String::from("temp_input_buffer.txt");
        fs::write(&temp_file_path, "")?;
        
        // Open keyboard device in non-blocking mode
        let keyboard_device = File::open("/dev/input/event0")?;
        // Set non-blocking
        use std::os::unix::io::AsRawFd;
        unsafe {
            let fd = keyboard_device.as_raw_fd();
            libc::fcntl(fd, libc::F_SETFL, libc::O_NONBLOCK);
        }

        Ok(Self {
            chars: Vec::new(),
            temp_file_path,
            keyboard_device,
        })
    }

    /// Gets a character from keyboard without blocking
    /// Returns None if no character is available
    pub fn get_char(&mut self) -> io::Result<Option<char>> {
        let mut event = InputEvent {
            tv_sec: 0,
            tv_usec: 0,
            type_: 0,
            code: 0,
            value: 0,
        };

        // Read raw event
        match self.keyboard_device.read(unsafe {
            std::slice::from_raw_parts_mut(
                &mut event as *mut InputEvent as *mut u8,
                std::mem::size_of::<InputEvent>(),
            )
        }) {
            Ok(_) => {
                // Key press event
                if event.type_ == 1 && event.value == 1 {
                    // Convert keycode to char (simplified mapping)
                    let c = match event.code {
                        16..=25 => ((event.code - 16) as u8 + b'q') as char,
                        30..=38 => ((event.code - 30) as u8 + b'a') as char,
                        44..=50 => ((event.code - 44) as u8 + b'z') as char,
                        28 => '\n',  // Enter key
                        57 => ' ',   // Space key
                        _ => return Ok(None),
                    };
                    self.add_char(c)?;
                    Ok(Some(c))
                } else {
                    Ok(None)
                }
            },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Adds character to both memory and file storage
    pub fn add_char(&mut self, c: char) -> io::Result<()> {
        // Add to memory buffer
        self.chars.push(c);

        // Append to temp file
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.temp_file_path)?;
        
        write!(file, "{}", c)?;
        file.flush()?;
        
        Ok(())
    }

    /// Returns current buffer contents without clearing
    pub fn get_current_content(&self) -> String {
        self.chars.iter().collect()
    }

    /// Flushes buffer and returns contents
    /// Clears both memory and file storage
    pub fn flush(&mut self) -> io::Result<String> {
        let content: String = self.chars.iter().collect();
        self.chars.clear();
        fs::write(&self.temp_file_path, "")?;
        Ok(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_creation() -> io::Result<()> {
        let buffer = ExternalizedInputBuffer::new()?;
        assert!(fs::read_to_string(&buffer.temp_file_path)?.is_empty());
        Ok(())
    }
}