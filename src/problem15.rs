use core::iter::{IntoIterator, Iterator};
use std::collections::{HashMap, VecDeque};

fn minTasksToCancelForNoConflict(digits: &str) -> Vec<String> {
    // Write your code here
    let mut digit_chars_map = construct_map();
    // permutationification("", digit_chars_map)
    letter_combinations(digits, &mut digit_chars_map)
}

fn letter_combinations(digits: &str, digit_map: &HashMap<char, Vec<char>>) -> Vec<String> {
    // if digits.is_empty() {
    //     return Vec::new();
    // }
    let groups: Vec<Vec<char>> = digits
        .chars()
        .filter_map(|d| digit_map.get(&d).cloned())
        .collect();
    let mut result = Vec::new();
    backtrack("", &groups, &mut result);
    result.sort();
    result
}

fn backtrack(prefix: &str, groups: &[Vec<char>], result: &mut Vec<String>) {
    if groups.is_empty() {
        result.push(prefix.to_string());
        return;
    }
    for &ch in &groups[0] {
        let new_prefix = format!("{prefix}{ch}");
        backtrack(&new_prefix, &groups[1..], result);
    }
}

fn permutationification(prefix: &str, suffix_possibles: &[Vec<char>]) -> Vec<String> {
    let mut result = Vec::new();
    if suffix_possibles.len() == 0 {
        result.push(prefix.to_string());
        return result;
    }
    for ch in &suffix_possibles[0] {
        let new_prefix = format!("{}{}", prefix, ch);
        let calculated_possibilities = permutationification(&new_prefix, &suffix_possibles[1..]);
        result.extend(calculated_possibilities);
    }
    result
}

fn construct_number_vecs(digits: &str, digit_map: &HashMap<char, Vec<char>>) -> Vec<char> {
    digits
        .chars()
        .flat_map(|d| digit_map.get(&d).into_iter().flatten().copied())
        .collect()
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
    number_letter.insert('7', vec!['p', 'q', 'r', 's']);
    number_letter.insert('8', vec!['t', 'u', 'v']);
    number_letter.insert('9', vec!['w', 'x', 'y', 'z']);

    number_letter
}

#[cfg(test)]
use super::*;

#[test]
fn test_empty() {
    let map = construct_map();
    let result = letter_combinations("", &map);
    assert_eq!(result, vec!["".to_string()]);
}

#[test]
fn test_single_zero() {
    let map = construct_map();
    let result = letter_combinations("0", &map);
    assert_eq!(result, vec!["0"]);
}

#[test]
fn test_single_one() {
    let map = construct_map();
    let result = letter_combinations("1", &map);
    assert_eq!(result, vec!["1"]);
}

#[test]
fn test_single_two() {
    let map = construct_map();
    let result = letter_combinations("2", &map);
    assert_eq!(result, vec!["a", "b", "c"]);
}

#[test]
fn test_single_seven() {
    let map = construct_map();
    let result = letter_combinations("7", &map);
    assert_eq!(result, vec!["p", "q", "r", "s"]);
}

#[test]
fn test_zero_one() {
    let map = construct_map();
    let result = letter_combinations("01", &map);
    assert_eq!(result, vec!["01"]);
}

#[test]
fn test_one_zero() {
    let map = construct_map();
    let result = letter_combinations("10", &map);
    assert_eq!(result, vec!["10"]);
}

#[test]
fn test_two_zero() {
    let map = construct_map();
    let result = letter_combinations("20", &map);
    assert_eq!(result, vec!["a0", "b0", "c0"]);
}

#[test]
fn test_double_zero_two() {
    let map = construct_map();
    let result = letter_combinations("002", &map);
    assert_eq!(result, vec!["00a", "00b", "00c"]);
}

#[test]
fn test_example_23() {
    let map = construct_map();
    let result = letter_combinations("23", &map);
    let expected = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    assert_eq!(result, expected);
}

#[test]
fn test_example_203() {
    let map = construct_map();
    let result = letter_combinations("203", &map);
    let expected = vec![
        "a0d", "a0e", "a0f", "b0d", "b0e", "b0f", "c0d", "c0e", "c0f",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<_>>();
    assert_eq!(result, expected);
}

#[test]
fn test_234() {
    let map = construct_map();
    let result = letter_combinations("234", &map);
    // 3×3×3 = 27 combinations
    assert_eq!(result.len(), 27);
    assert_eq!(result[0], "adg");
    assert_eq!(result[result.len() - 1], "cfi");
}

#[test]
fn test_171717() {
    let map = construct_map();
    let result = letter_combinations("171717", &map);
    // 1^3 × 4^3 = 64 combinations
    assert_eq!(result.len(), 64);
    assert_eq!(result[0], "1p1p1p");
    assert_eq!(result[result.len() - 1], "1s1s1s");
}
mod tests {}
