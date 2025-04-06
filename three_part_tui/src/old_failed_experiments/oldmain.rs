// src/main.rs
mod externalized_input_buffer;  // declare module in main
mod three_part_tui;            // declare module in main
use crate::three_part_tui::ThreePartTui;

/*
# Integration Guide: ExternalizedInputBuffer into ThreePartTUI

three_part_tui/src$ tree
.
├── externalized_input_buffer.rs
├── main.rs
└── three_part_tui.rs



## Background & Purpose
The ThreePartTUI and ExternalizedInputBuffer are designed to solve a specific problem: displaying multiple updating sections on a terminal without them interfering with each other. 

### Current Components:
1. **ThreePartTUI**: Uses temp files to maintain three separate display areas:
   - File listing (top)
   - Info bar (middle)
   - Input area (bottom)
   Each section reads from its own temp file, allowing independent updates.

2. **ExternalizedInputBuffer**: Manages character-by-character input while writing state to a file:
   - Accumulates characters until Enter
   - Writes current state to a temp file
   - Handles backspace and basic input control
   - Provides access to buffer state via `get_buffer()`

## Why This Design?
The key challenge was separating display updates from input handling. The solution uses temp files as "display buffers" because:
1. Each part can update independently
2. Changes are detected by checking file lengths
3. No need for complex terminal cursor management
4. Input can be monitored without interfering with display

## Integration Steps

### 1. File Structure
```rust
src/
  lib.rs               // Main TUI library
  input_buffer.rs      // ExternalizedInputBuffer module
  three_part_tui.rs    // ThreePartTUI implementation
```

### 2. Key Integration Points
- The input_buffer_path in ThreePartTUI should be passed to ExternalizedInputBuffer
- The main loop needs to:
  ```rust
  loop {
      // Handle input without blocking display
      input_buffer.handle_char()?;
      
      // Check all files for changes
      if self.needs_refresh()? {
          self.display_all()?;
      }
      
      thread::sleep(Duration::from_millis(50));
  }
  ```

### 3. Known Issues & Solutions
1. **Display Refresh**: Only refresh when files change (already implemented in needs_refresh())
2. **Input Handling**: 
   - Don't clear screen while typing
   - Use ExternalizedInputBuffer's file for input display
3. **File Management**:
   - Create temp directory if it doesn't exist
   - Clean up temp files on exit

## Testing Strategy
1. Test input accumulation:
   - Type characters, verify they appear
   - Press Enter, verify line completion
   - Check buffer clears properly

2. Test display updates:
   - Verify file listing updates
   - Verify info bar updates
   - Verify input shows while typing

3. Test integration:
   - All three sections visible
   - No interference between sections
   - Clean exit and cleanup

## Future Considerations
1. **Error Handling**: 
   - File system errors
   - Input buffer overflow
   - Terminal resize

2. **Performance**:
   - File change detection optimization
   - Refresh rate tuning
   - Buffer size limits

3. **Features to Consider**:
   - Configurable update rates
   - Custom input handlers
   - Section resize commands

## Code Example for Integration
```rust
// Example structure after integration
pub struct ThreePartTui {
    temp_dir: PathBuf,
    file_view_path: PathBuf,
    info_bar_path: PathBuf,
    input_buffer: ExternalizedInputBuffer,
    last_file_view_len: u64,
    last_info_bar_len: u64,
}

impl ThreePartTui {
    pub fn new() -> io::Result<Self> {
        let temp_dir = PathBuf::from("tui_temp");
        fs::create_dir_all(&temp_dir)?;
        
        let input_buffer = ExternalizedInputBuffer::new(
            temp_dir.join("input.txt"),
            true  // show cursor
        )?;
        
        // ... rest of initialization
    }
    
    pub fn run(&mut self) -> io::Result<()> {
        // File view update thread stays the same
        
        loop {
            // Handle input
            self.input_buffer.handle_char()?;
            
            // Update display if needed
            if self.needs_refresh()? {
                self.display_all()?;
            }
            
            thread::sleep(Duration::from_millis(50));
        }
    }
}
```

## Testing Commands
```bash
# Run basic test
cargo run

# Run with debug output
RUST_BACKTRACE=1 cargo run

# Clean temp files
rm -rf tui_temp/
```

## Next Steps
1. Implement the integration as shown above
2. Add error handling
3. Test all components together
4. Add any needed cleanup code
5. Document any new issues found

*/
fn main() -> std::io::Result<()> {
    let mut tui = ThreePartTui::new()?;
    tui.run()
}
