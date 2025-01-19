//! External Input Buffer Demo
//! 
//! Demonstrates character-by-character input handling with
//! both memory and file storage capabilities.

use std::thread;
use std::time::Duration;
use std::io;

mod external_user_input_buffer;
use external_user_input_buffer::ExternalizedInputBuffer;

fn main() -> io::Result<()> {
    let mut input = ExternalizedInputBuffer::new()?;
    
    println!("Type characters (available immediately)...");
    println!("Press Enter to flush, Ctrl+C to exit");

    loop {
        if let Ok(Some(c)) = input.get_char() {
            println!("Character received: {}", c);
            println!("Current buffer: {}", input.get_current_content());
            
            if c == '\n' {
                let content = input.flush()?;
                println!("Buffer flushed: {}", content);
            }
        }
        
        thread::sleep(Duration::from_millis(10));
    }
}