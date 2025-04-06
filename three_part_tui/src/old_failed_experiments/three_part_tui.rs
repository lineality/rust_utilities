//! Three-part TUI module that displays file listings, info, and input
//! using temp files as display buffers.
// src/three_part_tui.rs

// mod externalized_input_buffer;
use crate::externalized_input_buffer::ExternalizedInputBuffer;
// use externalized_input_buffer::ExternalizedInputBuffer;

// mod externalized_input_buffer;
// use crate::externalized_input_buffer::ExternalizedInputBuffer;

// use std::sync::atomic::{AtomicBool, Ordering};
// use std::sync::Arc;

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub enum TuiError {
    Io(io::Error),
    InputError(String),
    DisplayError(String),
}

// Implement conversions from io::Error
impl From<io::Error> for TuiError {
    fn from(err: io::Error) -> Self {
        TuiError::Io(err)
    }
}

/// Manages a three-part TUI display using temp files as buffers
pub struct ThreePartTui {
    /// Using module
    external_inputbuffer: ExternalizedInputBuffer,
    /// Directory containing all temp files
    temp_dir: PathBuf,
    /// Path to file containing directory listing
    file_view_path: PathBuf,
    /// Path to file containing status/info messages
    info_bar_path: PathBuf,
    /// Path to file containing input buffer
    input_buffer_path: PathBuf,
    /// Cached file sizes for change detection
    last_file_view_len: u64,
    last_info_bar_len: u64,
    last_input_buffer_len: u64,
}

impl Drop for ThreePartTui {
    fn drop(&mut self) {
        if let Err(e) = fs::remove_dir_all(&self.temp_dir) {
            eprintln!("Error cleaning up temp files: {}", e);
        }
    }
}

