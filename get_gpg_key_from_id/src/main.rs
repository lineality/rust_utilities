/*
asks for input

```bash
gpg --list-keys
```
with output key-id (if not clearly), in next line after pub  etc.

a Rust program can interact with GPG (GNU Privacy Guard) through commands to retrieve an armored public key based on the key ID. To achieve this, you can use the `std::process::Command` module to execute GPG commands from within your Rust program. Below is an example of how you can do this:

1. **Ensure GPG is installed**: Make sure GPG is installed on the system where your Rust program will run.

2. **Use Rust to execute GPG commands**: You can use the `std::process::Command` module to run GPG commands and capture the output.


### Explanation:

1. **Command Construction**:
   - `Command::new("gpg")`: Starts a new command to run the `gpg` program.
   - `.arg("--armor")`: Adds the `--armor` argument to export the key in armored format.
   - `.arg("--export")`: Adds the `--export` argument to export the key.
   - `.arg(key_id)`: Adds the key ID as an argument to specify which key to export.

2. **Executing the Command**:
   - `.output()?`: Executes the command and captures the output. The `?` operator is used to propagate any errors that occur.

3. **Handling the Output**:
   - `output.status.success()`: Checks if the command was successful.
   - `String::from_utf8_lossy(&output.stdout).to_string()`: Converts the output bytes to a UTF-8 string.
   - If the command fails, an error is returned with a message containing the standard error output.

### Best Practices:

- **Error Handling**: Always handle errors gracefully. In this example, errors are propagated using the `?` operator and checked using `output.status.success()`.
- **Security**: Be cautious with handling key IDs and ensuring that the key ID is sanitized to prevent injection attacks.
- **Dependencies**: Avoid using third-party crates for simple tasks like this unless necessary. In this case, using the standard library is sufficient.
*/


use std::process::Command;
use std::io::{self, Write};

/// gpg get public key long from public key-id
/// use std::process::Command;
/// use std::io::{self, Write};
fn get_armored_public_key(key_id: &str) -> io::Result<String> {
    /*
   // Prompt the user for the key ID
    print!("Enter the GPG key ID: ");
    if let Err(e) = io::stdout().flush() {
        eprintln!("Failed to flush stdout: {}", e);
        return;
    }

    let mut key_id = String::new();
    if let Err(e) = io::stdin().read_line(&mut key_id) {
        eprintln!("Failed to read line: {}", e);
        return;
    }

    let key_id = key_id.trim(); // Remove any trailing newline or whitespace

    match get_armored_public_key(key_id) {
        Ok(armored_key) => {
            println!("Armored Public Key:\n{}", armored_key);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    
    */
    // Construct the GPG command to export the public key in armored format
    let output = Command::new("gpg")
        .arg("--armor")
        .arg("--export")
        .arg(key_id)
        .output()?;

    // Check if the command was successful
    if output.status.success() {
        // Convert the output to a string
        let armored_key = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(armored_key)
    } else {
        // If the command failed, return an error
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to export public key: {}", String::from_utf8_lossy(&output.stderr)),
        ))
    }
}

// fn main() {
//     // Prompt the user for the key ID
//     print!("Enter the GPG key ID: ");
//     io::stdout().flush().unwrap(); // Ensure the prompt is displayed

//     let mut key_id = String::new();
//     io::stdin().read_line(&mut key_id).expect("Failed to read line");
//     let key_id = key_id.trim(); // Remove any trailing newline or whitespace

//     match get_armored_public_key(key_id) {
//         Ok(armored_key) => {
//             println!("Armored Public Key:\n{}", armored_key);
//         }
//         Err(e) => {
//             eprintln!("Error: {}", e);
//         }
//     }
// }

fn main() {
    // Prompt the user for the key ID
    print!("Enter the GPG key ID: ");
    if let Err(e) = io::stdout().flush() {
        eprintln!("Failed to flush stdout: {}", e);
        return;
    }

    let mut key_id = String::new();
    if let Err(e) = io::stdin().read_line(&mut key_id) {
        eprintln!("Failed to read line: {}", e);
        return;
    }

    let key_id = key_id.trim(); // Remove any trailing newline or whitespace

    match get_armored_public_key(key_id) {
        Ok(armored_key) => {
            println!("Armored Public Key:\n{}", armored_key);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
