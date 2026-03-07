use std::collections::Hashmap;
///Find the Smallest Missing Positive Integer
///Given an unsorted array of integers, find the smallest positive integer not present in the array in O(n) time and O(1) extra space.
///
/*
 * Complete the 'findSmallestMissingPositive' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY orderNumbers as parameter.
 */

fn findSmallestMissingPositive(orderNumbers: &[i32]) -> i32 {
    let mut positive_ints = Hashmap::from([
        (1, false),
        (2, false),
        (3, false),
        (4, false),
        (5, false),
        (6, false),
        (7, false),
        (8, false),
        (9, false),
    ]);
}
