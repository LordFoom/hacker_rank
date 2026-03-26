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
    // Write your code here
    let index = nums.len() / 2;
    let test_num = nums[index];

    if test_num == target {
        return index;
    }

    if test_num < target {
        return binarySearch(nums[0..index], target);
    }

    if test_num > target {
        return binarySearch(nums[index..nums.len()], target);
    }

    return -1;
}

