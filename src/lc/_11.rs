/*
You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).

Find two lines that together with the x-axis form a container, such that the container contains the most water.

Return the maximum amount of water a container can store.

Notice that you may not slant the container.





 */
pub fn max_area(height: Vec<i32>) -> i32 {
    let cap = |lo: usize, hi: usize| -> i32 {
        let x = hi - lo;
        height[lo].min(height[hi]) * x as i32
    };

    let mut res = 0;
    let mut lo = 0;
    let mut hi = height.len() - 1;

    while hi > lo {
        let c = cap(lo, hi);
        println!("{c}");

        if c > res {
            res = c;
        }
        if height[lo] < height[hi] {
            lo += 1
        } else {
            hi -= 1
        }
    }
    res
}
