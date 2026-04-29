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
fn debounceTimestamps(arr: &[i32], K: i32) -> i32 {
    let mut timestamps = arr.to_vec();

    if timestamps.is_empty() {
        return 0;
    }

    let mut j = 1; // Position to write next kept element
    let mut last_kept = timestamps[0]; // The value of the last kept element

    for i in 1..timestamps.len() {
        if timestamps[i] - last_kept >= K {
            // Keep this element
            timestamps[j] = timestamps[i];
            last_kept = timestamps[i];
            j += 1;
        }
        // Otherwise skip (don't increment j)
    }

    j as i32
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        // Empty array should return 0, not panic
        let timestamps: Vec<i32> = vec![];
        assert_eq!(debounceTimestamps(&timestamps, 1), 0);
    }

    #[test]
    fn test_basic_debounce() {
        // Timestamps: [1, 2, 3, 5, 10], K=2
        // Valid: 1 (start), 3 (3-1=2 >= 2), 5 (5-3=2 >= 2), 10 (10-5=5 >= 2)
        // Result: 3 additional timestamps (3, 5, 10)
        let timestamps = vec![1, 2, 3, 5, 10];
        assert_eq!(debounceTimestamps(&timestamps, 2), 3);
    }

    #[test]
    fn test_no_valid_timestamps() {
        // Timestamps: [1, 2, 3], K=5
        // All differences are < 5, so nothing gets added
        let timestamps = vec![1, 2, 3];
        assert_eq!(debounceTimestamps(&timestamps, 5), 0);
    }

    #[test]
    fn test_all_timestamps_valid() {
        // Timestamps: [1, 6, 11, 16], K=5
        // All differences are exactly 5 or more
        let timestamps = vec![1, 6, 11, 16];
        assert_eq!(debounceTimestamps(&timestamps, 5), 3);
    }

    #[test]
    fn test_single_timestamp() {
        // Only one timestamp, nothing to debounce
        let timestamps = vec![5];
        assert_eq!(debounceTimestamps(&timestamps, 1), 0);
    }

    #[test]
    fn test_exactly_k_apart() {
        // Timestamps: [0, 5, 10, 15], K=5
        // Each is exactly 5 apart
        let timestamps = vec![0, 5, 10, 15];
        assert_eq!(debounceTimestamps(&timestamps, 5), 3);
    }

    #[test]
    fn test_just_below_threshold() {
        // Timestamps: [0, 4, 9, 13], K=5
        // 4-0=4 (< 5, skip), 9-0=9 (>= 5, add), 13-9=4 (< 5, skip)
        let timestamps = vec![0, 4, 9, 13];
        assert_eq!(debounceTimestamps(&timestamps, 5), 1);
    }

    #[test]
    fn test_large_k_value() {
        // Timestamps: [1, 2, 3, 100], K=50
        // Only 100 is far enough from 1
        let timestamps = vec![1, 2, 3, 100];
        assert_eq!(debounceTimestamps(&timestamps, 50), 1);
    }

    #[test]
    fn test_negative_timestamps() {
        // Timestamps: [-10, -5, 0, 10], K=5
        // -5-(-10)=5, 0-(-5)=5, 10-0=10
        let timestamps = vec![-10, -5, 0, 10];
        assert_eq!(debounceTimestamps(&timestamps, 5), 3);
    }

    #[test]
    fn test_unsorted_timestamps() {
        // Note: This function assumes sorted timestamps
        // Timestamps: [1, 3, 2, 5], K=2
        // 3-1=2 (add 3), 2-3=-1 (skip, negative), 5-3=2 (add 5)
        let timestamps = vec![1, 3, 2, 5];
        assert_eq!(debounceTimestamps(&timestamps, 2), 2);
    }

    #[test]
    fn test_two_timestamps_valid() {
        // Timestamps: [0, 10], K=5
        // 10-0=10 >= 5, so add 10
        let timestamps = vec![0, 10];
        assert_eq!(debounceTimestamps(&timestamps, 5), 1);
    }

    #[test]
    fn test_two_timestamps_invalid() {
        // Timestamps: [0, 3], K=5
        // 3-0=3 < 5, so nothing added
        let timestamps = vec![0, 3];
        assert_eq!(debounceTimestamps(&timestamps, 5), 0);
    }

    #[test]
    fn test_realistic_scenario() {
        // Button clicks with debounce threshold of 100ms
        // Timestamps: [0, 50, 100, 200, 250, 350], K=100
        let timestamps = vec![0, 50, 100, 200, 250, 350];
        // 50-0=50 (skip), 100-0=100 (add), 200-100=100 (add),
        // 250-200=50 (skip), 350-200=150 (add)
        assert_eq!(debounceTimestamps(&timestamps, 100), 3);
    }

    #[test]
    fn test_k_zero() {
        // Timestamps: [1, 2, 3, 4], K=0
        // All timestamps after the first should be included (all differences >= 0)
        let timestamps = vec![1, 2, 3, 4];
        assert_eq!(debounceTimestamps(&timestamps, 0), 3);
    }

    #[test]
    fn test_k_one() {
        // Timestamps: [1, 2, 3, 4], K=1
        // All consecutive timestamps have difference >= 1
        let timestamps = vec![1, 2, 3, 4];
        assert_eq!(debounceTimestamps(&timestamps, 2), 3);
    }
}
