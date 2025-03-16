// src/main.rs

// import manage_absolute_executable_directory_relative_paths module w/ these 2 lines
mod manage_absolute_executable_directory_relative_paths;
use manage_absolute_executable_directory_relative_paths::make_input_path_name_abs_executabledirectoryrelative_nocheck;

fn main() {
    // Get a path relative to the executable directory, not the CWD
    match make_input_path_name_abs_executabledirectoryrelative_nocheck("data/config.json") {
        Ok(absolute_path) => println!("Absolute path: {}", absolute_path.display()),
        Err(e) => {
            eprintln!("Error resolving path: {}", e);
            std::process::exit(1);
        }
    }
}