use std::io::{self, BufRead};
use std::path::PathBuf;
use std::fs;

fn main() -> io::Result<()> {
    println!("Directory Tree Creator");
    println!("----------------------");
    println!("Paste your directory tree (Ctrl+D or Ctrl+Z then Enter when done):");

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().collect::<Result<Vec<_>, _>>()?;

    if lines.is_empty() {
        eprintln!("Error: No input provided.");
        return Ok(());
    }

    // Extract and process the root directory
    let root_dir = lines.remove(0);
    let root_dir = root_dir.trim().trim_end_matches('/').to_string();

    // Get the base directory from the user
    println!("Enter base directory (or press Enter for current directory):");
    let mut base_dir = String::new();
    io::stdin().read_line(&mut base_dir)?;
    let base_dir = base_dir.trim();
    let base_dir = if base_dir.is_empty() {
        PathBuf::from(".")
    } else {
        PathBuf::from(base_dir)
    };

    let mut stack: Vec<String> = vec![root_dir.clone()];
    let mut paths_to_create = Vec::new();

    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Replace tree characters with spaces to calculate indentation
        let replaced_line = line.replace(|c| matches!(c, '├' | '└' | '│'), " ");
        let parts: Vec<&str> = replaced_line.split("── ").collect();
        if parts.len() < 2 {
            continue; // Skip invalid lines
        }

        let indentation_part = parts[0];
        let name = parts[1].trim();
        if name.is_empty() {
            continue; // Skip lines without a name
        }

        let effective_indentation = indentation_part.len();
        let depth = effective_indentation / 4;

        // Adjust the stack to the current depth
        while stack.len() > depth + 1 {
            stack.pop();
        }

        // Build the current directory path from the stack
        let current_dir = stack.join("/");
        let full_path = base_dir.join(&current_dir).join(name);
        paths_to_create.push(full_path.clone());

        // If the entry is a directory, push to the stack
        if name.ends_with('/') {
            let dir_name = name.trim_end_matches('/').to_string();
            stack.push(dir_name);
        }
    }

    // Display the structure to be created
    println!("\nThis will create:");
    for path in &paths_to_create {
        println!("    {}", path.display());
    }

    // Ask for confirmation
    println!("\nProceed with creation? (y/n)");
    let mut proceed = String::new();
    io::stdin().read_line(&mut proceed)?;
    if proceed.trim().to_lowercase() != "y" {
        println!("Aborted.");
        return Ok(());
    }

    // Create directories and files
    for path in &paths_to_create {
        let path_str = path.to_string_lossy();
        if path_str.ends_with('/') {
            // It's a directory, create it
            fs::create_dir_all(path)?;
        } else {
            // Create parent directories and the file
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::File::create(path)?;
        }
    }

    println!("Directory tree created successfully.");

    Ok(())
}


