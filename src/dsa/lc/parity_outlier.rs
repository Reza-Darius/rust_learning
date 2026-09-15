// [2, 4, 0, 100, 4, 11, 2602, 36] -->  11 (the only odd number)
//
// [160, 3, 1719, 19, 11, 13, -21] --> 160 (the only even number)
fn parity_outlier(arr: &[i32]) -> i32 {
    // look at the first 3 elements to determing arr type
    let sum: i32 = arr.iter().take(3).sum::<i32>() % 2;
    let arr_even = if sum == 1 {
        // if the sum % 2 = 1 we have two cases
        // either all numbers are odd or 2 even 1 odd
        arr.iter().take(3).any(|x| *x % 2 == 0)
    } else {
        // if the sum % 2 = 0 we have two cases:
        // either all numbers are even or 2 odd + 1 even
        !arr.iter().take(3).any(|x| *x % 2 == 1)
    };

    if arr_even {
        // find odd number in even array
        *arr.iter().find(|&x| *x % 2 != 0).unwrap()
    } else {
        // find even number in odd array
        *arr.iter().find(|&x| *x % 2 == 0).unwrap()
    }
}

#[cfg(test)]
mod test {
    use std::{
        ops::Add,
        time::{Duration, Instant},
    };

    use super::*;
    use rand::*;

    #[test]
    fn parity_outlier_test() {
        const SIZE: usize = 10_000_000;
        const ITERATONS: usize = 10000;

        let mut even_arr: Vec<i32> = vec![0; SIZE];
        even_arr.fill_with(|| random_range(0..10_000) & !1);

        let mut time = Duration::default();
        for _ in 0..ITERATONS {
            let idx = random_range(0..even_arr.len() - 1);
            let needle = random_range(0..10_000) | 1;
            even_arr[idx] = needle;

            let now = Instant::now();
            parity_outlier(&even_arr);
            time += now.elapsed();
            even_arr[idx] = random_range(0..10_000) & !1;
        }
        println!("took: {:?}", time.div_f32(ITERATONS as f32));
    }
}
