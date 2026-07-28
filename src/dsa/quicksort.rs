fn qs<T: PartialOrd + Copy + std::fmt::Debug>(arr: &mut [T]) {
    if arr.len() < 2 {
        return;
    }
    let pivot = partition(arr);

    qs(&mut arr[..pivot]);
    qs(&mut arr[pivot + 1..]);
}

fn partition<T: PartialOrd + Copy + std::fmt::Debug>(arr: &mut [T]) -> usize {
    if arr.is_empty() {
        return 0;
    }

    let pivot_idx = median(arr);
    let pivot = arr[pivot_idx]; 
    let last = arr.len() - 1;

    arr.swap(pivot_idx, last); // swap pivot to the end
    
    let mut idx = 0;

    // iterate all items except the pivot
    for j in 0..last {
        if arr[j] <= pivot {
            arr.swap(j, idx);
            idx += 1;
        }
    }

    arr.swap(idx, last);
    idx
}

// calculates the median of three for the pivot, returns the idx
fn median<T: PartialOrd>(arr: &[T]) -> usize {
    let lo = 0;
    let mid = arr.len() / 2;
    let hi = arr.len() - 1;

    if arr[lo] < arr[mid] {
        if arr[mid] < arr[hi] {
            mid
        } else if arr[lo] < arr[hi] {
            hi
        } else {
            lo
        }
    } else {
        if arr[lo] < arr[hi] {
            lo
        } else if arr[mid] < arr[hi] {
            hi
        } else {
            mid
        }
    }
}

#[cfg(test)]
mod tests {

    use std::time::Instant;

    use super::*;
    use crate::dsa::arrays::rand_vec;

    #[test]
    fn quicksort() {
        let size = 10;
        let mut arr1 = rand_vec(size);
        let mut arr2 = arr1.clone();

        qs(&mut arr1[..]);
        arr2.sort();
        assert_eq!(arr1, arr2);
    }
}
