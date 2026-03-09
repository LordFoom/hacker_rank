use std::collections::BTreeMap;
///Find the Smallest Missing Positive Integer
///Given an unsorted array of integers, find the smallest positive integer not present in the array in O(n) time and O(1) extra space.
///
/*
 * Complete the 'findSmallestMissingPositive' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY orderNumbers as parameter.
 */

fn findSmallestMissingPositive(order_numbers: &[i32]) -> i32 {
    // let mut positive_ints = Hashmap::from([
    //     (1, false),
    //     (2, false),
    //     (3, false),
    //     (4, false),
    //     (5, false),
    //     (6, false),
    //     (7, false),
    //     (8, false),
    //     (9, false),
    // ]);

    if order_numbers.is_empty() {
        return 0;
    }
    let mut positive_ints = BTreeMap::new();

    let smallest_positive_int = order_numbers[0];

    if order_numbers.len() == 1 {
        if smallest_positive_int == 1 {
            return 2;
        } else {
            return 1;
        }
    }

    for i in 1..order_numbers.len() {
        if order_numbers[i] < smallest_positive_int {
            smallest_positive_int = order_numbers[i];
        }
    }
    // for (i, number) in order_numbers {
    //     if i == 0 {
    //         smallest_positive_int = number;
    //     }
    // }
}
