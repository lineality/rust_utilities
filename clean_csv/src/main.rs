use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn clean_csv_file(input_path: &str, output_path: &str) -> io::Result<()> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut writer = File::create(output_path)?;
    
    let mut current_record = Vec::new();
    let mut is_first = true;

    // Write header first
    if let Some(Ok(header)) = reader.lines().next() {
        writeln!(writer, "{}", header)?;
    }

    for line in reader.lines() {
        let line = line?;
        
        // New record starts with a number (ID)
        if line.split(',').next()
            .and_then(|s| s.parse::<u32>().ok())
            .is_some() 
        {
            // Write previous record if exists
            if !current_record.is_empty() {
                let combined = current_record.join(" ");
                if !is_first {
                    writeln!(writer, "{}", combined)?;
                }
                current_record.clear();
                is_first = false;
            }
            current_record.push(line);
        } else {
            // Continue current record
            if !current_record.is_empty() {
                current_record.push(line);
            }
        }
    }

    // Write final record
    if !current_record.is_empty() {
        let combined = current_record.join(" ");
        writeln!(writer, "{}", combined)?;
    }

    Ok(())
}

fn inspect_csv(path: &str) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    
    println!("Inspecting cleaned CSV:");
    for (i, line) in reader.lines().take(5).enumerate() {
        println!("Record {}: {:?}", i, line?);
    }
    Ok(())
}

fn main() -> io::Result<()> {
    
    // Inspect CSV before processing
    println!("Inspecting train.csv structure:");
    inspect_csv("file_targets/train.csv")?;
        
    // First clean the CSV
    println!("Cleaning CSV file...");
    clean_csv_file(
        "file_targets/train.csv",
        "file_targets/train_cleaned.csv"
    )?;
    println!("CSV cleaning complete. Output saved to train_cleaned.csv");

    // Inspect CSV before processing
    println!("Inspecting train.csv structure:");
    inspect_csv("file_targets/train.csv")?;
}