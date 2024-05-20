use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use crate::lib;

pub fn process_file(filename: &str, spaces_per_tab: usize) -> io::Result<()> {
	// Open the file for reading
	let file = File::open(filename)?;
	let reader = BufReader::new(file);

	let mut modified_lines = Vec::new();
	for line in reader.lines() {
		let line = line?;
		modified_lines.push(lib::replace_leading_spaces_with_tabs(&line, spaces_per_tab));
	}

	// Open the file for writing and write all modified lines
	let mut file = File::create(filename)?;
	for line in modified_lines {
		writeln!(file, "{}", line)?;
	}

	Ok(())
}
