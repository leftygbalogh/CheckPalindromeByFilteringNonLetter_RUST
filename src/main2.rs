fn is_alphabetic_palindrome(code: &str) -> bool {
	// Collect only alphabetic characters
	let filtered: String = code
		.chars()
		.filter(|c| c.is_alphabetic())
		.collect();

	// Reverse the filtered string
	let reversed: String = filtered.chars().rev().collect();

	// Compare ignoring case
	filtered.eq_ignore_ascii_case(&reversed)
}