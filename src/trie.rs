struct Trie {
    root: Vec<Option<TrieNode>>,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: (0..26).map(|_| None).collect(),
        }
    }

    fn insert(&mut self, str: &str) {
        let bytes = str.as_bytes();

        let mut curr = self.root.as_mut_slice();
        for char in bytes {
            let idx = char - b'a';
            println!("inserting: {idx}");

            if let Some(ref mut node) = curr[idx as usize] {
                curr = node.array.as_mut_slice()
            } else {
                let new = TrieNode {
                    array: (0..26).map(|_| None).collect(),
                };
                curr[idx as usize] = Some(new);
                curr = curr[idx as usize].as_mut().unwrap().array.as_mut_slice();
            }
        }
    }

    fn search(&self, str: &str) -> bool {
        let bytes = str.as_bytes();

        let mut curr = self.root.as_slice();
        for char in bytes {
            let idx = char - b'a';
            println!("searching: {idx}");

            if let Some(ref node) = curr[idx as usize] {
                curr = node.array.as_slice()
            } else {
                return false;
            }
        }
        true
    }
}

struct TrieNode {
    array: Vec<Option<TrieNode>>,
}

#[cfg(test)]
mod test {
    use crate::trie::Trie;

    #[test]
    fn trie() {
        let mut trie = Trie::new();
        let pokemon = "bulbasaur";

        trie.insert(pokemon);
        assert!(trie.search(pokemon))
    }
    #[test]
    fn pokemon_trie_insert_and_search() {
        let mut trie = Trie::new();

        let pokemons = [
            "pikachu",
            "bulbasaur",
            "charmander",
            "squirtle",
            "eevee",
            "snorlax",
            "mew",
            "mewtwo",
            "psyduck",
            "jigglypuff",
        ];

        // insert all pokemon
        for p in pokemons {
            trie.insert(p);
        }

        // exact matches should succeed
        for p in pokemons {
            assert!(trie.search(p), "expected '{}' to be found in trie", p);
        }

        // prefixes (depending on your design, these may be false later)
        assert!(trie.search("pik"));
        assert!(trie.search("char"));
        assert!(trie.search("mew"));

        // non-existent words
        let not_present = ["raichu", "ivysaur", "charizard", "squir", "eeve", "pidgey"];

        for p in not_present {
            assert!(
                !trie.search(p),
                "did NOT expect '{}' to be found in trie",
                p
            );
        }
    }
}
