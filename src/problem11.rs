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
    let mut index_diff;
    for i in 0..readings.len() - 1 {
        let num_to_check = readings[i];
        index_diff = 1;
        let mut gt_found = false;
        for j in (i + 1)..readings.len() {
            let next_num = readings[j];
            if next_num > num_to_check {
                let pair_vec = vec![next_num, index_diff];
                greater_than_pairs.push(pair_vec);
                gt_found = true;
                break;
            }
            index_diff += 1;
        }
        if !gt_found {
            let pair_vec = vec![-1, -1];
            greater_than_pairs.push(pair_vec);
        }
    }
    //final element always -1,-1
    greater_than_pairs.push(vec![-1, -1]);
    //add
    return greater_than_pairs;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascending_sequence() {
        let result = findNextGreaterElementsWithDistance(&[1, 2, 3]);
        assert_eq!(result, vec![vec![2, 1], vec![3, 1]]);
    }

    #[test]
    fn descending_sequence_has_no_greater_elements() {
        let result = findNextGreaterElementsWithDistance(&[3, 2, 1]);
        assert_eq!(result, vec![vec![-1, -1], vec![-1, -1]]);
    }

    #[test]
    fn mixed_order() {
        let result = findNextGreaterElementsWithDistance(&[2, 1, 3]);
        assert_eq!(result, vec![vec![3, 2], vec![3, 1]]);
    }

    #[test]
    fn equal_values_do_not_count_as_greater() {
        // The 1 at index 1 is skipped (not greater), distance increments to 2
        // before the 2 at index 2 is picked up.
        let result = findNextGreaterElementsWithDistance(&[1, 1, 2]);
        assert_eq!(result, vec![vec![2, 2], vec![2, 1]]);
    }

    #[test]
    fn single_element() {
        let result = findNextGreaterElementsWithDistance(&[5]);
        assert_eq!(result, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn two_elements_ascending() {
        let result = findNextGreaterElementsWithDistance(&[1, 2]);
        assert_eq!(result, vec![vec![2, 1]]);
    }

    #[test]
    fn two_elements_descending() {
        let result = findNextGreaterElementsWithDistance(&[2, 1]);
        assert_eq!(result, vec![vec![-1, -1]]);
    }

    #[test]
    fn negative_numbers() {
        // -3 finds -1 at distance 1; -1 has nothing greater after it.
        let result = findNextGreaterElementsWithDistance(&[-3, -1, -2]);
        assert_eq!(result, vec![vec![-1, 1], vec![-1, -1]]);
    }

    #[test]
    fn distance_counts_positions_regardless_of_gap() {
        // The `1`s in the middle aren't greater than 5, so distance keeps
        // incrementing until 10 is reached 4 positions away.
        let result = findNextGreaterElementsWithDistance(&[5, 1, 1, 1, 10]);
        assert_eq!(
            result,
            vec![vec![10, 4], vec![10, 3], vec![10, 2], vec![10, 1]],
        );
    }

    #[test]
    #[should_panic]
    fn empty_slice_panics() {
        // `readings.len() - 1` underflows on an empty slice (usize).
        findNextGreaterElementsWithDistance(&[]);
    }

    #[test]
    fn print_output_gt_prob() {
        let input = vec![2, 1, 2, 4, 3];
        let result = findNextGreaterElementsWithDistance(&input);
        println!("Input:  {:?}", input);
        println!("Output: {:?}", result);
        println!("Pretty output:");
        for (i, pair) in result.iter().enumerate() {
            println!("  [{}] = {:?}", i, pair);
        }
    }
}
