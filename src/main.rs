use std::io::{self, BufRead};



/*
 * Complete the 'isAlphabeticPalindrome' function below.
 *
 * The function is expected to return a BOOLEAN.
 * The function accepts STRING code as parameter.
 */

fn isAlphabeticPalindrome(code: &str) -> bool {

	let code = code.chars().filter(|&c| c.is_alphabetic()).collect::<String>();
	//print!("{}", &code);
	let reverse = code.chars().rev().collect::<String>();
	//print!("{}", &reverse);
	//println!("{}", &code == &reverse);
	&code == &reverse

}

fn main() {
	let stdin = io::stdin();
	let mut stdin_iterator = stdin.lock().lines();

	let code = stdin_iterator.next().unwrap().unwrap();

	let result = isAlphabeticPalindrome(&code);

	println!("{}", if result { 1 } else { 0 });
}
