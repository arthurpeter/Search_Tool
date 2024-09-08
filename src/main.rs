use std::env;
use std::path::Path;
use search::{search_with_options, print_help, normalize_directory_name, find_directories};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut max_depth = None;
    let mut directories = Vec::new();
    let mut search_name = "";

    // Parse the command-line arguments
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" => {
                print_help();
                return;
            }
            "--max-depth" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: --max-depth requires a value.");
                    std::process::exit(1);
                }
                max_depth = args[i + 1].parse().ok();
                i += 2;
            }
            "-d" | "--directory" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: --directory requires a value.");
                    std::process::exit(1);
                }
                directories.push(normalize_directory_name(&args[i + 1]));
                i += 2;
            }
            "--directory-path" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: --directory-path requires a value.");
                    std::process::exit(1);
                }
                directories.push(args[i + 1].clone());
                i += 2;
            }
            _ => {
                if search_name.is_empty() {
                    search_name = &args[i];
                    i += 1;
                } else {
                    eprintln!("Error: Unrecognized argument '{}'", args[i]);
                    std::process::exit(1);
                }
            }
        }
    }

    if search_name.is_empty() {
        eprintln!("Error: Missing search pattern.");
        std::process::exit(1);
    }

    if directories.is_empty() {
        directories.push(".".to_string()); // Default to the current directory if no directory is specified
    }

    // Search within all matching directories
    for directory in &directories {
        if Path::new(directory).exists() {
            // If the provided directory exists as a valid path
            search_with_options(Path::new(directory), search_name, max_depth);
        } else if let Some(matching_directories) = find_directories(Path::new("."), directory) {
            for matching_directory in matching_directories {
                search_with_options(&matching_directory, search_name, max_depth);
            }
        } else {
            eprintln!("Error: Directory '{}' not found.", directory);
        }
    }
}
