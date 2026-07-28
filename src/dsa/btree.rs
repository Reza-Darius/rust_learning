use std::ptr::{self, NonNull};

struct BTree<K, V> {
    root: Link<K, V>,
    len: usize,
    height: usize,
}

type Link<K, V> = Option<NonNull<Node<K, V>>>;

enum Node<K, V> {
    Internal {
        ptr: Vec<Link<K, V>>,
        elems: Vec<(K, V)>,
    },
    Leaf {
        elems: Vec<(K, V)>,
    },
}

impl<K: Ord, V> Node<K, V> {
    fn find_ge(&self, key: &K) -> Option<usize> {
        todo!()
    }
}

// - internal node store elems.len() + 1 pointer
// - all nodes except the root have at minimum MIN_SIZE - 1 keys
// - internal nodes have at least MIN_SIZE children
// - internal node may have at most 2 * MIN_SIZE children

// upper bound
const MAX_SIZE: usize = MIN_SIZE * 2 - 1;

// lower bound: t >= 2
const MIN_SIZE: usize = 2;

impl<K, V> BTree<K, V>
where
    K: Ord + std::fmt::Debug,
{
    fn new() -> Self {
        BTree {
            root: Self::new_leaf(),
            len: 0,
            height: 0,
        }
    }

    fn insert(&mut self, key: K, val: V) {
        todo!()
    }

    fn new_leaf() -> Link<K, V> {
        NonNull::new(Box::into_raw(Box::new(Node::Leaf { elems: Vec::new() })))
    }

    fn new_node() -> Link<K, V> {
        NonNull::new(Box::into_raw(Box::new(Node::Internal {
            ptr: Vec::new(),
            elems: Vec::new(),
        })))
    }
}
