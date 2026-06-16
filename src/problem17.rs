/*
 * Merge and Sort Intervals

Given an array of intervals [startTime, endTime], merge all overlapping intervals and return a sorted array of non-overlapping intervals.

Example

Input

intervals = [[1, 3], [2, 6], [8, 10], [15, 18]]

Output

[[1, 6], [8, 10], [15, 18]]

Explanation

- Step 1: Sort intervals by start time (already sorted).
- Step 2: Initialize merged list with first interval [1,3].
- Step 3: Compare [2,6] with last merged [1,3]. They overlap (2 ≤ 3), so merge into [1,6]. Step 4: Compare [8,10] with last merged [1,6]. No overlap (8 > 6), append [8,10].
- Step 5: Compare [15,18] with last merged [8,10]. No overlap (15 > 10), append [15,18].

Result: [[1,6],[8,10],[15,18]].

 * Complete the 'mergeHighDefinitionIntervals' function below.
 *
 * The function is expected to return a 2D_INTEGER_ARRAY.
 * The function accepts 2D_INTEGER_ARRAY intervals as parameter.
 */

fn mergeHighDefinitionIntervals(intervals: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut clone_intervals = Vec::from(intervals);
    clone_intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    merge_intervals(&clone_intervals)
}

fn merge_intervals(sorted_intervals: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut current_min = 0;
    let mut current_end = 0;
    let mut merged_intervals = Vec::new();
    for interval in sorted_intervals {
        let start_interval = interval[0];
        let end_interval = interval[1];
    }
    merged_intervals
}
