/// Leetcode 33 - search rotated sorted array
///
/// There is an integer array `nums` sorted in ascending order (with distinct values).
/// Prior to being passed to your function, `nums` is rotated at an unknown pivot index
/// `k (0 <= k < nums.length)` such that the resulting array is
/// `[nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed)`.
/// For example, `[0,1,2,4,5,6,7]` might be rotated at pivot index 3 and become `[4,5,6,7,0,1,2]`.
///
/// Given the array `nums` after the rotation and an integer target, return the index of `target`
/// if it is in `nums`, or `-1` if it is not in `nums`.


// do while nums slice is not empty
// check if nums first < nums last, if so, nums is sorted and binary search nums for target
// else
//   get midpoint, M, of nums
//   determine which half of nums is sorted
//   if target in range of sorted half, bin-search it and return
//   else target in un-sorted half
//     move mid point +/- one
//     and restart search in the new sublice

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    fn is_sorted(slice: &[i32], start: usize, end: usize) -> bool {
        slice[start] <= slice[end]
    }

    fn is_within(slice: &[i32], start: usize, end: usize, target: i32) -> bool {
        target >= slice[start] && target <= slice[end]
    }

    // the first index of a the
    let mut start_idx = 0;
    let mut end_idx = nums.len() - 1;

    while !nums[start_idx..=end_idx].is_empty() {

        if is_sorted(&nums, start_idx, end_idx) {
            // binary-search for target in slice and return result
            return match nums[start_idx..=end_idx].binary_search(&target) {
                Ok(idx) => (start_idx + idx) as i32,
                _ => -1
            }
        } else {
            // middle index
            let mi = (end_idx + start_idx) / 2;

            // check left side
            if is_sorted(&nums, start_idx, mi) {
                if is_within(&nums, start_idx, mi, target) {
                    end_idx = mi;
                } else {
                    start_idx = mi + 1;
                }
            } else {
                // else right side must be sorted
                if is_within(&nums, mi+1, end_idx, target) {
                    start_idx = mi + 1;
                } else {
                    end_idx = mi;
                }
            }
        }
    }
    -1
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::search;

    #[test]
    fn example0() {
        let nums = vec![1];
        assert_eq!(search(nums, 1), 0);
    }

    #[test]
    fn example1() {
        let nums = vec![4,5,6,7,0,1,2];
        assert_eq!(search(nums, 0), 4);
    }

    #[test]
    fn example2() {
        let nums = vec![4,5,6,7,0,1,2];
        assert_eq!(search(nums, 3), -1);
    }

    #[test]
    fn example3() {
        let nums = vec![1];
        assert_eq!(search(nums, 0), -1);
    }

    #[test]
    fn example4() {
        let nums = vec![3,4,5,6,1,2];
        assert_eq!(search(nums, 3), 0);
    }

    #[test]
    fn example5() {
        let nums = vec![3,4,5,6,1,2];
        assert_eq!(search(nums, 2), 5);
    }

    #[test]
    fn example6() {
        let nums = vec![3,4,5,6,1,2];
        assert_eq!(search(nums, 6), 3);
    }

}