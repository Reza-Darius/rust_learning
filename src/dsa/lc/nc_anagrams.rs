/*
given an array of strings strs, group all anagrams together into sublists. you may return the output in any order.

an anagram is a string that contains the exact same characters as another string, but the order of the characters can be different.

example 1:

input: strs = ["act","pots","tops","cat","stop","hat"]

output: [["hat"],["act", "cat"],["stop", "pots", "tops"]]
example 2:

input: strs = ["x"]

output: [["x"]]
example 3:

input: strs = [""]

output: [[""]]
constraints:

1 <= strs.length <= 1000.
0 <= strs[i].length <= 100
strs[i] is made up of lowercase english letters.
 */

#![allow(unused_mut, clippy::needless_return)]

use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<[u8; 26], Vec<String>> = HashMap::new();

    let str_num = |sub_str: &str| {
        let mut k = [0u8; 26];
        for char in sub_str.as_bytes().iter() {
            k[(char - b'a') as usize] += 1;
        }
        k
    };

    for str in strs.into_iter() {
        let key = str_num(&str);
        map.entry(key).or_default().push(str);
    }

    let mut res: Vec<_> = map.values().cloned().collect();

    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nc150() {
        let s = "act";
        let b = "cat";
        let str_num = |sub_str: &str| {
            let mut k = [0u8; 26];
            for char in sub_str.as_bytes().iter() {
                k[(char - b'a') as usize] += 1;
            }
            k
        };
        assert_eq!(str_num(s), str_num(b));

        let a = "duh";
        let b = "ill";
        assert_ne!(str_num(a), str_num(b))
    }
}
