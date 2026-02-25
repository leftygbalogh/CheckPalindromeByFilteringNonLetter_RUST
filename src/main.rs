use std::io::{self, BufRead};
/*
 * Complete the 'isAlphabeticPalindrome' function below.
 *
 * The function is expected to return a BOOLEAN.
 * The function accepts STRING code as parameter.
 */

mod unit_tests;

fn isAlphabeticPalindrome(code: &str) -> bool {
    let code = code
        .chars()
        .filter(|&c| c.is_alphabetic())
        .collect::<String>()
        .to_ascii_lowercase();

	let reverse = code.chars().rev().collect::<String>();

	&code == &reverse
}

fn main() {
    println!("\x1b[2J\x1b[H\x1b[3J");
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let code = stdin_iterator.next().unwrap().unwrap();

    let result = isAlphabeticPalindrome(&code);

    println!("{}", if result { 1 } else { 0 });
}
