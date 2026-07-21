/*
Given an integer array nums and an integer k, return the k most frequent elements within the array.

The test cases are generated such that the answer is always unique.

You may return the output in any order.

Example 1:

Input: nums = [1,2,2,3,3,3], k = 2

Output: [2,3]
Example 2:

Input: nums = [7,7], k = 1

Output: [7]
Constraints:

1 <= nums.length <= 10^4.
-1000 <= nums[i] <= 1000
1 <= k <= number of distinct elements in nums.
 */
pub fn top_k_frequent(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::BinaryHeap;

    nums.sort();
    let mut heap: BinaryHeap<(i32, i32)> = BinaryHeap::new();

    let mut curr = nums[0];
    let mut count = 1;

    for n in nums.iter().skip(1) {
        println!("{n}");
        if *n != curr {
            println!("swapping");
            heap.push((count, curr));
            curr = *n;
            count = 1;
        } else {
            count += 1;
        }
    }

    // pushing the last element
    heap.push((count, curr));

    println!("heap {heap:?}");
    let mut res = vec![];

    for n in 0..k as usize {
        res.push(heap.pop().unwrap().1)
    }
    res
}
