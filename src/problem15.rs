use core::iter::{IntoIterator, Iterator};
use std::collections::HashMap;

fn minTasksToCancelForNoConflict(digits: &str) -> Vec<String> {
    // Write your code here
    let digit_chars_map = construct_map();
    let mut digits_to_chars = construct_number_vecs(digits, &digit_chars_map);
    let mut result = Vec::new();
    // let chars = digits.chars().collect();
    permute(&mut digits_to_chars, 0, &mut result);
    result
    // .digits
    // .chars()
    // .for_each(|d| {
    //     let digit_chars = digit_to_chars.get(d);
    //     possible_chars.push(digit_chars);
    // });
}

fn construct_number_vecs(digits: &str, digit_map: &HashMap<char, Vec<char>>) -> Vec<char> {
    digits
        .chars()
        .flat_map(|d| digit_map.get(&d).into_iter().flatten().copied())
        .collect()
}

fn permute(chars: &mut Vec<char>, start: usize, result: &mut Vec<String>) {
    if start == chars.len() {
        result.push(chars.iter().collect());
        return;
    }

    for i in start..chars.len() {
        chars.swap(start, i);
        permute(chars, start + 1, result);
        chars.swap(start, i);
    }
}

fn construct_map() -> HashMap<char, Vec<char>> {
    let mut number_letter = HashMap::new();

    number_letter.insert('0', vec!['0']);
    number_letter.insert('1', vec!['1']);
    number_letter.insert('2', vec!['a', 'b', 'c']);
    number_letter.insert('3', vec!['d', 'e', 'f']);
    number_letter.insert('4', vec!['g', 'h', 'i']);
    number_letter.insert('5', vec!['j', 'k', 'l']);
    number_letter.insert('6', vec!['m', 'n', 'o']);
    number_letter.insert('7', vec!['q', 'r', 's']);
    number_letter.insert('8', vec!['t', 'u', 'v']);
    number_letter.insert('9', vec!['w', 'x', 'y', 'z']);

    number_letter
}
