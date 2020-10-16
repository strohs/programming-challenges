/// # 27 Remove Element
/// Given an array nums and a value val, remove all instances of that value in-place and return the
/// new length.
/// Do not allocate extra space for another array, you must do this by modifying the input array
/// in-place with O(1) extra memory.
/// The order of elements can be changed. It doesn't matter what you leave beyond the new length.
///
/// ## Example 1
/// Given nums = `[3,2,2,3], val = 3`,
/// our function should return length = 2, with the first two elements of nums being 2.
/// It doesn't matter what you leave beyond the returned length.
///
/// ## Example 2
/// Given nums = `[0,1,2,2,3,0,4,2], val = 2`,
/// Your function should return length = 5, with the first five elements of nums containing
/// 0, 1, 3, 0, and 4.
/// Note that the order of those five elements can be arbitrary.

/**
 * @param {number[]} nums
 * @param {number} val
 * @return {number}
 */
const removeElement = function(nums, val) {
    let length = 0;

    for (let i = 0; i < nums.length; i++) {
        if (nums[i] === val) {
            // find index of first element != val and swap it with nums[i]
            const fi = nums.slice(i).findIndex(e => e !== val);
            if (fi > -1) {
                let t = nums[i];
                nums[i] = nums[i + fi];
                nums[i + fi] = t;
            } else {
                // all remaining elements in nums are === val, so return
                return length;
            }
        }
        length++;
    }
    return length;
}

let v = [0,1,2,2,3,0,4,2];
let l = removeElement(v, 2);
console.log(l, v);

let v2 = [3,2,2,3];
let l2 = removeElement(v2, 3);
console.log(l2, v2);