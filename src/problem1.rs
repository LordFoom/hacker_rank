use std::{
    io::{self, BufRead},
    ops::Div,
};

/*
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
        if response_time > &avg {
            count += 1;
        }
        previous_resp_times.push(response_time.to_owned());
    }

    return count;
}

//how would i memoize this...perhaps need a pair?
static mut avg_memoized: (i32, i32) = (0, 0);
///Return avg of the array
fn average(previous_resp_times: &Vec<i32>) -> i32 {
    if previous_resp_times.is_empty() {
        return -1;
    };
    let sz = previous_resp_times.len();
    if sz == 1 {
        return previous_resp_times.first().unwrap().to_owned();
    }

    previous_resp_times
        .iter()
        .fold(0, |acc, rt| acc + rt)
        .div(sz as i32)
}

#[cfg(test)]
mod test {
    use crate::problem1::average;

    #[test]
    pub fn test_average() {
        let test_vec_one = vec![];
        let test_one = average(&test_vec_one);
        assert_eq!(-1, test_one, "Expect to receive -1 for empty vec")
    }
}
