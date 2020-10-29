/// # 26. Remove duplicates from a Sorted Array
/// Given a sorted array nums, remove the duplicates in-place such that each element appears only
/// once and **returns the new length.**
/// Do not allocate extra space for another array, you must do this by modifying the input array
/// in-place with *O(1)* extra memory.
///
/// ## Example 1
/// ```
/// Given nums = [1,1,2],
/// Your function should return length = 2, with the first two elements of nums being 1 and 2
/// respectively. It doesn't matter what you leave beyond the returned length.
/// ```
///
/// ## Example 2
/// ```
/// Given nums = [0,0,1,1,1,2,2,3,3,4]
/// Your function should return length = 5, with the first five elements of nums being modified
/// to 0,1,2,3 and 4 respectively. It doesn't matter what values are set beyond the returned
/// length.
/// ```


/// returns the length of nums once all duplicates have been removed. Nums will be modified in-place
/// so that the non repeating elements appear at the beginning of the array, making sure to keep
/// their sorted order
function remove_duplicates(nums) {
  let numsIdx = 0;
  let startIdx = numsIdx;

  while (numsIdx < nums.length && startIdx < nums.length) {
    let peekIdx = startIdx;

    while (peekIdx < nums.length && nums[peekIdx] === nums[startIdx]) {
      peekIdx++;
    }

    startIdx = (peekIdx - startIdx > 1) ? peekIdx : startIdx + 1;

    nums[numsIdx] = nums[peekIdx - 1];
    numsIdx++;
  }
  return numsIdx;
}

let nums = [0,0,1,1,2,2,3,3,3,4];

let l = remove_duplicates(nums);
console.log(l, nums);