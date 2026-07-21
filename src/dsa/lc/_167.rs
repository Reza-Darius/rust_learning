/*
Given a 1-indexed array of integers numbers that is already sorted in non-decreasing order, find two numbers such that they add up to a specific target number. Let these two numbers be numbers[index1] and numbers[index2] where 1 <= index1 < index2 <= numbers.length.

Return the indices of the two numbers index1 and index2, each incremented by one, as an integer array [index1, index2] of length 2.

The tests are generated such that there is exactly one solution. You may not use the same element twice.

Your solution must use only constant extra space.



Example 1:

Input: numbers = [2,7,11,15], target = 9
Output: [1,2]
Explanation: The sum of 2 and 7 is 9. Therefore, index1 = 1, index2 = 2. We return [1, 2].
Example 2:

Input: numbers = [2,3,4], target = 6
Output: [1,3]
Explanation: The sum of 2 and 4 is 6. Therefore index1 = 1, index2 = 3. We return [1, 3].
Example 3:

Input: numbers = [-1,0], target = -1
Output: [1,2]
Explanation: The sum of -1 and 0 is -1. Therefore index1 = 1, index2 = 2. We return [1, 2].


Constraints:

2 <= numbers.length <= 3 * 104
-1000 <= numbers[i] <= 1000
numbers is sorted in non-decreasing order.
-1000 <= target <= 1000
The tests are generated such that there is exactly one solution.
 */

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut idx_hi = numbers.len() - 1;
    let mut idx_lo = 0;

    loop {
        let lo = numbers[idx_lo];
        let hi = numbers[idx_hi];

        if lo + hi == target {
            break;
        }

        if lo + hi > target {
            idx_hi -= 1;
            continue;
        }
        if lo + hi < target {
            idx_lo += 1;
            continue;
        }
    }

    vec![idx_lo as i32 + 1, idx_hi as i32 + 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_sum2() {
        let n = vec![2, 7, 11, 15];
        assert_eq!(two_sum(n, 9), [1, 2]);
        let n = vec![-3, 3, 4, 90];
        assert_eq!(two_sum(n, 0), [1, 2]);
        let n = vec![-5, -3, 0, 2, 4, 6, 8];
        assert_eq!(two_sum(n, 5), [2, 7]);
        let n = vec![
            12, 13, 23, 28, 43, 44, 59, 60, 61, 68, 70, 86, 88, 92, 124, 125, 136, 168, 173, 173,
            180, 199, 212, 221, 227, 230, 277, 282, 306, 314, 316, 321, 325, 328, 336, 337, 363,
            365, 368, 370, 370, 371, 375, 384, 387, 394, 400, 404, 414, 422, 422, 427, 430, 435,
            457, 493, 506, 527, 531, 538, 541, 546, 568, 583, 585, 587, 650, 652, 677, 691, 730,
            737, 740, 751, 755, 764, 778, 783, 785, 789, 794, 803, 809, 815, 847, 858, 863, 863,
            874, 887, 896, 916, 920, 926, 927, 930, 933, 957, 981, 997,
        ];
        // assert_eq(two_sum(n, 542), )
    }
}
