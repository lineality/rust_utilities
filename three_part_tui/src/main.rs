/*
# Modal TUI Implementation Handover Document

## Overview

This document explains the implementation of a modal text user interface (TUI) system developed as a proof of concept for balancing terminal refreshes with uninterrupted user input. The system addresses the fundamental conflict between displaying up-to-date content and allowing users to type without interruption.

## Core Functionality

The application implements a Vim-inspired modal interface with two primary modes:

1. **Refresh Mode**: Terminal updates automatically, but user input may be interrupted
2. **Insert Mode**: Terminal does not refresh, allowing uninterrupted typing

Users can toggle between these modes by pressing Enter with an empty input buffer.

## Technical Architecture

### Component Structure

The system consists of:

1. **Main Thread**: Coordinates application state and handles rendering
2. **Input Thread**: Continuously polls for user input
3. **Refresh Thread**: Monitors directory for changes
4. **Message Passing System**: Facilitates thread communication

### Key Data Types

- `App`: Maintains application state (mode, files, input buffer)
- `Mode`: Enum representing the current interface mode (Refresh/Insert)
- `Message`: Enum for inter-thread communication

### Thread Communication

The implementation uses channels (`mpsc`) to enable non-blocking communication between threads:

```
Input Thread ──┐
               │
               ▼
Refresh Thread ─── Channel ─── Main Thread
```

## Implementation Details

### Mode Management

The toggle mechanism uses a state pattern to manage mode transitions:

```rust
fn toggle_mode(&mut self) -> Mode {
    let previous_mode = self.mode;
    
    self.mode = match self.mode {
        Mode::Refresh => Mode::Insert,
        Mode::Insert => Mode::Refresh,
    };
    
    previous_mode
}
```

When switching from Insert to Refresh mode, we perform an immediate refresh to show any changes that occurred during Insert mode.

### Directory Monitoring

Directory changes are detected efficiently using a hash-based approach:

1. Calculate a hash of directory metadata (filenames, sizes, modification times)
2. Compare with previous hash to detect changes
3. Only trigger refresh when changes are detected

```rust
fn calculate_directory_hash(dir: &str) -> io::Result<u64> {
    let mut hasher = DefaultHasher::new();
    // Hash relevant file metadata...
    Ok(hasher.finish())
}
```

### Input Buffer Management

The input buffer is managed differently depending on the current mode:

- **Refresh Mode**: Buffer can be cleared by automatic refreshes
- **Insert Mode**: Buffer is preserved regardless of directory changes

```rust
if app.mode == Mode::Refresh {
    if app.update_directory_list()? {
        app.input_buffer.clear(); // Discard input in refresh mode
        force_refresh = true;
    }
}
```

### Rendering Strategy

The display is updated only when needed:

1. When the user types or deletes characters
2. When the mode changes
3. When directory contents change (in Refresh mode only)

This prevents unnecessary screen flicker and reduces CPU usage.

## User Experience Flow

### Normal Operation

1. Application starts in Refresh mode
2. Directory contents displayed and automatically updated
3. User presses Enter with empty input to enter Insert mode
4. User types without interruption
5. User presses Enter with empty input to return to Refresh mode
6. Application immediately refreshes to show any changes

### Input Processing

1. User types characters → Added to input buffer
2. User presses Backspace → Removes last character from buffer
3. User presses Enter with non-empty buffer → Processes command
4. User presses Enter with empty buffer → Toggles mode

## Implementation Challenges

### Problem: Input Buffer Conflicts

**Challenge**: Terminal refresh operations disrupt the input buffer.

**Solution**: Separate threading model with modes to control when refreshes occur.

### Problem: Timely Updates

**Challenge**: Balancing update frequency with performance.

**Solution**: Hash-based change detection with a throttled refresh rate (500ms).

### Problem: Mode Transition Consistency

**Challenge**: Ensuring display is up-to-date when switching modes.

**Solution**: Force directory refresh when switching from Insert to Refresh mode.

## Integration Guidelines

To integrate this functionality into the existing codebase:

1. **Extract Core Modal Logic**: 
   - Move the mode enumeration and toggle functions
   - Maintain mode state in your application

2. **Implement Buffer Management**:
   - Clear input buffer during refresh in Refresh mode
   - Preserve buffer in Insert mode

3. **Add Mode Indicator**:
   - Display current mode in the info bar
   - Provide visual feedback on mode changes

4. **Thread Management**:
   - Adapt the threading model to your application architecture
   - Consider using a similar message-passing approach

5. **Refresh Triggers**:
   - Add logic to force refresh when entering Refresh mode
   - Only perform directory refresh in Refresh mode

## Performance Considerations

1. **Hash Calculation**: Keep the directory hash calculation efficient
2. **Thread Sleep**: Adjust sleep durations based on application needs
3. **Render Frequency**: Only render when state actually changes

## Future Improvement Possibilities

1. **Terminal Capabilities**: Better handling of terminal capabilities (resize, color)
2. **Input Handling**: Support for arrow keys, tab completion
3. **Configuration**: Make refresh rate and other parameters configurable
4. **State Persistence**: Save/restore mode between sessions
5. **Visual Enhancements**: More pronounced mode indicators
*/

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
