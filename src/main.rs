use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
// MIT License

// Copyright (c) 2024 Yaswanth Sai Kumar

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

fn find_duplicate_number_folders(directory: &Path) -> HashMap<String, Vec<PathBuf>> {
    let mut duplicates: HashMap<String, Vec<PathBuf>> = HashMap::new();
    let mut folder_names: HashMap<String, Vec<PathBuf>> = HashMap::new();

    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries.filter_map(|entry| entry.ok()) {
            let folder_path = entry.path();
            if let Some(folder_name) = folder_path.file_name().and_then(|name| name.to_str()) {
                let number = folder_name
                    .chars()
                    .take_while(|c| c.is_digit(10))
                    .collect::<String>();
                if !number.is_empty() {
                    folder_names.entry(number).or_default().push(folder_path);
                }
            }
        }
    }

    for (_, paths) in folder_names {
        if paths.len() > 1 {
            for path in paths {
                let number = path
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .and_then(|stem| {
                        stem.chars()
                            .take_while(|c| c.is_digit(10))
                            .collect::<String>()
                            .parse::<u32>()
                            .ok()
                    })
                    .unwrap_or_default()
                    .to_string();
                duplicates.entry(number).or_default().push(path.clone());
            }
        }
    }

    duplicates
}

fn find_duplicate_folders(directory: &Path) -> HashMap<String, Vec<PathBuf>> {
    let mut duplicates: HashMap<String, Vec<PathBuf>> = HashMap::new();
    let mut folder_names: HashMap<String, Vec<PathBuf>> = HashMap::new();

    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries.filter_map(|entry| entry.ok()) {
            let folder_path = entry.path();
            if let Some(folder_name) = folder_path.file_name().and_then(|name| name.to_str()) {
                println!("Found folder: {}", folder_name); // Debug: Print found folder name
                folder_names
                    .entry(folder_name.to_string())
                    .or_default()
                    .push(folder_path);
            }
        }
    } else {
        println!("Failed to read directory"); // Debug: Print if directory reading fails
    }

    for (_, paths) in &folder_names {
        println!("Folder paths: {:?}", paths); // Debug: Print folder paths
    }

    for (folder_name, paths) in &folder_names {
        println!("Folder name: {}", folder_name);
        println!("Paths: {:?}", paths);
        if paths.len() > 1 {
            println!("Duplicate found for folder: {}", folder_name);
            for path in paths {
                let folder_name = path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or_default()
                    .to_string();
                println!("Adding path {:?} to duplicates", path);
                duplicates
                    .entry(folder_name)
                    .or_default()
                    .push(path.clone());
            }
        }
    }

    duplicates
}

fn find_missing_serial_numbers(directory: &Path) -> Vec<u32> {
    let mut serial_numbers = Vec::new();

    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries.filter_map(|entry| entry.ok()) {
            let folder_name = entry.file_name().into_string().unwrap_or_default();
            //println!("Folder name: {:?}", folder_name); // Add this line for debugging
            let number = folder_name
                .chars()
                .take_while(|c| c.is_digit(10))
                .collect::<String>();
            if let Ok(parsed_number) = number.parse::<u32>() {
                serial_numbers.push(parsed_number);
            }
        }
    }

    serial_numbers.sort_unstable();
    serial_numbers.dedup();

    //println!("Parsed serial numbers: {:?}", serial_numbers);

    let mut missing_serial_numbers = Vec::new();
    let mut expected_number = 1;

    for &number in &serial_numbers {
        while number != expected_number {
            missing_serial_numbers.push(expected_number);
            expected_number += 1;
        }
        expected_number += 1;
    }

    //println!("Final list of serial numbers: {:?}", serial_numbers);
    //println!("Missing serial numbers: {:?}", missing_serial_numbers);

    missing_serial_numbers
}

fn count_files_and_folders(directory: &Path) -> (usize, usize) {
    let (mut num_files, mut num_folders) = (0, 0);

    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries.filter_map(|entry| entry.ok()) {
            if entry.path().is_dir() {
                num_folders += 1;
            } else {
                num_files += 1;
            }
        }
    }

    (num_files, num_folders)
}

fn prompt_for_directory() -> String {
    println!("Enter the path to the directory to search:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

fn main() {
    println!("Welcome to the Duplicate Folder Finder (eg. 01 Folder name, 02 Folder name)!");
    println!("Developer Name B.Yaswanth Sai Kumar");
    loop {
        println!("Please select an option:");
        println!("1. Find duplicate number folders");
        println!("2. Find duplicate folders (based on full folder names)");
        println!("3. Find missing serial numbers");
        println!("4. Count files and folders");
        println!("5. Exit");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim() {
            "1" => {
                let directory = prompt_for_directory();
                let directory_path = Path::new(&directory);
                let duplicate_folders = find_duplicate_number_folders(&directory_path);
                if !duplicate_folders.is_empty() {
                    println!("Duplicate number folders found:");
                    for (number, paths) in duplicate_folders {
                        println!("Number folder '{}' is duplicated at:", number);
                        for path in paths {
                            println!("- {}", path.display());
                        }
                    }
                } else {
                    println!("No duplicate number folders found.");
                }
            }
            "2" => {
                let directory = prompt_for_directory();
                let directory_path = Path::new(&directory);
                let duplicate_folders = find_duplicate_folders(&directory_path);
                if !duplicate_folders.is_empty() {
                    println!("Duplicate folders found (based on full folder names):");
                    for (name, paths) in duplicate_folders {
                        println!("Folder '{}' is duplicated at:", name);
                        for path in paths {
                            println!("- {}", path.display());
                        }
                    }
                } else {
                    println!("No duplicate folders found.");
                }
            }
            "3" => {
                let directory = prompt_for_directory();
                let directory_path = Path::new(&directory);
                let missing_serial_numbers = find_missing_serial_numbers(&directory_path);
                if !missing_serial_numbers.is_empty() {
                    println!("Missing serial numbers found:");
                    for number in missing_serial_numbers {
                        println!("{}", number);
                    }
                } else {
                    println!("No missing serial numbers found.");
                }
            }
            "4" => {
                let directory = prompt_for_directory();
                let directory_path = Path::new(&directory);
                let (num_files, num_folders) = count_files_and_folders(&directory_path);
                println!("Number of files: {}", num_files);
                println!("Number of folders: {}", num_folders);
            }
            "5" => {
                println!("Exiting the program. Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please select a valid option."),
        }
    }
}
