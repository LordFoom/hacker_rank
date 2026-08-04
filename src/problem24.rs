use std::collections::BTreeSet;
/**
* Longest Arithmetic Subsequence with Given Difference

Given an array of integers and a positive integer k, find the length of the longest arithmetic progression with common difference k. Ignore duplicates.

Example

Input

arr = [8, 1, -1, 0, 3, 6, 2, 4, 5, 7, 9]
k = 2

Output

6

Explanation

Remove duplicates (none here) and consider the set of unique elements: 

We seek the longest arithmetic progression with difference k=2. 

Starting at -1 gives the sequence [-1,1,3,5,7,9] of length 6. 
No other starting point yields a longer progression, so the result is 6.

Input Format

    The first line contains an integer n denoting the number of elements in the array.
    The next n lines contains an integer denoting elements in the array.
    The last line contains the value for integer k.

*/
fn findLongestArithmeticProgression(arr: &[i32], k: i32) -> i32 {
    let mut sorted_num_set = BTreeSet::new();
    //keeps things sorted on insert
    arr.iter().for_each(|i| sorted_num_set.insert(i));
    let sorted_nums = sorted_num_set.into_iter().collect::<Vec<i32>>();
    let mut curr_max = 0;
    let mut first_pointer = 0;
    let mut second_pointer = 1;
    let mut i = 0;
    while i < sorted_nums.len()-1 {
        let first_num = sorted_nums[i];
    }
    for i in sorted_nums.keys(){
        if i == sorted_nums.len()-1 {
            continue
        }
        

    }
    k
}



