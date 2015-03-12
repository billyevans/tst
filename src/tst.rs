/*
    // all keys in symbol table
    public Iterable<String> keys() {
        Queue<String> queue = new Queue<String>();
        collect(root, "", queue);
        return queue;
    }
    // all keys starting with given prefix
    public Iterable<String> prefixMatch(String prefix) {
        Queue<String> queue = new Queue<String>();
        Node x = get(root, prefix, 0);
        if (x == null) return queue;
        if (x.val != null) queue.enqueue(prefix);
        collect(x.mid, prefix, queue);
        return queue;
    }
    // all keys in subtrie rooted at x with given prefix
    private void collect(Node x, String prefix, Queue<String> queue) {
        if (x == null) return;
        collect(x.left,  prefix,       queue);
        if (x.val != null) queue.enqueue(prefix + x.c);
        collect(x.mid,   prefix + x.c, queue);
        collect(x.right, prefix,       queue);
    }
}*/

/*
 *  Symbol table with string keys, implemented using a ternary search
 *  trie (TST).
 */

use std::mem;
use std::ops;
use std::fmt::{self, Debug};

#[derive(Clone)]
pub struct TST<V> {
    root: Option<Box<Node<V>>>,
    size: usize
}

impl<V> TST<V> {
    pub fn new() -> TST<V> {
        TST { root: None, size: 0 }
    }
    #[inline]
    pub fn len(&self) -> usize { self.size }
    // key must be non-empty string!
    pub fn insert(&mut self, key: &str, val: V) -> Option<V> {
        assert!(key.len() > 0);
        let ret = Node::insert(&mut self.root, key.chars().collect(), val, 0);
        if ret.is_none() { self.size += 1 }
        ret
    }
    pub fn remove(&mut self, key: &str) -> Option<V> {
        let ret = Node::remove(&mut self.root, key.chars().collect(), 0);
        if ret.is_some() { self.size -= 1 }
        ret
    }
    pub fn get(&self, key: &str) -> Option<&V> {
        Node::get(&self.root, key.chars().collect(), 0)
    }
    pub fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }
    pub fn is_empty(&self) -> bool { self.size == 0 }
    pub fn clear(&mut self) { *self = TST::<V>::new(); }
    pub fn longest_prefix<'r>(&self, pref: &'r str) -> &'r str {
        let mut length: usize = 0;
        let mut x = &self.root;
        let mut i: usize = 0;
        for k in pref.chars() {
            loop {
                match *x {
                    None => {
                        return pref.slice_to(length);
                    }
                    Some(ref r) => {
                        if k < r.c { x = &r.lt; }
                        else if k > r.c { x = &r.gt; }
                        else {
                            i += 1;
                            if r.val.is_some() { length = i; }
                            x = &r.eq;
                            break;
                        }
                    }
                }

            }
        }
        return pref.slice_to(length);
    }
  /*  pub fn iter(&self) -> Iter<V> {
        Iter {
            stack: vec![],
            ptr: &self.root,
            elems_left: self.len(),
        }
    }*/
}

impl<'x, V> ops::Index<&'x str> for TST<V> {
    type Output = V;
    #[inline]
    fn index(&self, idx: &&str) -> &V {
        self.get(idx).expect("no entry found for key")
    }
}
/*
impl<'x, V> ops::IndexMut<&'x str> for TST<V> {
    #[inline]
    fn index_mut(&mut self, idx: &&str) -> &mut V {
        self.get_mut(idx).expect("no entry found for key")
    }
}*/

impl<V: Debug> Debug for TST<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{{"));
        try!(write!(f, "{:?}", self.root));
        (write!(f, "}}"))
    }
}

#[derive(Clone)]
struct Node<V> {
    lt: Option<Box<Node<V>>>,
    eq: Option<Box<Node<V>>>,
    gt: Option<Box<Node<V>>>,
    val: Option<V>,
    c: char
}

