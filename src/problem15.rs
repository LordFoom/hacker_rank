use core::iter::{IntoIterator, Iterator};
use std::collections::{HashMap, VecDeque};

fn minTasksToCancelForNoConflict(digits: &str) -> Vec<String> {
    // Write your code here
    let mut digit_chars_map = construct_map();
    // let chars = digits.chars().collect();
    letter_combinations(digits, &mut digit_chars_map)
    // .digits
    // .chars()
    // .for_each(|d| {
    //     let digit_chars = digit_to_chars.get(d);
    //     possible_chars.push(digit_chars);
    // });
}

fn letter_combinations(digits: &str, digit_map: &HashMap<char, Vec<char>>) -> Vec<String> {
    let mut result = Vec::new();
    if digits.is_empty() {
        return result;
    }
    let groups: Vec<&Vec<char>> = digits.chars().filter_map(|d| digit_map.get(&d)).collect();
    let mut current = String::new();

    if groups.len() == 1 {
        let simple_group = groups[0].iter().map(|c| c.to_string()).collect();
        return simple_group;
    }
    //we iterate over all the groups of chars, eg 2=>"a,b,c", 3=>"d,e,f"
    for i in 0..groups.len() - 1 {
        for j in i..groups.len() {
            let first_char_group = groups[i];
            let second_char_group = groups[j];
            for first_char in first_char_group {
                for second_char in second_char_group {
                    result.push(format!("{}{}", first_char, second_char))
                }
            }
        }
    }
    backtrack(&groups, 0, &mut current, &mut result);
    result
}

fn permutationification(prefix: &str, suffix_possibles: &Vec<Vec<char>>) -> Vec<String> {
    let mut result = VecDeque::new();
    if suffix_possibles.len() == 0 {
        result.push_back(prefix.to_string());
        return result.into();
    }
    for possibilities in suffix_possibles {
        let mut possibilities_deq = VecDeque::from(possibilities.clone());
        //we take the tail of the outer vec of vecs to recurse
        possibilities_deq.pop_front();
        for curr_char in possibilities {
            let new_prefix = format!("{}{}", prefix, curr_char);
            let calculated_possibilities = permutification(new_prefix, possibilities_deq.into());
        }
    }
    result.into()
}

fn backtrack(groups: &[&Vec<char>], index: usize, current: &mut String, result: &mut Vec<String>) {
    if index == groups.len() {
        result.push(current.clone());
        return;
    }
    for &ch in groups[index] {
        current.push(ch);
        backtrack(groups, index + 1, current, result);
    }
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
    number_letter.insert('7', vec!['p', 'q', 'r', 's']);
    number_letter.insert('8', vec!['t', 'u', 'v']);
    number_letter.insert('9', vec!['w', 'x', 'y', 'z']);

    number_letter
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut v: Vec<String>) -> Vec<String> {
        v.sort();
        v
    }

    #[test]
    fn empty_input_yields_empty() {
        let map = construct_map();
        assert_eq!(letter_combinations("", &map), Vec::<String>::new());
    }

    #[test]
    fn single_digit_three_letters() {
        let map = construct_map();
        assert_eq!(sorted(letter_combinations("2", &map)), vec!["a", "b", "c"]);
    }

    #[test]
    fn single_digit_with_self_mapping() {
        let map = construct_map();
        // '1' maps to vec!['1'], so the only combination is "1"
        assert_eq!(letter_combinations("1", &map), vec!["1"]);
    }

    #[test]
    fn two_digits_cartesian_product() {
        let map = construct_map();
        // "23" -> 3 * 3 = 9 combinations, digit order preserved (2 then 3)
        let result = sorted(letter_combinations("23", &map));
        assert_eq!(
            result,
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
    }

    #[test]
    fn combination_count_is_product_of_group_sizes() {
        let map = construct_map();
        // "234" -> 3 * 3 * 3 = 27
        assert_eq!(letter_combinations("234", &map).len(), 27);
        // "79" -> 3 * 4 = 12  (note: '7' has 3 letters here because 'p' is missing)
        assert_eq!(letter_combinations("79", &map).len(), 12);
    }

    #[test]
    fn order_is_preserved_per_digit() {
        let map = construct_map();
        // First combination should take the first letter of each group in order.
        let result = letter_combinations("23", &map);
        assert_eq!(result.first().unwrap(), "ad");
    }

    #[test]
    fn construct_number_vecs_flattens_in_order() {
        let map = construct_map();
        assert_eq!(
            construct_number_vecs("23", &map),
            vec!['a', 'b', 'c', 'd', 'e', 'f']
        );
    }

    #[test]
    fn construct_number_vecs_empty() {
        let map = construct_map();
        assert_eq!(construct_number_vecs("", &map), Vec::<char>::new());
    }

    #[test]
    fn permute_single_char() {
        let mut chars = vec!['a'];
        let mut result = Vec::new();
        permute(&mut chars, 0, &mut result);
        assert_eq!(result, vec!["a"]);
    }

    #[test]
    fn permute_produces_factorial_count() {
        let mut chars = vec!['a', 'b', 'c'];
        let mut result = Vec::new();
        permute(&mut chars, 0, &mut result);
        // 3! = 6 permutations
        assert_eq!(result.len(), 6);
    }

    #[test]
    fn permute_all_orderings() {
        let mut chars = vec!['a', 'b', 'c'];
        let mut result = Vec::new();
        permute(&mut chars, 0, &mut result);
        assert_eq!(
            sorted(result),
            vec!["abc", "acb", "bac", "bca", "cab", "cba"]
        );
    }

    #[test]
    fn permute_restores_input_vec() {
        // The swap/swap-back means chars should be unchanged after permuting.
        let mut chars = vec!['x', 'y', 'z'];
        let original = chars.clone();
        let mut result = Vec::new();
        permute(&mut chars, 0, &mut result);
        assert_eq!(chars, original);
    }

    #[test]
    fn map_has_all_ten_digits() {
        let map = construct_map();
        for d in '0'..='9' {
            assert!(map.contains_key(&d), "missing digit {d}");
        }
    }
}
