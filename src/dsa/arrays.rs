use rand::prelude::*;

#[allow(dead_code)]
pub(crate) fn array_rand_get(n: u32) -> Vec<u32> {
    let mut array: Vec<u32> = (1..=n).collect();
    array.shuffle(&mut rand::rng());
    array
}
#[allow(dead_code)]
fn linear_search(target: u32, array: &Vec<u32>) -> Option<u32> {
    for i in 0..array.len() {
        if array[i] == target {
            return Some(array[i]);
        }
    }
    None
}

#[allow(dead_code)]
pub fn bubble_sort(arr: &mut [u32]) {
    let mut max = arr.len();
    while max > 0 {
        for i in 0..max - 1 {
            if arr[i] < arr[i + 1] {
                arr.swap(i, i + 1);
            }
        }
        max -= 1;
    }
}

#[cfg(test)]
mod tests {

    use std::time::Instant;

    use super::*;

    #[test]
    fn linearsearch() {
        let array = array_rand_get(100000);
        let target = rand::random_range(1..100) as u32;

        let now = Instant::now();
        let result = linear_search(target, &array);
        println!(
            "linear search found {} in: {:?} ms",
            result.unwrap(),
            now.elapsed().as_millis()
        )
    }
    #[test]
    fn bubblesort() {
        let mut arr = array_rand_get(100);
        let mut arr2 = array_rand_get(100);

        bubble_sort(&mut arr);
        arr2.sort();

        assert_ne!(arr, arr2);
    }
}
