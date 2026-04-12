fn qs<T: PartialOrd + Copy + std::fmt::Debug>(arr: &mut [T]) {
    if arr.len() < 2 {
        return;
    }
    let pivot = partition(arr);

    qs(&mut arr[..pivot]);
    println!("first half: {:?}", arr);
    qs(&mut arr[pivot + 1..]);
    println!("second half: {:?}", arr);
}

fn partition<T: PartialOrd + Copy + std::fmt::Debug>(arr: &mut [T]) -> usize {
    let hi = arr.len() - 1;
    let pivot = arr[hi]; // end point pivot
    let mut idx = 0;
    for i in 0..hi {
        if arr[i] <= pivot {
            arr.swap(i, idx);
            idx += 1;
        }
    }
    arr.swap(idx, hi);
    idx
}

#[cfg(test)]
mod tests {

    use std::time::Instant;

    use super::*;
    use crate::arrays::array_rand_get;

    #[test]
    fn quicksort() {
        let size = 10;
        let mut arr1 = array_rand_get(size);
        let mut arr2 = array_rand_get(size);

        qs(&mut arr1[..]);
        arr2.sort();
        assert_eq!(arr1, arr2);
    }
}
