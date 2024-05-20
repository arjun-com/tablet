mod lib;
mod utils;

use std::env;
use std::process;

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <filename> <spaces_per_tab>", args[0]);
        process::exit(1);
    }

    // Parse the number of spaces per tab from the command line argument
    let spaces_per_tab: usize = args[2].parse().unwrap_or_else(|_| {
        eprintln!("Invalid number of spaces per tab: {}", args[2]);
        process::exit(1);
    });

    // Process the file
    if let Err(e) = utils::process_file(&args[1], spaces_per_tab) {
        eprintln!("Error processing file: {}", e);
        process::exit(1);
    }
}
