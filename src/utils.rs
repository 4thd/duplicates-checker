use std::env;
use std::fs;
use std::io;
use std::io::Result;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn list_files() -> Result<()> {
    match env::current_dir() {
        Ok(path) => println!("Current path: {}", path.display()),
        Err(e) => eprint!("Error fetching current path: {}", e),
    }

    let entries = fs::read_dir(".")?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() || path.is_dir() {
            // Check if the path is either a file or directory
            if let Some(name) = path.file_name() {
                if let Some(name_str) = name.to_str() {
                    println!("=> {:?}", name_str);
                }
            }
        }
    }

    Ok(())
}

pub fn get_directory_path() -> String {
    println!("Enter directory: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error for input");
    let result = input.trim();
    result.to_string()
}

pub struct Counter {
    analyzed: u64,
    copies: u64,
}

impl Counter {
    pub fn new() -> Self {
        Counter {
            analyzed: 0,
            copies: 0,
        }
    }

    pub fn incrementer_analyzed(&mut self) {
        self.analyzed += 1;
    }
    pub fn incrementer_copy(&mut self) {
        self.copies += 1;
    }

    pub fn display_analyzed(&self) -> u64 {
        self.analyzed
    }

    pub fn display_copied(&self) -> u64 {
        self.copies
    }
}

pub fn move_files() {
    let path_to_file = "copies.txt";
    let file = fs::File::open(path_to_file).expect("Error reading file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line_content) = line {
            let parts: Vec<&str> = line_content.split(" - Copie de").collect();
            if parts.len() == 2 {
                let path_to_copy = parts[0].trim();
                let dest_folder = "doublons";

                let dest_path =
                    Path::new(dest_folder).join(Path::new(path_to_copy).file_name().unwrap());

                if !Path::new(dest_folder).exists() {
                    fs::create_dir(dest_folder).expect("Erreur lors de la création du répertoire");
                }
                fs::rename(path_to_copy, dest_path).unwrap_or_else(|err| {
                    eprintln!("Erreur lors du déplacement de '{}': {}", path_to_copy, err);
                });
                println!("[+] File {} moved to doublons folder", path_to_copy);
            }
        }
    }
}
