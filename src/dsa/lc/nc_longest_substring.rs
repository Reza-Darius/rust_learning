use std::cmp::max;

/*
Given a string s, find the length of the longest substring without duplicate characters.

A substring is a contiguous sequence of characters within a string.


Example 1:

Input: s = "zxyzxyz"

Input: = "abcbfg"

Output: 3
Explanation: The string "xyz" is the longest without duplicate characters.


Example 2:

Input: s = "xxxx"

Output: 1
*/
pub fn length_of_longest_substring(s: String) -> i32 {
    use std::collections::HashMap;

    if s.is_empty() {
        return 0;
    }
    let s = s.as_bytes();
    let mut letters: HashMap<u8, usize> = HashMap::new();

    let mut longest = 0;
    let mut sub;
    let mut lo = 0;
    let mut hi = 0;

    while hi < s.len() {
        let char = s[hi];

        if let Some(i) = letters.get(&char) {
            lo = max(lo, i + 1);
        }
        letters.insert(char, hi);

        sub = hi - lo + 1;
        longest = max(sub, longest);

        hi += 1;
    }
    longest as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn longest_substring() {
        assert_eq!(3, length_of_longest_substring("zxyzxyz".to_string()));
        assert_eq!(1, length_of_longest_substring("xxxx".to_string()));
        assert_eq!(2, length_of_longest_substring("abba".to_string()));
    }
}
