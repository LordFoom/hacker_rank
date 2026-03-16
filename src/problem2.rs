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
    let mut order_numbers = Vec::from(orderNumbers);

    if order_numbers.is_empty() {
        return 0;
    }

    //if there is only 1 integer passed in,
    //and then it is either 1 or 2 that is smallest positive int
    if order_numbers.len() == 1 {
        if order_numbers[0] == 1 {
            return 2;
        } else {
            return 1;
        }
    }

    //apparently all this is bullshit, we should be using cycle sort - would I have worked that out
    //in enough time? Who knows - I do, I would not have

    for i in 0..order_numbers.len() {
        while order_numbers[i] >= 1
            && order_numbers[i] <= order_numbers.len() as i32
            && order_numbers[i] != order_numbers[(order_numbers[i] - 1) as usize]
        {
            //the -1 is why we use the order_numbers
            let idx = (order_numbers[i] - 1) as usize;
            order_numbers.swap(idx, i);
        }
    }

    let smallest_positive_int = (0..order_numbers.len())
        .find(|&i| i + 1 != order_numbers[i] as usize)
        .map(|num_opt| num_opt + 1)
        .unwrap_or(order_numbers.len() + 1);

    smallest_positive_int as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        let mut arr = [];
        assert_eq!(findSmallestMissingPositive(&mut arr), 0);
    }

    #[test]
    fn test_single_element_one() {
        let mut arr = [1];
        assert_eq!(findSmallestMissingPositive(&mut arr), 2);
    }

    #[test]
    fn test_single_element_not_one() {
        let mut arr = [5];
        assert_eq!(findSmallestMissingPositive(&mut arr), 1);
    }

    #[test]
    fn test_missing_one() {
        let mut arr = [2, 3, 4, 5];
        assert_eq!(findSmallestMissingPositive(&mut arr), 1);
    }

    #[test]
    fn test_gap_in_middle() {
        let mut arr = [1, 2, 4, 5];
        assert_eq!(findSmallestMissingPositive(&mut arr), 3);
    }

    #[test]
    fn test_all_present() {
        let mut arr = [1, 2, 3, 4, 5];
        assert_eq!(findSmallestMissingPositive(&mut arr), 6);
    }

    #[test]
    fn test_unsorted() {
        let mut arr = [3, 1, 5, 2];
        assert_eq!(findSmallestMissingPositive(&mut arr), 4);
    }

    #[test]
    fn test_with_negatives_and_zeros() {
        let mut arr = [-1, 0, 1, 2];
        assert_eq!(findSmallestMissingPositive(&mut arr), 3);
    }

    #[test]
    fn test_duplicates() {
        let mut arr = [1, 1, 2, 2];
        assert_eq!(findSmallestMissingPositive(&mut arr), 3);
    }

    #[test]
    fn test_large_numbers_only() {
        let mut arr = [100, 200, 300];
        assert_eq!(findSmallestMissingPositive(&mut arr), 1);
    }
}
