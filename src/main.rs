use indicatif::{ProgressBar, ProgressStyle};
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::fs::{self, File};
use std::hash::Hasher;
use std::io::{self, BufWriter, Write};
use std::path::Path;
mod utils;
use std::process;

fn main() {

    let mut counter = utils::Counter::new();

    let output_path = "copies.txt";

    println!("Current folder: ");
    let _ = utils::list_files();

    let directory_path = utils::get_directory_path();
    let mut hash_map: HashMap<u64, Vec<String>> = HashMap::new();

    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner().template("{spinner:.green} Scanning... {msg}"));

    find_duplicates(&mut hash_map, Path::new(&directory_path), &pb)
        .expect("Error while searching copies");

    pb.finish_with_message("Done!");

    let mut output_file =
        BufWriter::new(File::create(output_path).expect("Error on output file creation"));

    for (_, filenames) in hash_map.iter() {
        if filenames.len() > 1 {
            let mut is_original = true;
            let mut original_name = String::new();

            for filename in filenames {
                counter.incrementer_analyzed();
                pb.set_message(&format!("{}", counter.display_analyzed()));

                if is_original {
                    writeln!(output_file, "{} - Original", filename)
                        .expect("Error while writing result");
                    original_name = filename.clone();
                    is_original = false;
                } else {
                    counter.incrementer_copy();
                    writeln!(output_file, "{} - Copie de {}", filename, original_name)
                        .expect("Error while writing result");
                }
            }
        }
    }
    
    output_file.flush().expect("Unable to flush data");
    output_file.get_mut().sync_all().expect("Unable to sync data");
    
    println!("Scan finished : {}", output_path);
    println!("Analyzed files: {}", counter.display_analyzed());
    println!("Copies detected: {}", counter.display_copied());
    println!("Move {} copies? YeS/n", counter.display_copied());

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to get input");
    if choice.trim() == "YeS" {
        utils::move_files();
    } else {
        process::exit(0);
    }
}

fn find_duplicates(
    hash_map: &mut HashMap<u64, Vec<String>>,
    path: &Path,
    pb: &ProgressBar,
) -> io::Result<()> {
    if path.is_file() {
        let hash = calculate_file_hash(path)?;
        hash_map
            .entry(hash)
            .or_insert(Vec::new())
            .push(path.to_string_lossy().into_owned());
        pb.tick();
    } else if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry_path = entry?.path();
            find_duplicates(hash_map, &entry_path, pb)?;
        }
    }

    Ok(())
}

fn calculate_file_hash(path: &Path) -> io::Result<u64> {
    let mut hasher = DefaultHasher::new();
    let data = fs::read(path)?;

    for byte in data.iter() {
        hasher.write_u8(*byte);
    }
    Ok(hasher.finish())
}
