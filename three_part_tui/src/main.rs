use std::env;
use std::fs;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::io::{self, Write, Read};
use std::thread;
use std::time::{
Duration,
//Instant,
};
use std::sync::mpsc::{self, Sender, Receiver, TryRecvError};

/// Application mode - either refresh terminal or accept input
#[derive(PartialEq, Clone, Copy, Debug)]
enum Mode {
    Refresh, // Allow refreshes, input may be interrupted
    Insert,  // No refreshes, focus on input
}

/// Holds application state
struct App {
    mode: Mode,
    input_buffer: String,
    files: Vec<String>,
    last_hash: u64,
    //last_refresh: Instant,
    terminal_width: u16,
    terminal_height: u16,
}

impl App {
    fn new() -> io::Result<Self> {
        // Default terminal size if we can't detect it
        let (width, height) = (80, 24);
        
        Ok(Self {
            mode: Mode::Refresh,
            input_buffer: String::new(),
            files: scan_directory(".")?,
            last_hash: calculate_directory_hash(".")?,
            //last_refresh: Instant::now(),
            terminal_width: width,
            terminal_height: height,
        })
    }

    /// Toggle between Refresh and Insert modes
    fn toggle_mode(&mut self) -> Mode {
        let previous_mode = self.mode;
        
        self.mode = match self.mode {
            Mode::Refresh => Mode::Insert,
            Mode::Insert => Mode::Refresh,
        };
        
        previous_mode
    }

    /// Check for changes in directory and update file list if needed
    /// Returns true if directory changed
    fn update_directory_list(&mut self) -> io::Result<bool> {
        let current_hash = calculate_directory_hash(".")?;
        
        if current_hash != self.last_hash {
            self.files = scan_directory(".")?;
            self.last_hash = current_hash;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Force update directory list regardless of hash changes
    fn force_update_directory_list(&mut self) -> io::Result<()> {
        self.files = scan_directory(".")?;
        self.last_hash = calculate_directory_hash(".")?;
        Ok(())
    }

    /// Render the current application state to the terminal
    fn render(&self) -> io::Result<()> {
        // Clear screen
        print!("\x1B[2J\x1B[1;1H");
        
        // 1. Display directory files
        let path = env::current_dir()?;
        println!("Current Path: {}", path.display());
        println!();
        
        for (i, item) in self.files.iter().enumerate() {
            println!("{}. {}", i + 1, item);
        }
        
        // Fill the rest of the screen with empty lines to ensure consistent layout
        let path_length = 2;  // Path header + empty line
        let file_count = self.files.len();
        let info_bar_position = (self.terminal_height - 2) as usize;
        
        for _ in 0..info_bar_position.saturating_sub(path_length + file_count) {
            println!();
        }
        
        // 2. Display info bar with mode
        match self.mode {
            Mode::Refresh => println!("\\|/  Refresh Mode - 'enter' to toggle insert-mode"),
            Mode::Insert => println!(">_  Insert Mode - 'enter' to toggle refresh-mode"),
        }
        
        // 3. Display user prompt
        print!("> {}", self.input_buffer);
        io::stdout().flush()
    }
}

/// Scan current directory and return list of files
fn scan_directory(dir: &str) -> io::Result<Vec<String>> {
    let mut files = Vec::new();
    
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let file_name = entry.file_name().to_string_lossy().into_owned();
        files.push(file_name);
    }
    
    files.sort();
    Ok(files)
}

/// Calculate hash of directory contents to detect changes
fn calculate_directory_hash(dir: &str) -> io::Result<u64> {
    let mut hasher = DefaultHasher::new();
    let entries = fs::read_dir(dir)?;
    
    for entry in entries {
        let entry = entry?;
        let metadata = entry.metadata()?;
        
        // Hash relevant file metadata
        entry.file_name().hash(&mut hasher);
        metadata.len().hash(&mut hasher);
        if let Ok(modified) = metadata.modified() {
            modified.hash(&mut hasher);
        }
    }
    
    Ok(hasher.finish())
}

/// Message types for communication between threads
enum Message {
    Input(char),
    Backspace,
    Enter,
    Refresh,
    Quit,
}

fn main() -> io::Result<()> {
    // Initialize app state
    let mut app = App::new()?;
    
    // Set up channel for communication between input thread and main thread
    let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();
    
    // Input thread - constantly reads from stdin
    let input_tx = tx.clone();
    thread::spawn(move || {
        let mut stdin = io::stdin();
        let mut buffer = [0; 1];
        
        loop {
            if stdin.read_exact(&mut buffer).is_ok() {
                match buffer[0] {
                    b'\n' | b'\r' => { 
                        input_tx.send(Message::Enter).unwrap_or(());
                    },
                    8 | 127 => { 
                        input_tx.send(Message::Backspace).unwrap_or(());
                    },
                    b'q' => {
                        input_tx.send(Message::Quit).unwrap_or(());
                    },
                    c => {
                        input_tx.send(Message::Input(c as char)).unwrap_or(());
                    }
                }
            }
            thread::sleep(Duration::from_millis(10));
        }
    });
    
    // Refresh thread - periodically checks directory for changes
    let refresh_tx = tx.clone();
    thread::spawn(move || {
        let mut last_hash = 0;
        
        loop {
            // Only calculate hash every 500ms to avoid excessive CPU usage
            thread::sleep(Duration::from_millis(500));
            
            if let Ok(current_hash) = calculate_directory_hash(".") {
                if current_hash != last_hash {
                    last_hash = current_hash;
                    refresh_tx.send(Message::Refresh).unwrap_or(());
                }
            }
        }
    });
    
    // Initial render
    app.render()?;
    
    // Main application loop
    let mut force_refresh = false;
    
    loop {
        // Process messages from threads
        match rx.try_recv() {
            Ok(Message::Input(c)) => {
                if app.mode == Mode::Insert || !force_refresh {
                    app.input_buffer.push(c);
                    force_refresh = true; // Need to update display with new input
                }
            },
            Ok(Message::Backspace) => {
                app.input_buffer.pop();
                force_refresh = true;
            },
            Ok(Message::Enter) => {
                if app.input_buffer.is_empty() {
                    // Toggle mode
                    let previous_mode = app.toggle_mode();
                    
                    // If switching from Insert to Refresh, immediately refresh directory
                    if previous_mode == Mode::Insert && app.mode == Mode::Refresh {
                        // Force an immediate directory update to show any changes that occurred
                        app.force_update_directory_list()?;
                    }
                    
                    force_refresh = true;
                } else {
                    // Process command
                    if app.input_buffer == "q" || app.input_buffer == "quit" {
                        break;
                    }
                    
                    // Here you would handle the input command
                    println!("\nYou typed: {}", app.input_buffer);
                    app.input_buffer.clear();
                    force_refresh = true;
                }
            },
            Ok(Message::Refresh) => {
                if app.mode == Mode::Refresh {
                    // If in refresh mode, directory changes should trigger refresh
                    // and discard any partial input
                    if app.update_directory_list()? {
                        app.input_buffer.clear(); // Discard input buffer in refresh mode
                        force_refresh = true;
                    }
                }
            },
            Ok(Message::Quit) => {
                break;
            },
            Err(TryRecvError::Empty) => {
                // No messages, continue
            },
            Err(TryRecvError::Disconnected) => {
                // Channel closed, should exit
                break;
            }
        }
        
        // Update display if needed
        if force_refresh {
            app.render()?;
            force_refresh = false;
        }
        
        // Small sleep to prevent tight loop
        thread::sleep(Duration::from_millis(10));
    }
    
    // Clear screen on exit
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush()?;
    
    Ok(())
}
