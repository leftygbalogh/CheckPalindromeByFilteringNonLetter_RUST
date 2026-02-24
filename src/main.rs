/*
 * Complete the 'isAlphabeticPalindrome' function below.
 *
 * The function is expected to return a BOOLEAN.
 * The function accepts STRING code as parameter.
 */
mod main2;

fn isAlphabeticPalindrome(code: &str) -> bool {
    let code = code
        .chars()
        .filter(|&c| c.is_alphabetic())
        .collect::<String>();
    //print!("{}", &code);
    let reverse = code.chars().rev().collect::<String>();
    //print!("{}", &reverse);
    //println!("{}", &code == &reverse);
	//println!("VELO: {}", &code.eq_ignore_ascii_case(&reverse));
	*(&code.eq_ignore_ascii_case(&reverse))
	//    &code == &reverse
}

fn rotator() {
    let l = [
        "aAaÀÁÂÃÄÅàáâãäåɑΑαаᎪＡａdef£$%%*&^&*fed/ａＡᎪаαΑɑåäãâáàÅÄÃÂÁÀaAa",
        "aAaÀÁÂÃÄÅàáâãäåɑΑαаᎪＡａdef£$%%*&^&*fed/ａＡᎪаαΑɑåäãâáàÅÄÃÂÁÀaAa",
        ": ։ ܃ ܄ ∶ ꞉ ：： ꞉ ∶ ܄ ܃ ։ :",
        "B b ß ʙ Β β В Ь Ᏼ ᛒ Ｂ ｂｂ Ｂ ᛒ Ᏼ Ь В β Β ʙ ß b B",
        "C c ϲ Ϲ С с Ꮯ Ⅽ ⅽ 𐐠𺀠Ｃ ｃｃ Ｃ�𺀠�ⅽ Ⅽ Ꮯ с С Ϲ ϲ c C",
        "D d Ď ď Đ đ ԁ ժ Ꭰ ḍ Ⅾ ⅾ Ｄ ｄｄ Ｄ ⅾ Ⅾ ḍ Ꭰ ժ ԁ đ Đ ď Ď d D",
        "E e È É Ê Ë é ê ë Ē ē Ĕ ĕ Ė ė Ę Ě ě Ε Е е Ꭼ Ｅ ｅｅ Ｅ Ꭼ е Е Ε ě Ě Ę ė Ė ĕ Ĕ ē Ē ë ê é Ë Ê É È e E",
    ];

    for item in l {
        println!("{}", isAlphabeticPalindrome(item))
    }
}

fn main() {
    println!("\x1b[2J\x1b[H\x1b[3J");
    rotator();

    /*
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let code = stdin_iterator.next().unwrap().unwrap();

    let result = isAlphabeticPalindrome(&code);

    println!("{}", if result { 1 } else { 0 });

     */
}
