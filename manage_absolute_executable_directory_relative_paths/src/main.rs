// src/main.rs

// import manage_absolute_executable_directory_relative_paths module w/ these 2 lines
mod manage_absolute_executable_directory_relative_paths;
use manage_absolute_executable_directory_relative_paths::get_absolute_executable_directory_relative_path;

fn main() {
    // Get a path relative to the executable directory, not the CWD
    match get_absolute_executable_directory_relative_path("data/config.json") {
        Ok(absolute_path) => println!("Absolute path: {}", absolute_path.display()),
        Err(e) => {
            eprintln!("Error resolving path: {}", e);
            std::process::exit(1);
        }
    }
}
