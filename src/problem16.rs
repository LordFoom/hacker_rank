use std::collections::VecDeque;
/*
 * Complete the 'countInstallationSequences' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts INTEGER n as parameter.
 *
 * Ways to Fill Slots with Single or Double Coverage
Given n slots numbered 0 to n-1, return the number of ways to fill all slots where each operation covers either 1 slot or 2 adjacent slots.
Examples
Example 1
Input:
n = 3
Output:
3
Explanation:
    We want the number of ways to install panels in 3 slots with operations of size 1 or 2. Let dp[i] be the number of ways to fill i slots.
        Base cases: dp[0] = 1 (do nothing), dp[1] = 1 (only one 1-panel install).
        For each i ≥ 2: dp[i] = dp[i-1] + dp[i-2].
            dp[2] = dp[1] + dp[0] = 1 + 1 = 2 (sequences: [1,1], [2])
            dp[3] = dp[2] + dp[1] = 2 + 1 = 3 (sequences: [1,1,1], [1,2], [2,1])
    Therefore, the answer for n=3 is "3".
Example 2
Input:
n = 5
Output:
8
Explanation: Compute dp up to 5:
    dp[0]=1, dp[1]=1
    dp[2]=dp[1]+dp[0]=1+1=2 ( [1,1], [2] )
    dp[3]=dp[2]+dp[1]=2+1=3 ( [1,1,1], [1,2], [2,1] )
    dp[4]=dp[3]+dp[2]=3+2=5 ( [1,1,1,1], [1,1,2], [1,2,1], [2,1,1], [2,2] )
    dp[5]=dp[4]+dp[3]=5+3=8 (eight distinct sequences)
        Hence for n=5 the function returns "8".
Input Format
The input consists of a single integer n on one line.
Constraints
    0 <= n <= 1000
Output Format
Output a single line containing the number of distinct installation sequences for a rack of size n, represented as a decimal string.
Sample Input 0
2
Sample Output 0
2
Sample Input 1
3
Sample Output 1
3
 */
fn countInstallationSequences(n: i32) -> String {
    // This is basically the fibonacci
    if n == 0 || n == 1 {
        return "1".to_string();
    }
    let mut prev = "1".to_string();
    let mut prev_prev = "1".to_string();
    // let mut prev = BigInt::from(1);
    // let mut prev_prev = BigInt::from(1);
    for i in 2..=n {
        let tmp = prev.clone();
        // println!("{i} iter, pre==============");
        // println!("tmp={tmp}");
        // println!("prev={prev}");
        // println!("prev_prev={prev_prev}");
        prev = add_string(&prev, &prev_prev);
        prev_prev = tmp;
        // prev = prev + prev_prev;
        // prev_prev = tmp;
        // println!("{i} iter, post==============");
        // println!("tmp={tmp}");
        // println!("prev={prev}");
        // println!("prev_prev={prev_prev}");
    }
    // let ret_val = prev;
    eprintln!("Returning {prev}");
    prev
    // ret_val.to_string()
}

fn add_string(num1: &str, num2: &str) -> String {
    let curr_idx = 0;
    //is there a way we can zip this up?
    // num1.chars().into_iter().zip
    // or condemned to trying to iterate
    let mut big_str = Vec::new();
    let mut short_str = Vec::new();
    let mut result_str = VecDeque::new();
    // make each one the reverse and turn it into a collection of numbers
    if num1.len() >= num2.len() {
        big_str = num1.chars().rev().collect();
        short_str = num2.chars().rev().collect();
    } else {
        big_str = num2.chars().rev().collect();
        short_str = num1.chars().rev().collect();
    }
    let mut carry_over_digit = 0;
    for (index, num_char_1) in big_str.iter().enumerate() {
        let num_1 = num_char_1.to_digit(10).unwrap();
        // let num_2 = num_char_2.to_digit();

        let num_2 = if let Some(num) = short_str.get(index) {
            num.to_digit(10).unwrap()
        } else {
            0
        };
        println!("Adding {num_1} to {num_2}");
        let num_3 = num_1 + num_2 + carry_over_digit;
        let final_num = if num_3 <= 9 {
            carry_over_digit = 0;
            num_3
        } else {
            carry_over_digit = 1;
            num_3 - 10
        };
        println!("carry_over_digit = {carry_over_digit}");
        println!("final_num = {final_num}");
        result_str.push_front(final_num);
    }

    if carry_over_digit != 0 {
        result_str.push_front(carry_over_digit);
    }
    result_str.into_iter().map(|num| format!("{num}")).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn prb_16_test_low_val() {
        let n = 2;
        let count = countInstallationSequences(n);
        assert_eq!("2", count);
        let n = 3;
        let count = countInstallationSequences(n);
        assert_eq!("3", count);
        let n = 4;
        let count = countInstallationSequences(n);
        assert_eq!("5", count);
    }

    #[test]
    fn prb_16_test_less_low_val() {
        let n = 8;
        let count = countInstallationSequences(n);
        assert_eq!("34", count);
    }

    #[test]
    fn prb_16_test_15() {
        let n = 14;
        let count = countInstallationSequences(n);
        assert_eq!("610", count);
    }

    #[test]
    fn prb_16_test_add_string_low() {
        let a_str = "994";
        let b_str = "7";
        let ab_str = add_string(a_str, b_str);
        assert_eq!("1001", ab_str);
    }
    #[test]
    fn prb_16_test_add_string_medium() {}
}
