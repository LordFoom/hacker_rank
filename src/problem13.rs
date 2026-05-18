/**
 * Given an array of positive integers and a target integer, return the indices of two elements that sum to the target or [-1, -1] if no such pair exists.

*/
fn findTaskPairForSlot(taskDurations: &[i32], slotLength: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut seen: HashMap<i32, usize> = HashMap::new();
    for (j, &num) in taskDurations.iter().enumerate() {
        let complement = slotLength - num;
        if let Some(&i) = seen.get(&complement) {
            return vec![i as i32, j as i32];
        }
        seen.insert(num, j);
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
