/*
 * Complete the 'areBracketsProperlyMatched' function below.
 *
 * The function is expected to return a BOOLEAN.
 * The function accepts STRING code_snippet as parameter.
 */

fn areBracketsProperlyMatched(code_snippet: &str) -> bool {
    let left_brackets: Vec<char> = vec!['{', '[', '('];
    let right_brackets: Vec<char> = vec!['}', ']', ')'];
    let mut stack = Vec::new();
    // Write your code here
    // let mut stack = code_snippet
    //     .chars()
    //     .into_iter()
    //     .filter(|c| brackets.contains(c))
    //     .collect::<Vec<char>>();
    // .for_each(|bracket| stack.push(bracket));
    let mut balanced = true;
    for c in code_snippet.chars() {
        if left_brackets.contains(&c) {
            stack.push(c);
        } else if right_brackets.contains(&c) {
            if stack.is_empty() {
                balanced = false;
                break;
            }
            let pop_c = stack.pop().unwrap_or('!');
            if pop_c == '{' && c != '}' || pop_c == '[' && c != ']' || pop_c == '(' && c != ')' {
                return false;
            }
            // else {
            //     panic!("Unknown value! {pop_c}");
            // }
        }
    }
    balanced
}