impl<V> Node<V> {
    fn new(c: char) -> Node<V> {
        Node {
            lt: None,
            eq: None,
            gt: None,
            val: None,
            c: c
        }
    }
    fn insert(node: &mut Option<Box<Node<V>>>, key: Vec<char>, val: V, i: usize) -> Option<V> {
        let k = key[i];
        match *node {
            None => {
                *node = Some(Box::new(Node::new(k)));
                Node::insert(node, key, val, i)
            }
            Some(ref mut cur) => {
                if k < cur.c {
                    Node::insert(&mut cur.lt, key, val, i)
                }
                else if k > cur.c {
                    Node::insert(&mut cur.gt, key, val, i)
                }
                else if i+1 < key.len() {
                    Node::insert(&mut cur.eq, key, val, i+1)
                }
                else {
                    match cur.val {
                        None => {
                            cur.val = Some(val);
                            None
                        }
                        Some(_) => {
                            mem::replace(&mut cur.val, Some(val))
                        }
                    }
                }
            }
        }
    }
    fn get(node: &Option<Box<Node<V>>>, key: Vec<char>, i: usize) -> Option<&V> {
        if i >= key.len() { return None; }
        match *node {
            None => None,
            Some(ref cur) => {
                let k = key[i];
                if k < cur.c {
                    Node::get(&cur.lt, key, i)
                }
                else if k > cur.c {
                    Node::get(&cur.gt, key, i)
                }
                else if i + 1 < key.len() {
                    Node::get(&cur.eq, key, i+1)
                }
                else {
                    match cur.val {
                        None => None,
                        Some(ref r) => Some(r)
                    }
                }
            }
        }
    }
    // TODO: add shrink all tails
    fn remove(node: &mut Option<Box<Node<V>>>, key: Vec<char>, i: usize) -> Option<V> {
        if i >= key.len() { return None; }
        match *node {
            None => None,
            Some(ref mut cur) => {
                let k = key[i];
                if k < cur.c {
                    Node::remove(&mut cur.lt, key, i)
                }
                else if k > cur.c {
                    Node::remove(&mut cur.gt, key, i)
                }
                else if i + 1 < key.len() {
                    Node::remove(&mut cur.eq, key, i+1)
                }
                else {
                    match cur.val {
                        None => None,
                        Some(_) => mem::replace(&mut cur.val, None)
                    }
                }
            }
        }
    }
}

impl<V: Debug> Debug for Option<Box<Node<V>>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn iterate<V: Debug>(node: &Option<Box<Node<V>>>, f: &mut fmt::Formatter, prefix: &mut String)
            -> fmt::Result 
        {
            match *node {
                None => write!(f, ""),
                Some(ref cur) => {
                    try!(iterate(&cur.lt, f, prefix));
                    prefix.push(cur.c);
                    match cur.val {
                        None => (),
                        Some(ref val) => {
                            try!(write!(f, "{:?}: {:?},", prefix, val));
                        }
                    }
                    try!(iterate(&cur.eq, f, prefix));
                    prefix.pop();
                    iterate(&cur.gt, f, prefix)
                }
            }
        }
        iterate(self, f, &mut String::from_str(""))
    }
}

/*
#[derive(Clone)]
pub struct Iter<'a, V: 'a> {
    stack: Vec<&'a Box<Node<V>>>,
    ptr: &'a Option<Box<Node<V>>>,
    elems_left: usize,
}

impl<'a, V> Iterator for Iter<'a, V> {
    type Item = (&'a str, &'a V);
    fn next(&mut self) -> Option<(&'a str, &'a V)> {
        match *self.ptr {
            None => None,
            Some(ref r) => {
                loop {
                    if k < r.c { x = &r.lt; }
                    else if k > r.c { x = &r.gt; }
                    else {
                        i += 1;
                        if r.val.is_some() { length = i; }
                        x = &r.eq;
                        break;
                    }
                }
            }
        }

    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.elems_left, Some(self.elems_left))
    }
}
impl<'a, V> ExactSizeIterator for Iter<'a, V> {
    fn len(&self) -> usize { self.elems_left }
}
*/
