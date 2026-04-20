/*
 *
    nums (INTEGER_ARRAY): a sorted array of distinct integers in strictly increasing order. Its length n satisfies 0 ≤ n ≤ 10^6 and each element satisfies -10^9 ≤ nums[i] ≤ 10^9.
    target (INTEGER): the integer value to search for in nums, satisfying -10^9 ≤ target ≤ 10^9.

Constraints

    0 <= nums.length <= 10^6
    -10^9 <= nums[i] <= 10^9 for each valid i
    For all 0 <= i < nums.length - 1, nums[i] < nums[i + 1] (strictly increasing order)
    -10^9 <= target <= 10^9

Output Format

Return a single INTEGER: the 0-based index of target in nums if it exists; otherwise return -1.
 * Complete the 'binarySearch' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY nums
 *  2. INTEGER target
 */

fn binarySearch(nums: &[i32], target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len();

    while left < right {
        let mid = left + (right - left) / 2;
        match nums[mid].cmp(&target) {
            std::cmp::Ordering::Less => todo!(),
            std::cmp::Ordering::Equal => todo!(),
            std::cmp::Ordering::Greater => todo!(),
        }
    }
    0
    // println!("Here is the array we are searching: {:?}", nums);
    // println!("    This is the target{target}");
    // // Write your code here
    // if nums.is_empty() {
    //     return -1;
    // }
    // let index = nums.len() / 2;
    // println!("    This is the index={index}");
    // let test_num = nums[index];
    // println!("    This is the test_num={test_num}");
    //
    // if test_num == target {
    //     return index as i32;
    // }
    //
    // if test_num > target {
    //     return binarySearch(&nums[0..index], target);
    // }
    //
    // if test_num < target {
    //     return binarySearch(&nums[index..nums.len()], target);
    // }
    //
    // return -1;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_target_in_middle() {
        assert_eq!(binarySearch(&[1, 2, 3, 4, 5], 3), 2);
    }

    #[test]
    fn test_target_at_start() {
        assert_eq!(binarySearch(&[1, 2, 3, 4, 5], 1), 0);
    }

    #[test]
    fn test_target_at_end() {
        assert_eq!(binarySearch(&[1, 2, 3, 4, 5], 5), 4);
    }

    #[test]
    fn test_not_found() {
        assert_eq!(binarySearch(&[1, 2, 3, 4, 5], 6), -1);
    }

    #[test]
    fn test_empty_slice() {
        assert_eq!(binarySearch(&[], 1), -1); // will panic - no base case
    }
}
