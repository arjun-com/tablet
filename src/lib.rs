pub fn replace_leading_spaces_with_tabs(line: &str, spaces_per_tab: usize) -> String {
	let mut result = String::new();
	let mut space_count = 0;

	for c in line.chars() {
		if c == ' ' {
			space_count += 1;
			if space_count == spaces_per_tab {
				result.push('\t');
				space_count = 0;
			}
		}
		else {
			if space_count > 0 {
				result.push_str(&" ".repeat(space_count));
				space_count = 0;
			}
			result.push(c);
		}
	}

	// Add any remaining spaces that were not converted to tabs
	if space_count > 0 {
		result.push_str(&" ".repeat(space_count));
	}

	result
}
