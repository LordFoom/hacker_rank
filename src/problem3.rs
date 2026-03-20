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
    //now we check for palindrome
    let left = letters.iter();
    let right = letters.iter().rev();

    for (l, r) in left.zip(right) {
        if l != r {
            return false;
        }
    }
    return true;
}

///Extract only the letters, downcased
fn extract_letters(code: &str) -> Vec<char> {
    code.chars()
        .into_iter()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect()
}
