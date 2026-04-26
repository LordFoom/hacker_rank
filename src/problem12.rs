/*
Remove Elements Within K Distance

Given a non-decreasing array of integers and an integer K, remove in-place any element that is within K of the previous kept element and return the new length. Use constant extra space and single pass with two pointers.

Example

Input:

timestamps = [1, 2, 3, 8, 10]
K = 3

Output:

2

 */
fn debounceTimestamps(timestamps: &[i32], K: i32) -> i32 {
    let mut check_int = timestamps[0];
    let mut ret_vec = Vec::new();
    for i in 1..timestamps.len() {
        let curr_number = timestamps[i];
        let int_minus_check = curr_number - check_int;

        if int_minus_check >= K {
            check_int = curr_number;
            ret_vec.push(curr_number);
        }
    }
    ret_vec.len() as i32
    // Write your code here
}
