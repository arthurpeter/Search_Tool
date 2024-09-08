use walkdir::WalkDir;
use std::path::{Path, PathBuf};

pub fn normalize_directory_name(directory: &str) -> String {
    directory.trim_end_matches('/').to_string()
}

// Find all matching directories with the given name
pub fn find_directories(base_directory: &Path, dir_name: &str) -> Option<Vec<PathBuf>> {
    let mut matching_directories = Vec::new();

    for entry in WalkDir::new(base_directory) {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.is_dir() && path.file_name().unwrap_or_default() == dir_name {
                    matching_directories.push(path.to_path_buf());
                }
            }
            Err(e) => eprintln!("Error reading directory entry: {}", e),
        }
    }

    if matching_directories.is_empty() {
        None
    } else {
        Some(matching_directories)
    }
}

// Search with max-depth and exit on first match
pub fn search_with_options(directory: &Path, search_name: &str, max_depth: Option<usize>) {
    let mut walker = WalkDir::new(directory);

    // Apply max depth if provided
    if let Some(depth) = max_depth {
        walker = walker.max_depth(depth);
    }

    for entry in walker {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                // Check if the directory or file name contains the search_name
                if let Some(file_name) = path.file_name() {
                    let file_name_str = file_name.to_string_lossy();
                    if file_name_str.contains(search_name) {
                        println!("{}", path.display());
                    }
                }
            }
            Err(e) => eprintln!("Error reading directory entry: {}", e),
        }
    }
}

pub fn print_help() {
    println!("Usage: search [Options]... [Pattern]");
    println!();
    println!("Searches for files or directories in the current directory and its subdirectories.");
    println!();
    println!("Options:");
    println!("[Pattern]                               A pattern in the name of the file or directory to search for.");
    println!("-d, --directory <directory-name>        Specify a child directory to search from.");
    println!("    --directory-path <directory-path>   Specify a directory path to search from.");
    println!("    --max-depth <n>                     Limit the search depth to 'n' levels.");
    println!("-h, --help                              Show this help message.");
}