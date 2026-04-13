pub fn encode(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::from("");
    }
    let mut s = String::new();

    for str in strs.iter() {
        let len = str.len() as u8;

        s.push_str(&format!("{len:04}{str}"));
    }
    println!("{s}");
    s
}

pub fn decode(s: String) -> Vec<String> {
    if s.is_empty() {
        return vec![];
    }

    let mut res = vec![];
    let bytes = s.as_bytes();
    let mut idx = 0;

    while idx < s.len() {
        let len_str = std::str::from_utf8(&bytes[idx..idx + 4]).unwrap();
        let len = len_str.parse::<u32>().unwrap() as usize;
        idx += 4;

        if len == 0 {
            res.push("".to_string());
            continue;
        }

        let str = &bytes[idx..idx + len];

        res.push(String::from(std::str::from_utf8(str).unwrap()));
        idx += len;

        println!("{res:?}");
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nc_encode_string_test() {
        let input = vec!["Hello".to_string(), "World".to_string()];
        let r = encode(input);
        let d = decode(r);
        let input = vec![
            "we".to_string(),
            "say".to_string(),
            ":".to_string(),
            "yes".to_string(),
            "!@#$%^&*()".to_string(),
        ];
        let r = encode(input);
        let d = decode(r);
    }
}
