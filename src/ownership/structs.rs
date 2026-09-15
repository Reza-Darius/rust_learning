#[cfg(test)]
mod meme_test {
    #[derive(Debug)]
    struct Pair {
        left: String,
        right: String,
    }

    impl Pair {
        // doesnt compile!
        
        // fn foo(&mut self) {
        //     let left = &mut self.left;
        //     left.push_str("hi");
        //     self.bar();
        //     println!("{left}");
        // }
        fn bar(&self) {
            println!("{self:?}");
        }
    }

    struct Foo {
        s1: String,
        s2: String,
    }

    #[test]
    fn memes() {
        let mut foo = Foo {
            s1: String::new(),
            s2: String::new(),
        };

        let r1 = &mut foo;
        let r2 = &mut r1.s2;

        r1.s1.push_str("foo");
        r2.push_str("bar");
    }
}

