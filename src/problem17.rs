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
    if intervals.is_empty() {
        return vec![];
    }
    let mut clone_intervals = Vec::from(intervals);
    clone_intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    merge_intervals(&clone_intervals)
}

fn merge_intervals(sorted_intervals: &[Vec<i32>]) -> Vec<Vec<i32>> {
    //used to create merged intervals
    let mut current_start = 0;
    let mut current_end = 0;
    let mut first = true;
    let mut merged_intervals = Vec::new();
    for interval in sorted_intervals {
        let start_interval = interval[0];
        let end_interval = interval[1];

        //first interval initiates the vars
        if first {
            current_start = start_interval;
            current_end = end_interval;
            first = false;
            continue;
        }
        // intervals = [[1, 3], [2, 6], [8, 10], [15, 18]]
        // Output
        // [[1, 6], [8, 10], [15, 18]]

        //if the start overlaps, and end is gt curent end, we move our second pointer out
        if (start_interval >= current_start && start_interval <= current_end)
            && end_interval > current_end
        {
            current_end = end_interval;
        }
        if start_interval > current_end {
            //if we are past the current edge, let's store the possibly merged interval
            let itvl = vec![current_start, current_end];
            merged_intervals.push(itvl);
            //now we have new current_start and _end
            current_start = start_interval;
            current_end = end_interval;
        }
    }
    //keep an eye out for sneaky off-by-one
    let final_itvl = vec![current_start, current_end];
    merged_intervals.push(final_itvl);
    merged_intervals
}

#[cfg(test)]
mod test {
    use super::*;

    // intervals = [[1, 3], [2, 6], [8, 10], [15, 18]]
    // Output
    // [[1, 6], [8, 10], [15, 18]]
    #[test]
    pub fn test_mergeHighDefinitionIntervals() {
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 8]];

        let processed_intervals = mergeHighDefinitionIntervals(&intervals);
        println!("processed_intervals={:?}", processed_intervals);
    }
}