impl ThreePartTui {
    /// Creates new TUI instance and initializes temp files
    pub fn new() -> io::Result<Self> {
        let temp_dir = PathBuf::from("tui_temp");
        
        // Clean up any existing temp directory
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir)?;
        }
        
        // start anew
        fs::create_dir_all(&temp_dir)?;

        let file_view_path = temp_dir.join("file_view.txt");
        let info_bar_path = temp_dir.join("info_bar.txt");
        let input_buffer_path = temp_dir.join("input_buffer.txt");

        // Initialize files with empty content
        File::create(&file_view_path)?;
        File::create(&info_bar_path)?;
        File::create(&input_buffer_path)?;

        let external_inputbuffer = ExternalizedInputBuffer::new(
            input_buffer_path.clone(),
            true
        )?;

        Ok(ThreePartTui {
            external_inputbuffer,
            temp_dir,
            file_view_path,
            info_bar_path,
            input_buffer_path,
            last_file_view_len: 0,
            last_info_bar_len: 0,
            last_input_buffer_len: 0,
        })
    }
    // pub fn new() -> io::Result<Self> {
    //     let temp_dir = PathBuf::from("tui_temp");
    //     fs::create_dir_all(&temp_dir)?;
        
    //     let external_inputbuffer = ExternalizedInputBuffer::new(
    //         input_buffer_path.clone(),
    //         true
    //     )?;

    //     let file_view_path = temp_dir.join("file_view.txt");
    //     let info_bar_path = temp_dir.join("info_bar.txt");
    //     let input_buffer_path = temp_dir.join("input_buffer.txt");

    //     // Initialize files with empty content
    //     File::create(&file_view_path)?;
    //     File::create(&info_bar_path)?;
    //     File::create(&input_buffer_path)?;

    //     Ok(ThreePartTui {
    //         external_inputbuffer,
    //         temp_dir,
    //         file_view_path,
    //         info_bar_path,
    //         input_buffer_path,
    //         last_file_view_len: 0,
    //         last_info_bar_len: 0,
    //         last_input_buffer_len: 0,
    //     })
    // }

    /// Processes a completed input line and executes appropriate commands
    /// 
    /// # Arguments
    /// * `input_line` - The completed input string to process
    /// 
    /// # Returns
    /// * `io::Result<()>` - Success or IO error from file operations
    /// 
    /// # Command Documentation
    /// Currently supported commands:
    /// - "exit" or "quit": Safely exits the program
    /// - "clear": Clears the file view
    /// - "help": Displays available commands
    /// - Any other input: Treated as unrecognized command
    fn process_completed_input(&mut self, input_line: &str) -> io::Result<()> {
        // Trim whitespace and convert to lowercase for consistent matching
        let cleaned_input = input_line.trim().to_lowercase();
        
        // Log the received command to info bar
        self.update_info_bar_status(&format!("Processing command: {}", cleaned_input))?;

        match cleaned_input.as_str() {
            "exit" | "quit" => {
                self.update_info_bar_status("Exiting program...")?;
                // Allow time for message to be seen
                thread::sleep(Duration::from_millis(500));
                // Exit program safely
                std::process::exit(0);
            },
            
            "clear" => {
                // Clear the file view
                fs::write(&self.file_view_path, "")?;
                self.update_info_bar_status("Cleared file view")?;
            },
            
            "help" => {
                let help_text = self.generate_help_text();
                self.update_info_bar_status(&help_text)?;
            },
            
            "" => {
                // Empty input - just update status
                self.update_info_bar_status("Ready for input")?;
            },
            
            // Unrecognized command
            _ => {
                self.update_info_bar_status(
                    &format!("Unrecognized command: '{}'. Type 'help' for available commands.", 
                            cleaned_input)
                )?;
            }
        }
        
        Ok(())
    }

    /// Generates help text showing available commands
    /// 
    /// # Returns
    /// * `String` - Formatted help text
    fn generate_help_text(&self) -> String {
        [
            "Available Commands:",
            "- exit/quit : Exit the program",
            "- clear    : Clear the file view",
            "- help     : Show this help message",
            "",
            "Press Enter after typing command"
        ].join("\n")
    }
    
    /// Updates the file view temp file with current directory contents
    fn update_file_view(&self) -> io::Result<()> {
        let mut file = File::create(&self.file_view_path)?;
        let entries = fs::read_dir(".")?;
        
        for entry in entries {
            if let Ok(entry) = entry {
                writeln!(file, "{}", entry.file_name().to_string_lossy())?;
            }
        }
        Ok(())
    }

    /// Checks if any display files have changed
    fn needs_refresh(&mut self) -> io::Result<bool> {
        let file_len = fs::metadata(&self.file_view_path)?.len();
        let info_len = fs::metadata(&self.info_bar_path)?.len();
        let input_len = fs::metadata(&self.input_buffer_path)?.len();

        let needs_update = file_len != self.last_file_view_len 
            || info_len != self.last_info_bar_len
            || input_len != self.last_input_buffer_len;

        // Update cached lengths
        self.last_file_view_len = file_len;
        self.last_info_bar_len = info_len;
        self.last_input_buffer_len = input_len;

        Ok(needs_update)
    }

    /// Displays all three sections by reading from temp files
    fn display_all(&self) -> io::Result<()> {
        print!("\x1B[2J\x1B[1;1H");  // Clear screen
        
        println!("=== Files ===");
        let file_content = fs::read_to_string(&self.file_view_path)?;
        print!("{}", file_content);
        
        println!("\n=== Info ===");
        let info_content = fs::read_to_string(&self.info_bar_path)?;
        print!("{}", info_content);
        
        println!("\n=== Input ===");
        let input_content = fs::read_to_string(&self.input_buffer_path)?;
        print!("> {}", input_content);
        
        io::stdout().flush()?;
        Ok(())
    }

//     /// Main run loop - updates file view and refreshes display
//     pub fn run(&mut self) -> io::Result<()> {
            
//         // Start file view update thread
//         let file_view_path = self.file_view_path.clone();
//         thread::spawn(move || {
//             let mut last_content = String::new();
//             loop {
//                 let mut current_content = String::new();
//                 let entries = fs::read_dir(".").unwrap();
//                 for entry in entries {
//                     if let Ok(entry) = entry {
//                         current_content.push_str(&entry.file_name().to_string_lossy());
//                         current_content.push('\n');
//                     }
//                 }
                
