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
    // let mut positive_ints = BTreeMap::new();
    let mut positive_ints = BTreeMap::new();

    //if there is only 1 integer passed in,
    //and then it is either 1 or 2 that is smallest positive int
    if order_numbers.len() == 1 {
        if order_numbers[0] == 1 {
            return 2;
        } else {
            return 1;
        }
    }

    let mut smallest_positive_int = i32::MAX;
    let mut next_positive_int = i32::MAX;

    for int in order_numbers {
        //negative numbers do not matter
        if int < 0 {
            continue;
        }
        if smallest_positive_int > int {
            next_positive_int = smallest_positive_int;
            smallest_positive_int = int;
        } else if next_positive_int > int {
            next_positive_int = int;
        }
    }

    println!(
        "smallest_positive_int={smallest_positive_int}, next_positive_int={next_positive_int}"
    );

    for i in smallest_positive_int..next_positive_int {}
}
