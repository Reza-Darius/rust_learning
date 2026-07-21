// #[cfg(test)]
// mod meme_test {
//     #[derive(Debug)]
//     struct Pair {
//         left: String,
//         right: String,
//     }

//     impl Pair {
//         fn foo(&mut self) {
//             let left = &mut self.left;
//             left.push_str("hi");
//             self.bar();
//             println!("{left}");
//         }
//         fn bar(&self) {
//             println!("{self:?}");
//         }
//     }

//     struct Foo {
//         s1: String,
//         s2: String,
//     }

//     #[test]
//     fn memes() {
//         let mut foo = Foo {
//             s1: String::new(),
//             s2: String::new(),
//         };

//         let r1 = &mut foo;
//         let r2 = &mut r1.s2;

//         r1.s1.push_str("foo");
//         r2.push_str("bar");
//     }
// }

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn json() {
        let input = "key=abc12, age=5, key=xyz99, age=1, key=def45, age=1";

        let v: Vec<&str> = input.split(',').map(|str| str.trim()).collect();

        if v.is_empty() {
            return;
        }

        let iter = v.iter().enumerate();
        let mut res = String::new();

        let mut key: &str = "";
        let mut age: &str = "";
        let len = v.len();

        for (idx, str) in v.iter().enumerate() {
            if str.starts_with("key") {
                key = str;
                continue;
            }

            if str.starts_with("age") {
                age = str;
            }

            if !age.ends_with("1") {
                if !res.is_empty() && idx != len - 2 {
                    res.push_str(", ");
                }

                res.push_str(key);
                res.push_str(", ");
                res.push_str(age);
            }
        }
        assert_eq!("key=abc12, age=5", res);
    }
}