//                 // Only write if content changed
//                 if current_content != last_content {
//                     File::create(&file_view_path)
//                         .unwrap()
//                         .write_all(current_content.as_bytes())
//                         .unwrap();
//                     last_content = current_content;
//                 }
                
//                 thread::sleep(Duration::from_secs(2));
//             }
//         });

//         // Write initial info bar content
//         fs::write(&self.info_bar_path, "Test Mode Active\n")?;

//         // Main display loop
//         loop {
//             if self.needs_refresh()? {
//                 self.display_all()?;
//             }
//             thread::sleep(Duration::from_millis(100));
//         }
//     }



    /// Main TUI loop that handles:
    /// 1. Input processing via ExternalizedInputBuffer
    /// 2. File view updates
    /// 3. Display refresh
    /// 
    /// Returns io::Result to propagate any IO errors that occur
    pub fn run(&mut self) -> io::Result<()> {
        // Start file view update thread
        let file_view_path = self.file_view_path.clone();
        thread::spawn(move || {
            let mut last_content = String::new();
            loop {
                let mut current_content = String::new();
                // Handle potential errors in directory reading
                match fs::read_dir(".") {
                    Ok(entries) => {
                        for entry in entries {
                            if let Ok(entry) = entry {
                                current_content.push_str(&entry.file_name().to_string_lossy());
                                current_content.push('\n');
                            }
                        }
                    },
                    Err(e) => {
                        current_content = format!("Error reading directory: {}", e);
                    }
                }
                
                // Only write if content changed
                if current_content != last_content {
                    if let Err(e) = File::create(&file_view_path)
                        .and_then(|mut f| f.write_all(current_content.as_bytes())) 
                    {
                        eprintln!("Error updating file view: {}", e);
                    }
                    last_content = current_content;
                }
                
                thread::sleep(Duration::from_secs(2));
            }
        });

        // Write initial info bar status
        self.update_info_bar_status("TUI Started - Ready for Input")?;

        // Main input and display loop
        loop {
            // First priority: Handle any pending input
            match self.external_inputbuffer.handle_char() {
                Ok(true) => {
                    // Enter was pressed - get and process the completed line
                    // Clone the buffer string to avoid borrow conflicts
                    let input_to_process = String::from(self.external_inputbuffer.get_buffer());
                    self.process_completed_input(&input_to_process)?;
                },
                Ok(false) => {
                    // No Enter press - continue normal operation
                },
                Err(e) => {
                    // Log input error to info bar but don't crash
                    self.update_info_bar_status(&format!("Input error: {}", e))?;
                }
            }

            // Second priority: Update display if needed
            if self.needs_refresh()? {
                if let Err(e) = self.display_all() {
                    self.update_info_bar_status(&format!("Display error: {}", e))?;
                }
            }

            // Prevent CPU spinning while still maintaining responsiveness
            thread::sleep(Duration::from_millis(50));
        }
    }


    /// Updates the info bar with a status message
    /// 
    /// # Arguments
    /// * `status_message` - The message to display in the info bar
    /// 
    /// # Returns
    /// * `io::Result<()>` - Success or IO error
    fn update_info_bar_status(&self, status_message: &str) -> io::Result<()> {
        fs::write(&self.info_bar_path, format!("{}\n", status_message))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_input_buffer() {
        // Add tests
    }
    
    #[test]
    fn test_display_refresh() {
        // Add tests
    }

    #[test]
    fn test_process_completed_input() -> io::Result<()> {
        let mut tui = ThreePartTui::new()?;
        
        // Test help command
        tui.process_completed_input("help")?;
        let info_content = fs::read_to_string(&tui.info_bar_path)?;
        assert!(info_content.contains("Available Commands"));
        
        // Test clear command
        tui.process_completed_input("clear")?;
        let file_view_content = fs::read_to_string(&tui.file_view_path)?;
        assert!(file_view_content.is_empty());
        
        // Test unrecognized command
        tui.process_completed_input("invalid_command")?;
        let info_content = fs::read_to_string(&tui.info_bar_path)?;
        assert!(info_content.contains("Unrecognized command"));
        
        Ok(())
    }
    
    
}
