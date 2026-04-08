/*!
mod problem7;
Constraints

    0 <= meetings.length <= 1000
    meetings[i].length == 2 for all 0 <= i < meetings.length
    0 <= meetings[i][0] < meetings[i][1] <= 10^9
    All start and end times are integers
    Meetings may share start or end times

Output Format

    Return a single integer denoting the maximum number of non-overlapping meetings that can be scheduled.
*/

/*
 * Complete the 'maximizeNonOverlappingMeetings' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY meetings as parameter.
 */

fn maximizeNonOverlappingMeetings(meetings: &[Vec<i32>]) -> i32 {
    let mut time_vec = meetings.to_vec();
    time_vec.sort_by_key(|v| v.last().copied());

    let mut count = 0;
    let mut last_end_time = -1;

    for v in time_vec {
        //guaranteed to be appropriately formed tuple
        let (i, j) = validate(v);

        if i >= last_end_time {
            count += 1;
            last_end_time = j;
        }
    }

    count
}

/*
 * Return the time tuple, panicking if it cannot
 */
fn validate(v: Vec<i32>) -> (i32, i32) {
    if v.len() != 2 {
        panic!("Expected tuple of size 2, got one with size: {}", v.len());
    }
    let (i, j) = (v.first().unwrap_or(&-1), v.last().unwrap_or(&-1));
    if i == &-1 {
        panic!("No first element?");
    }
    if j == &-1 {
        panic!("No second element?");
    }
    (*i, *j)
}
