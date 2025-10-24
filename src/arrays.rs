
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
        if array[i] == target {return Some(array[i])}
    }
    None
}


#[allow(dead_code)]
pub fn bubble_sort(arr: &mut Vec<u32>) {
    let max = arr.len();
    if max == 0 {
        return
    }
    for j in 0..max {
        for i in 0..max - 1 - j {
            if arr[i] < arr[i+1] {
                arr.swap(i, i+1);
            }
        }
    }
    
}


#[cfg(test)]
mod tests {

    use rand::prelude::*;
    use std::time::{Instant};
    use crate::stack::*;

    use super::*;

    #[test]
    fn linearsearch() {
        let array = array_rand_get(100000);
        let target = rand::random_range(1..100) as u32;

        let now = Instant::now();
        let result = linear_search(target, &array);
        println!("linear search found {} in: {:?} ms", result.unwrap(), now.elapsed().as_millis())

    }
    
}
