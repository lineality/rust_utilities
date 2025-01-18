// // src/externalized_input_buffer/mod.rs

// use std::fs;
// use std::io::{self, Read, Write};
// use std::path::PathBuf;

// /// Manages an input buffer that writes its state to a file for external reading
// pub struct ExternalizedInputBuffer {
//     /// Current content of input buffer
//     buffer: String,
//     /// Path to file where buffer content is written
//     buffer_file_path: PathBuf,
//     /// Whether to show cursor marker at end
//     show_cursor: bool,
// }

// impl ExternalizedInputBuffer {
//     pub fn new(buffer_file_path: PathBuf, show_cursor: bool) -> io::Result<Self> {
//         // Ensure file exists and is empty
//         fs::write(&buffer_file_path, "")?;
        
//         Ok(ExternalizedInputBuffer {
//             buffer: String::new(),
//             buffer_file_path,
//             show_cursor,
//         })
//     }

//     // /// Handles a single character of input
//     // /// Returns true if Enter was pressed
//     // pub fn handle_char(&mut self) -> io::Result<bool> {
//     //     let mut char_buf = [0u8; 1];
//     //     if io::stdin().read_exact(&mut char_buf).is_ok() {
//     //         match char_buf[0] {
//     //             // Enter key
//     //             13 | 10 => {
//     //                 let completed_line = self.buffer.clone();
//     //                 self.buffer.clear();
//     //                 self.write_to_file()?;
//     //                 Ok(true)
//     //             },
//     //             // Backspace
//     //             127 | 8 => {
//     //                 self.buffer.pop();
//     //                 self.write_to_file()?;
//     //                 Ok(false)
//     //             },
//     //             // Regular character
//     //             c if c.is_ascii_graphic() || c == b' ' => {
//     //                 self.buffer.push(c as char);
//     //                 self.write_to_file()?;
//     //                 Ok(false)
//     //             },
//     //             _ => Ok(false)
//     //         }
//     //     } else {
//     //         Ok(false)
//     //     }
//     // }

//     pub fn handle_char(&mut self) -> io::Result<bool> {
//         let mut char_buf = [0u8; 1];
//         if io::stdin().read_exact(&mut char_buf).is_ok() {
//             match char_buf[0] {
//                 // Enter key
//                 13 | 10 => {
//                     // FIXED: Don't clear buffer until AFTER we've used it
//                     let completed_line = self.buffer.clone();
//                     println!("Line completed: {}", completed_line);  // Show what was entered
//                     self.buffer.clear();
//                     self.write_to_file()?;
//                     Ok(true)
//                 },
//                 // Rest stays the same
//                 127 | 8 => {
//                     self.buffer.pop();
//                     self.write_to_file()?;
//                     Ok(false)
//                 },
//                 c if c.is_ascii_graphic() || c == b' ' => {
//                     self.buffer.push(c as char);
//                     self.write_to_file()?;
//                     Ok(false)
//                 },
//                 _ => Ok(false)
//             }
//         } else {
//             Ok(false)
//         }
//     }
    
//     /// Writes current buffer content to file
//     fn write_to_file(&self) -> io::Result<()> {
//         let mut content = self.buffer.clone();
//         if self.show_cursor {
//             content.push_str("[]");
//         }
//         fs::write(&self.buffer_file_path, content)
//     }

//     /// Gets current buffer content
//     pub fn get_buffer(&self) -> &str {
//         &self.buffer
//     }

//     /// Clears the buffer and file
//     pub fn clear(&mut self) -> io::Result<()> {
//         self.buffer.clear();
//         self.write_to_file()
//     }
// }


// src/externalized_input_buffer/mod.rs

use std::fs;
use std::io::{self, Read};  // Removed unused Write
use std::path::PathBuf;

/// ExternalizedInputBuffer: Manages text input and writes state to a file
/// 
/// This allows other processes to monitor the input state by reading the file.
/// The buffer accumulates characters until Enter is pressed, then clears.
/// 
/// # Example
/// ```
/// let buffer = ExternalizedInputBuffer::new(path, true)?;
/// while buffer.handle_char()? {
///     // Check buffer_file for current input state
/// }
/// ```
pub struct ExternalizedInputBuffer {
    /// Current content of input buffer
    buffer: String,
    /// Path to file where buffer content is written
    buffer_file_path: PathBuf,
    /// Whether to show cursor marker at end of content
    show_cursor: bool,
}

impl ExternalizedInputBuffer {
    /// Creates new input buffer instance
    /// 
    /// # Arguments
    /// * `buffer_file_path` - Path where buffer content will be written
    /// * `show_cursor` - If true, adds "[]" at end of content
    /// 
    /// # Returns
    /// * `io::Result<Self>` - New buffer instance or IO error
    pub fn new(buffer_file_path: PathBuf, show_cursor: bool) -> io::Result<Self> {
        // Initialize empty file
        fs::write(&buffer_file_path, "")?;
        
        Ok(ExternalizedInputBuffer {
            buffer: String::new(),
            buffer_file_path,
            show_cursor,
        })
    }

    /// Handles a single character of input
    /// 
    /// # Returns
    /// * `io::Result<bool>` - true if Enter was pressed, false otherwise
    /// 
    /// # Behavior
    /// - Enter (13, 10): Completes line, clears buffer
    /// - Backspace (127, 8): Removes last character
    /// - ASCII printable or space: Adds to buffer
    pub fn handle_char(&mut self) -> io::Result<bool> {
        let mut char_buf = [0u8; 1];
        if io::stdin().read_exact(&mut char_buf).is_ok() {
            match char_buf[0] {
                // Enter key
                13 | 10 => {
                    println!("Line completed: {}", self.buffer);
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
    /// 
    /// Adds cursor markers if show_cursor is true
    fn write_to_file(&self) -> io::Result<()> {
        let mut content = self.buffer.clone();
        if self.show_cursor {
            content.push_str("[]");
        }
        fs::write(&self.buffer_file_path, content)
    }

    /// Returns current buffer content
    pub fn get_buffer(&self) -> &str {
        &self.buffer
    }
}