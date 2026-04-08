fn findFirstOccurrence(nums: &[i32], target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len();
    let mut result = -1;

    while left < right {
        let mid = left + (right - left) / 2;
        match nums[mid].cmp(&target) {
            std::cmp::Ordering::Equal => {
                result = mid as i32;
                right = mid;
            }
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid,
        }
    }
    result
}

fn binarySearch(nums: &[i32], target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len();

    while left < right {
        let mid = left + (right - left) / 2;
        match nums[mid].cmp(&target) {
            std::cmp::Ordering::Equal => return mid as i32,
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid,
        }
    }
    -1
}
