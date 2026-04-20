/*
 */

fn generateAngleBracketSequences(n: i32) -> Vec<String> {
    // Write your code here
    let mut angle_pairs = Vec::new();
    let mut current = String::new();
    backtrack(&mut angle_pairs, &mut current, n as u32, 0);
    angle_pairs
}

fn backtrack(results: &mut Vec<String>, current: &mut String, open: u32, close: u32) {
    if open == 0 && close == 0 {
        results.push(current.clone());
        return;
    }

    if open > 0 {
        current.push('<');
        backtrack(results, current, open - 1, close + 1);
        current.pop();
    }

    if close > 0 {
        current.push('>');
        backtrack(results, current, open, close - 1);
        current.pop();
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    /// Helper that prints every sequence on its own line, with a header.
    /// `--nocapture` is needed to actually see this output:
    ///     cargo test -- --nocapture
    fn print_sequences(n: i32, seqs: &[String]) {
        println!("\n--- n = {} ({} sequences) ---", n, seqs.len());
        for s in seqs {
            println!("  {}", s);
        }
    }

    #[test]
    fn n_1_produces_one_sequence() {
        let result = generateAngleBracketSequences(1);
        print_sequences(1, &result);
        assert_eq!(result, vec!["<>"]);
    }

    #[test]
    fn n_2_produces_two_sequences() {
        let result = generateAngleBracketSequences(2);
        print_sequences(2, &result);

        let mut sorted = result.clone();
        sorted.sort();
        assert_eq!(sorted, vec!["<<>>", "<><>"]);
    }

    #[test]
    fn n_3_produces_five_sequences() {
        let result = generateAngleBracketSequences(3);
        print_sequences(3, &result);

        let mut sorted = result.clone();
        sorted.sort();
        assert_eq!(
            sorted,
            vec!["<<<>>>", "<<><>>", "<<>><>", "<><<>>", "<><><>"]
        );
    }

    #[test]
    fn n_4_matches_catalan_number() {
        // The count of valid sequences for n pairs is the nth Catalan number.
        // C_1=1, C_2=2, C_3=5, C_4=14, C_5=42
        let result = generateAngleBracketSequences(4);
        print_sequences(4, &result);
        assert_eq!(result.len(), 14);
    }

    #[test]
    fn n_5_matches_catalan_number() {
        let result = generateAngleBracketSequences(5);
        print_sequences(5, &result);
        assert_eq!(result.len(), 42);
    }

    #[test]
    fn n_0_produces_single_empty_string() {
        // Edge case: zero pairs. The recursion's base case fires immediately
        // and pushes one empty string. Worth confirming explicitly.
        let result = generateAngleBracketSequences(0);
        print_sequences(0, &result);
        assert_eq!(result, vec![""]);
    }

    #[test]
    fn every_sequence_is_balanced() {
        // Structural invariant: at no point should `>` outnumber `<` as we
        // scan left to right, and totals must match at the end.
        for n in 1..=5 {
            let result = generateAngleBracketSequences(n);
            for s in &result {
                let mut depth: i32 = 0;
                for c in s.chars() {
                    match c {
                        '<' => depth += 1,
                        '>' => depth -= 1,
                        _ => panic!("unexpected char {:?} in {}", c, s),
                    }
                    assert!(depth >= 0, "sequence {} goes negative", s);
                }
                assert_eq!(depth, 0, "sequence {} doesn't close fully", s);
            }
        }
    }
}
