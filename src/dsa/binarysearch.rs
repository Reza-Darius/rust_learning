pub fn bs_recursive(target: u32, arr: &[u32]) -> Option<usize> {
    let end = match arr.len() {
        0 => return None,
        n => n,
    };
    if let Some(n) = bs(target, 0, end, arr) {
        return Some(n);
    }
    None
}
fn bs(target: u32, start: usize, end: usize, arr: &[u32]) -> Option<usize> {
    let mid = (start + end) / 2;
    if arr[mid] == target {
        return Some(mid);
    }
    if start >= end {
        return None;
    }
    if target > arr[mid] {
        return bs(target, mid + 1, end, arr);
    }
    if target < arr[mid] {
        return bs(target, start, mid - 1, arr);
    }
    None
}

fn bs_loop<T: Copy + Ord>(target: T, arr: &[T]) -> bool {
    let mut hi = arr.len();
    let mut lo = 0;
    while hi > lo {
        let m = (hi + lo) / 2;
        if arr[m] == target {
            return true;
        }
        if arr[m] > target { hi = m } else { lo = m + 1 }
    }
    false
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn bsearch() {
        let array = vec![2, 3, 6, 9, 15, 16, 19, 21, 24];
        assert_eq!(bs_recursive(9, &array).unwrap(), 3);
        assert_eq!(bs_recursive(21, &array).unwrap(), 7);
        assert_eq!(bs_recursive(24, &array).unwrap(), 8);
        assert!(bs_recursive(1, &array).is_none());

        assert!(bs_loop(3, &array));
        assert!(bs_loop(24, &array));
        assert!(!bs_loop(7, &array));
        assert!(!bs_loop(1, &array));
    }
}
