//! Three-part TUI module that displays file listings, info, and input
//! using temp files as display buffers.

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

/// Manages a three-part TUI display using temp files as buffers
pub struct ThreePartTui {
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

impl ThreePartTui {
    /// Creates new TUI instance and initializes temp files
    pub fn new() -> io::Result<Self> {
        let temp_dir = PathBuf::from("tui_temp");
        fs::create_dir_all(&temp_dir)?;

        let file_view_path = temp_dir.join("file_view.txt");
        let info_bar_path = temp_dir.join("info_bar.txt");
        let input_buffer_path = temp_dir.join("input_buffer.txt");

        // Initialize files with empty content
        File::create(&file_view_path)?;
        File::create(&info_bar_path)?;
        File::create(&input_buffer_path)?;

        Ok(ThreePartTui {
            temp_dir,
            file_view_path,
            info_bar_path,
            input_buffer_path,
            last_file_view_len: 0,
            last_info_bar_len: 0,
            last_input_buffer_len: 0,
        })
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

    /// Main run loop - updates file view and refreshes display
    pub fn run(&mut self) -> io::Result<()> {
        // Start file view update thread
        let file_view_path = self.file_view_path.clone();
        thread::spawn(move || {
            let mut last_content = String::new();
            loop {
                let mut current_content = String::new();
                let entries = fs::read_dir(".").unwrap();
                for entry in entries {
                    if let Ok(entry) = entry {
                        current_content.push_str(&entry.file_name().to_string_lossy());
                        current_content.push('\n');
                    }
                }
                
                // Only write if content changed
                if current_content != last_content {
                    File::create(&file_view_path)
                        .unwrap()
                        .write_all(current_content.as_bytes())
                        .unwrap();
                    last_content = current_content;
                }
                
                thread::sleep(Duration::from_secs(2));
            }
        });

        // Write initial info bar content
        fs::write(&self.info_bar_path, "Test Mode Active\n")?;

        // Main display loop
        loop {
            if self.needs_refresh()? {
                self.display_all()?;
            }
            thread::sleep(Duration::from_millis(100));
        }
    }
}