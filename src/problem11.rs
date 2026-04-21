/*
 * Next Greater Element with Position Offset

Given an integer array readings, return an array result where result[i] = [value, distance], with value being the next greater element to the right of readings[i] and distance being the index difference. If no greater element exists, return [-1, -1].

Example

Input

readings = [2, 1, 2, 4, 3]

Output

[[4, 3], [2, 1], [4, 1], [-1, -1], [-1, -1]]
 *
 * Complete the 'findNextGreaterElementsWithDistance' function below.
 *
 * The function is expected to return a 2D_INTEGER_ARRAY.
 * The function accepts INTEGER_ARRAY readings as parameter.
 */

fn findNextGreaterElementsWithDistance(readings: &[i32]) -> Vec<Vec<i32>> {
    let mut greater_than_pairs = Vec::new();
    for i in 0..readings.len() - 1 {
        let num_to_check = readings[i];
        for j in (i + 1)..readings.len() {
            let next_num = readings[j];
        }
    }
    //start with first number
    // Write your code here
    return greater_than_pairs;
}
