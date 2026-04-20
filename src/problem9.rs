/*
 * Count Number Pairs

Given a sorted array of positive integers and a target value
count the number of pairs (i, j) where i < j and array[i] + array[j] <= target.
 */

fn countAffordablePairs(prices: &[i32], budget: i32) -> i32 {
    //build up pairs
    // let mut pairs = Vec::new();
    let mut count = 0;
    for i in 0..prices.len().saturating_sub(1) {
        for j in i + 1..prices.len() {
            let first_num = prices[i];
            let second_num = prices[j];
            let sum = first_num + second_num;
            if sum <= budget {
                count += 1;
            }
        }
    }
    count
}
