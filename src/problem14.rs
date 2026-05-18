use core::iter::Iterator;
use std::io::{self, BufRead};

/*
 * Complete the 'isAnagram' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. STRING s
 *  2. STRING t
 */

fn isAnagram(s: &str, t: &str) -> i32 {
    if s.len() != t.len() {
        return 0;
    }
    // Write your code here
    let mut sorted_s: Vec<char> = s.chars().collect();
    sorted_s.sort();
    let mut sorted_t: Vec<char> = t.chars().collect();
    sorted_t.sort();

    if sorted_s == sorted_t {
        return 1;
    }
    0
}
