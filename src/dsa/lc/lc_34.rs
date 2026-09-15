/*
Given an array of integers nums sorted in non-decreasing order, find the starting and ending position of a given target value.

If target is not found in the array, return [-1, -1].

You must write an algorithm with O(log n) runtime complexity.



Example 1:

Input: nums = [5,7,7,8,8,10], target = 8
Output: [3,4]
Example 2:

Input: nums = [5,7,7,8,8,10], target = 6
Output: [-1,-1]
Example 3:

Input: nums = [], target = 0
Output: [-1,-1]


Constraints:

0 <= nums.length <= 105
-109 <= nums[i] <= 109
nums is a non-decreasing array.
-109 <= target <= 109
*/
pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.is_empty() {
        return vec![-1, -1];
    }

    let res = bs(&nums[..], target);

    if nums[res] != target {
        return vec![-1, -1];
    }

    if nums[nums.len() - 1] == target {
        return vec![res as i32, (nums.len() - 1) as i32]
    }

    let res2 = bs(&nums[..], target + 1);

    vec![res as i32, (res2 - 1) as i32]
}

fn bs(arr: &[i32], target: i32) -> usize {
    let mut lo = 0;
    let mut hi = arr.len() - 1;
    let mut mid;

    while lo < hi {
        mid = lo + ((hi - lo) / 2);
        if arr[mid] < target {
            lo = mid + 1
        } else {
            hi = mid
        }
    }
    hi
}
