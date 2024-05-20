mod lib;
mod utils;

use std::env;
use std::process;

fn main() {
	// Collect command line arguments
	let args: Vec<String> = env::args().collect();
	if args.len() < 3 {
		eprintln!("Usage: {} <spaces_per_tab> <filename x> <filename y> <filename z> <filename ...>", args[0]);
		process::exit(1);
	}

	// Parse the number of spaces per tab from the command line argument
	let spaces_per_tab: usize = args[1].parse().unwrap_or_else(|_| {
		eprintln!("Invalid number of spaces per tab: {}", args[1]);
		process::exit(1);
	});

	// Process files

	for filename in &args[2 .. args.len()] {
		if let Err(e) = utils::process_file(filename, spaces_per_tab) {
			eprintln!("Error processing file: {}\nError: {}", filename, e);
			process::exit(1);
		}
	}
}
