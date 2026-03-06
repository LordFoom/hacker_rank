use std::{
    io::{self, BufRead},
    ops::Div,
}; /*
 * Complete the 'countResponseTimeRegressions' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY responseTimes as parameter.
 */

fn countResponseTimeRegressions(responseTimes: &[i32]) -> i32 {
    let mut count = 0;
    let mut previous_resp_times = Vec::new();

    for (i, response_time) in responseTimes.iter().enumerate() {
        if i == 0 {
            previous_resp_times.push(response_time.to_owned());
            continue;
        }
        let avg = average(&previous_resp_times);
        if *response_time as i64 > avg {
            count += 1;
        }
        previous_resp_times.push(response_time.to_owned());
    }

    return count;
}

//how would i memoize this...perhaps need a pair? or...a hashmap?
// static mut avg_memoized: (i32, i32) = (0, 0);
static mut AVG_MEMOIZED: (i32, i32) = (0, 0);
//keep the total running
static mut RUNNING_TOTAL: i32 = 0;
///Return avg of the array
fn average(previous_resp_times: &Vec<i32>) -> i64 {
    if previous_resp_times.is_empty() {
        return -1;
    };
    let sz = previous_resp_times.len();
    if sz == 1 {
        return previous_resp_times.first().unwrap().to_owned() as i64;
    }

    previous_resp_times
        .iter()
        .fold(0i64, |acc, rt| acc + *rt as i64)
        .div(sz as i64)
}

#[cfg(test)]
mod test {
    use crate::problem1::{average, countResponseTimeRegressions};

    #[test]
    pub fn test_avg_empty() {
        let test_vec_empty = vec![];
        let test_empty = average(&test_vec_empty);
        assert_eq!(-1, test_empty, "Expect to receive -1 for empty vec")
    }

    #[test]
    pub fn test_avg_one() {
        let test_vec_one = vec![250];
        let test_one = average(&test_vec_one);
        assert_eq!(
            250, test_one,
            "Expect to receive single item for single item vec"
        )
    }

    #[test]
    pub fn test_avg_three() {
        let test_vec_three = vec![250, 100, 105];
        let test_three = average(&test_vec_three);
        assert_eq!(151, test_three, "Expected 151");
    }

    #[test]
    pub fn test_response_time_regressions() {
        let response_times = vec![100, 200, 150, 300];
        let count = countResponseTimeRegressions(&response_times);
        assert_eq!(2, count, "Expected count to be 2");
    }
}
