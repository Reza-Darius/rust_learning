// You are given a 0-indexed integer array nums and a positive integer k.
//
// You can do the following operation on the array any number of times:
//
// Choose any two distinct indices i and j and simultaneously update the values of nums[i] to (nums[i] AND nums[j]) and nums[j] to (nums[i] OR nums[j]). Here, OR denotes the bitwise OR operation, and AND denotes the bitwise AND operation.
// You have to choose k elements from the final array and calculate the sum of their squares.
//
// Return the maximum sum of squares you can achieve.
//
// Since the answer can be very large, return it modulo 109 + 7.
pub fn max_sum(mut nums: Vec<i32>, k: i32) -> i32 {
    const MOD: u64 = 10u64.pow(9) + 7;

    // bucket counting all bits at a given position for 32 bit integer
    // idx 0 = least siginifacnt bit
    let mut buckets: [u32; 32] = [0; _];
    fill_bucket(&mut buckets, &nums);

    // println!("buckets: {buckets:?}");
    nums.sort();

    let mut res = 0;
    for n in 0..k {
        let num = get_num(&mut buckets);
        res += (num as u64 * num as u64) % MOD;
    }

    (res % MOD) as i32
}

fn fill_bucket(bucket: &mut [u32; 32], nums: &[i32]) {
    for &n in nums {
        for (idx, b) in bucket.iter_mut().enumerate() {
            // println!("b {b}, idx {idx}, n {n}");
            *b += (n as u32 & (1 << idx)) >> idx;
        }
    }
}

fn get_num(bucket: &mut [u32; 32]) -> i32 {
    let mut num = 0;
    for (idx, b) in bucket.iter_mut().enumerate() {
        if *b == 0 {
            continue;
        }
        // println!("b {b}, idx {idx}, n {n}");
        num |= 1 << idx;
        *b -= 1;
    }
    num
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn max_sum_test() {
        let nums = [2, 6, 5, 8].to_vec();
        assert_eq!(max_sum(nums, 2), 261);
        let nums = [4, 5, 4, 7].to_vec();
        assert_eq!(max_sum(nums, 3), 90);
        let nums = [25, 52, 75, 65].to_vec();
        assert_eq!(max_sum(nums, 4), 24051);
    }
}
