
use std::mem;
use std::ops;
use std::fmt::{self, Debug};
use std::default::Default;

///
/// Symbol table with string keys, implemented using a ternary search
/// trie (TST).
///
/// There is character on each node of the trie, value and links for children.
/// Each node has 3 children: smaller (lt), equal (eq), larger (gt).
/// It could be used as associative array for strings as keys.
/// Also it provides extra features, like getting all keys, values with common prefix.
/// # Examples
///
/// ```rust
/// use tst::tst::TST;
///
/// let mut m = TST::new();
///
/// m.insert("first", 1);
/// m.insert("second", 2);
/// m.insert("firstthird", 3);
/// m.insert("firstsecond", 12);
///
/// for (key, value) in m.iter() {
///     println!("{}: {}", key, value);
/// }
/// assert_eq!(Some(&1), m.get("first"));
/// assert_eq!(4, m.len());
///
/// // calculating longest prefix
/// assert_eq!("firstsecond", m.longest_prefix("firstsecondthird"));
///
/// // get values with common prefix
/// for (key, value) in m.iter_prefix("first") {
///     println!("{}: {}", key, value);
/// }
/// ```


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
        let node = Node::get_node(&self.root, key.chars().collect(), 0);
        match node {
            None => None,
            Some(ptr) => {
                match *ptr {
                    None => None,
                    Some(ref cur) => {
                        match cur.val {
                            None => None,
                            Some(ref r) => Some(r)
                        }                        
                    }
                 }
            }
        }
    }
    pub fn get_mut(&mut self, key: &str) -> Option<&mut V> {
        Node::get_mut(&mut self.root, key.chars().collect(), 0)
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
    pub fn iter_prefix(&self, pref: &str) -> Iter<V> {
        let node = Node::get_node(&self.root, pref.chars().collect(), 0);
        match node {
            None => Iter { // TODO: Defaut ?!
                        stack: vec![],
                        min_size: 0,
                        max_size: 0,
                    },
            Some(ptr) => Iter::<V>::new(ptr, pref.slice_to(pref.len()-1), 0, self.len()),
        }
    }
    pub fn iter(&self) -> Iter<V> {
        Iter::<V>::new(&self.root, "", self.len(), self.len())
    }
}

impl<'x, V> ops::Index<&'x str> for TST<V> {
    type Output = V;
    #[inline]
    fn index(&self, idx: &&str) -> &V {
        self.get(idx).expect("no entry found for key")
    }
}

impl<'x, V> ops::IndexMut<&'x str> for TST<V> {
    #[inline]
    fn index_mut(&mut self, idx: &&str) -> &mut V {
        self.get_mut(idx).expect("no entry found for key")
    }
}

impl<V: Debug> Debug for TST<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{{"));
        for (k, v) in self.iter() {
            try!(write!(f, "{:?}: {:?},", k, v));
        }
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
    fn get_node(node: &Option<Box<Node<V>>>, key: Vec<char>, i: usize) ->
            Option<&Option<Box<Node<V>>>> 
    {
        if i >= key.len() { return None; }
        match *node {
            None => None,
            Some(ref cur) => {
                let k = key[i];
                if k < cur.c {
                    Node::get_node(&cur.lt, key, i)
                }
                else if k > cur.c {
                    Node::get_node(&cur.gt, key, i)
                }
                else if i + 1 < key.len() {
                    Node::get_node(&cur.eq, key, i+1)
                }
                else {
                    Some(node)
                }
            }
        }
    }

    fn get_mut(node: &mut Option<Box<Node<V>>>, key: Vec<char>, i: usize) -> Option<&mut V> {
        if i >= key.len() { return None; }
        match *node {
            None => None,
            Some(ref mut cur) => {
                let k = key[i];
                if k < cur.c {
                    Node::get_mut(&mut cur.lt, key, i)
                }
                else if k > cur.c {
                    Node::get_mut(&mut cur.gt, key, i)
                }
                else if i + 1 < key.len() {
                    Node::get_mut(&mut cur.eq, key, i+1)
                }
                else {
                    match cur.val {
                        None => None,
                        Some(ref mut r) => Some(r)
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

#[derive(Clone)]
pub struct Iter<'a, V: 'a> {
    stack: Vec<(Option<&'a Option<Box<Node<V>>>>, String, Option<&'a V>)>,
    min_size: usize,
    max_size: usize,
}

impl<'a, V> Iter<'a, V> {
    fn new(ptr: &'a Option<Box<Node<V>>>, prefix: &str, min: usize, max: usize) -> Iter<'a, V> {
        Iter {
            stack: vec![(Some(ptr), String::from_str(prefix), None)],
            min_size: min,
            max_size: max,
        }
    }
}

impl<'a, V> Default for Iter<'a, V> {
    fn default() -> Iter<'a, V> {
        Iter {
            stack: vec![],
            min_size: 0,
            max_size: 0,
        }
    }
}

impl<'a, V> Iterator for Iter<'a, V> {
    type Item = (String, &'a V);
    fn next(&mut self) -> Option<(String, &'a V)> {
        while !self.stack.is_empty() {
            let node = self.stack.pop().unwrap();
            match node.0 {
                None => {
                    if self.min_size == self.max_size {
                        self.min_size -= 1;
                    }
                    self.max_size -= 1;
                    return Some((node.1, node.2.unwrap()));
                }
                Some(n) => {
                    match *n {
                        None => {}
                        Some(ref cur) => {
                            let mut prefix = String::new();
                            if cur.gt.is_some() {
                                self.stack.push((Some(&cur.gt), node.1.clone(), None));
                            }
                            if cur.eq.is_some() || cur.val.is_some() {
                                prefix.push_str(node.1.as_slice());
                                prefix.push(cur.c);
                            }
                            if cur.eq.is_some() {
                                self.stack.push((Some(&cur.eq), prefix.clone(), None));
                            }
                            if cur.val.is_some() {
                                self.stack.push((None, prefix, Some(cur.val.as_ref().unwrap())));
                            }
                            if cur.lt.is_some() {
                                self.stack.push((Some(&cur.lt), node.1.clone(), None));
                            }
                        }
                    }
                }
            }
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.min_size, Some(self.max_size))
    }
}

