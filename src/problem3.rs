use std::io::{self, BufRead};

/*
 * Complete the 'isAlphabeticPalindrome' function below.
 *
 * The function is expected to return a BOOLEAN.
 * The function accepts STRING code as parameter.
 */

fn isAlphabeticPalindrome(code: &str) -> bool {
    //first we extract only letters
    let letters = extract_letters(code);
    //then we extract
}

fn extract_letters(code: &str) -> Vec<char> {
    code.chars()
        .into_iter()
        .filter(|c| c.is_ascii_alphabetic())
        .collect()
}
