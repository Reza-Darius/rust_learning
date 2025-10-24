#[allow(dead_code)]
pub fn binary_search(target: u32, arr: &Vec<u32>) -> Option<usize> {
    let end = match arr.len() {
        0 => return None,
        n => n,
    };
    if let Some(n) = bs(target, 0, end, &arr){
        return Some(n);
    }
    None
}
fn bs(target: u32, start: usize, end: usize, arr: &[u32]) -> Option<usize> {
    let mid = (start + end) / 2;
    if arr[mid] == target {
        return Some(mid)
    }
    if start >= end {
        return None
    }
    if target > arr[mid] {
        return bs(target, mid + 1, end, arr)
    }
    if target < arr[mid] {
        return bs(target, start , mid - 1, arr)
    }
    None
}

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn bsearch() {
        let array = vec![2,3,6,9,15,16,19,21,24];
        assert_eq!(binary_search(9, &array).unwrap(), 3);
        assert_eq!(binary_search(21, &array).unwrap(), 7);
        assert_eq!(binary_search(24, &array).unwrap(), 8);
        assert!(binary_search(1, &array) == None);
    }

    
}
