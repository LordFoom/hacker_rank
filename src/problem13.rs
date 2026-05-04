/**
 * Given an array of positive integers and a target integer, return the indices of two elements that sum to the target or [-1, -1] if no such pair exists.

*/
fn findTaskPairForSlot(taskDurations: &[i32], slotLength: i32) -> Vec<i32> {
    if taskDurations.is_empty() {
        return vec![-1, -1];
    }
    for i in 0..taskDurations.len() - 1 {
        for j in i + 1..taskDurations.len() {
            let num1 = taskDurations[i];
            let num2 = taskDurations[j];
            if num1 + num2 == slotLength {
                return vec![i as i32, j as i32];
            }
        }
    }

    vec![-1, -1]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn simple_test_find_early_vec() {
        let test_vec = vec![1, 2, 3, 4];
        let res = findTaskPairForSlot(&test_vec, 3);
        assert_eq!(vec![0, 1], res);
    }

    #[test]
    pub fn simple_test_empty() {
        let res = findTaskPairForSlot(&vec![], 4);
        assert_eq!(vec![-1, -1], res);
    }

    #[test]
    pub fn simple_test_end_vec() {
        let test_vec = vec![1, 2, 3, 4, 5, 6, 100];
        let res = findTaskPairForSlot(&test_vec, 106);
        assert_eq!(vec![5, 6], res);
    }

    #[test]
    pub fn simple_test_start_end_vec() {
        let test_vec = vec![1, 2, 3, 4, 5, 6, 100];
        let res = findTaskPairForSlot(&test_vec, 101);
        assert_eq!(vec![0, 6], res);
    }
}
