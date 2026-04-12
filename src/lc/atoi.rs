#![allow(unused_mut, unused_assignments)]

pub fn my_atoi(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }

    let mut res: u64 = 0;
    let mut pos = true;

    let mut slice = s.as_str().trim().as_bytes();

    if slice.is_empty() {
        return 0;
    }

    match slice[0] {
        b'+' => slice = &slice[1..],
        b'-' => {
            pos = false;
            slice = &slice[1..];
        }
        _ => (),
    }

    while !slice.is_empty() {
        match slice[0] {
            b'0' => slice = &slice[1..],
            _ => break,
        }
    }

    if slice.is_empty() {
        return 0;
    }

    while !slice.is_empty() {
        match slice[0] {
            b if b.is_ascii_digit() => {
                res = res * 10 + (b - b'0') as u64;

                if !pos && res > i32::MAX as u64 {
                    return i32::MIN;
                }
                if pos && res >= i32::MAX as u64 {
                    return i32::MAX;
                }
                slice = &slice[1..];
            }
            _ => break,
        }
    }
    let mut res = res as i32;
    if !pos {
        res = -res;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atoi() {
        let s = " -0000042".to_string();
        assert_eq!(my_atoi(s), -42);
        let s = "00001337c0d3".to_string();
        assert_eq!(my_atoi(s), 1337);
        let s = "0-1".to_string();
        assert_eq!(my_atoi(s), 0);
        let s = "21474836460".to_string();
        assert_eq!(my_atoi(s), i32::MAX);
        let s =
            "10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000522545459".to_string();
        assert_eq!(my_atoi(s), i32::MAX);
        let s = "0  123".to_string();
        assert_eq!(my_atoi(s), 0);
        let s = "123-".to_string();
        assert_eq!(my_atoi(s), 123);
        let s = "-2147483647".to_string();
        assert_eq!(my_atoi(s), -2147483647);
    }
}
