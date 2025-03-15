

mod read_toml_field;  // This declares the module and tells Rust to look for handle_gpg.rs
use crate::read_toml_field::{
    read_field_from_toml,
    read_basename_fields_from_toml,
    read_single_line_string_field_from_toml,
    read_multi_line_toml_string,
    read_integer_array,
    read_singleline_string_from_clearsigntoml,
    read_multiline_string_from_clearsigntoml,
}; 

fn main() -> Result<(), String> {
    let value = read_field_from_toml("test.toml", "fieldname");
    println!("Field value -> {}", value);
    
    // Read all prompt fields
    let prompt_values = read_basename_fields_from_toml("config.toml", "prompt");
    println!("Prompts: {:?}", prompt_values);

    let single_line = read_single_line_string_field_from_toml("config.toml", "promptsdir_1")?;
    let multi_line = read_multi_line_toml_string("config.toml", "multi_line")?;
    let integer_array = read_integer_array("config.toml", "schedule_duration_start_end")?;
    
    println!("Single line: {}", single_line);
    println!("Multi line: {}", multi_line);
    println!("Numbers: {:?}", integer_array);
    
    Ok(())
}
