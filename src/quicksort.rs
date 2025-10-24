#[allow(dead_code)]
pub fn quicksort_recursive<T: PartialOrd + Copy>(array: &mut Vec<T>) {
    let hi = array.len();
    if hi > 0 {
        qs(array, 0 , hi - 1);
    }
}

fn qs<T: PartialOrd + Copy>(array: &mut Vec<T>, lo: usize, hi: usize) {
    if lo >= hi {
        return
    }
    let pivotidx = partition(array, lo, hi);
    if pivotidx > 0 {
        qs(array, lo, pivotidx - 1);
    }
    qs(array, pivotidx + 1, hi);
}

fn partition<T: PartialOrd + Copy>(array: &mut Vec<T>, lo: usize, hi: usize) -> usize { 
    let pivot = array[hi];
    let mut idx = lo;
    for i in lo..hi {
        if array[i] <= pivot {
            array.swap(i, idx);
            idx += 1;
        }
    }
    array.swap(idx, hi);
    idx
}

#[cfg(test)]
mod tests {

    use rand::prelude::*;
    use std::time::{Instant};
    use crate::stack::*;

    use super::*;
    use crate::arrays::array_rand_get;

    #[test]
    fn quicksort() {
        let size = 100;
        let mut array = array_rand_get(size);
        let target = rand::random_range(1..=size) as u32;

        let mut check_arr = Vec::with_capacity(size as usize);
        check_arr = (1..=size).collect();

        quicksort_recursive(&mut array);
        assert_eq!(array, check_arr);
    }

}