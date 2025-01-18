// src/main.rs
// examples/test_buffer.rs
mod externalized_input_buffer;
use externalized_input_buffer::ExternalizedInputBuffer;

// examples/test_input_buffer.rs

// use std::path::PathBuf;
// use std::thread;
// use std::time::Duration;
// use std::fs;
// use std::io::Write;



/// test
/// Expected Behavior for this test:
/// 1. When you type "frogs":
///    - Each character should appear immediately in "Current buffer: " line
///    - The buffer should show: "Current buffer: frogs[]" (with [] as cursor)
/// 2. When you press Enter:
///    - You should see "Enter pressed! Buffer was: frogs"
///    - The buffer should clear
/// 3. When you press Backspace:
///    - The last character should be removed
///    - The display should update immediately
// fn main() -> std::io::Result<()> {
//     // Create temp directory for test
//     let temp_dir = PathBuf::from("input_buffer_test");
//     fs::create_dir_all(&temp_dir)?;
//     let buffer_file = temp_dir.join("input.txt");

//     // Create input buffer
//     let mut input_buffer = ExternalizedInputBuffer::new(buffer_file.clone(), true)?;

//     println!("Type something (content will be shown below):");
//     println!("Press Ctrl+C to exit\n");

//     // Display loop
//     loop {
//         // Handle any pending input
//         if input_buffer.handle_char()? {
//             println!("\nEnter pressed! Buffer was: {}", input_buffer.get_buffer());
//         }

//         // Display current buffer content from file
//         if let Ok(content) = fs::read_to_string(&buffer_file) {
//             print!("\r                                        \r"); // Clear line
//             print!("Current buffer: {}", content);
//             std::io::stdout().flush()?;
//         }

//         thread::sleep(Duration::from_millis(50));
//     }
// }

// src/main.rs




// fn main() -> std::io::Result<()> {
//     let temp_dir = std::path::PathBuf::from("input_buffer_test");
//     std::fs::create_dir_all(&temp_dir)?;
//     let buffer_file = temp_dir.join("input.txt");

//     let mut input_buffer = ExternalizedInputBuffer::new(buffer_file.clone(), true)?;

//     println!("Input Buffer Test:");
//     println!("- Type characters to see them appear");
//     println!("- Backspace works to delete");
//     println!("- Enter completes the line");
//     println!("- Ctrl+C to exit\n");

//     let mut last_content = String::new();

//     loop {
//         // Handle input
//         if input_buffer.handle_char()? {
//             println!("\nLine completed: {}", input_buffer.get_buffer());
//             println!("\nStart typing again...");
//         }

//         // Only update display if content changed
//         if let Ok(content) = std::fs::read_to_string(&buffer_file) {
//             if content != last_content {
//                 print!("\r\x1B[K"); // Clear entire line
//                 print!("Current input: {}", content);
//                 std::io::stdout().flush()?;
//                 last_content = content;
//             }
//         }

//         std::thread::sleep(std::time::Duration::from_millis(50));
//     }
// }

// src/main.rs
fn main() -> std::io::Result<()> {
    let temp_dir = std::path::PathBuf::from("input_buffer_test");
    std::fs::create_dir_all(&temp_dir)?;
    let buffer_file = temp_dir.join("input.txt");

    let mut input_buffer = ExternalizedInputBuffer::new(buffer_file, true)?;
    println!("Type (Enter to complete line, Ctrl+C to exit):\n");

    loop {
        println!("before Buffer input_buffer.handle_char()?; contained: {}", input_buffer.get_buffer());
        input_buffer.handle_char()?;
        println!("after Buffer input_buffer.handle_char()?; contained: {}\n", input_buffer.get_buffer());
        // println!("Buffer contained: {}", input_buffer.get_buffer());
        
    }
    // println!("Buffer contained: {}", input_buffer.get_buffer());
}