
use std::mem;
use std::ops;
use std::fmt::{self, Debug};
use std::default::Default;
use self::Entry::*;
use std::iter::{Map};

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
    size: usize,
}

impl<V> TST<V> {
    pub fn new() -> TST<V> {
        TST { root: None, size: 0 }
    }
    #[inline]
    pub fn len(&self) -> usize { self.size }
    // key must be non-empty string!
    pub fn insert(&mut self, key: &str, val: V) -> Option<V> {
        assert!(key.len() > 0, "Empty key");
        assert!(key.len() < 2000, "Key is too long");

        let cur = Node::insert_node(&mut self.root, key.chars().collect(), 0);
        let ret = mem::replace(&mut cur.val, Some(val));
        if ret.is_none() { self.size += 1 }
        ret
    }
    pub fn entry(&mut self, key: &str) -> Entry<V> {
        assert!(key.len() > 0, "Empty key");
        let l = &mut self.size;
        let cur = Node::insert_node(&mut self.root, key.chars().collect(), 0);
        Entry::<V>::new(cur, l)
    }
    pub fn remove(&mut self, key: &str) -> Option<V> {
        let ret = Node::remove(&mut self.root, key.chars().collect(), 0);
        if ret.is_some() {
            self.size -= 1;
        }
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
    pub fn longest_prefix<'a>(&self, pref: &'a str) -> &'a str {
        let mut length: usize = 0;
        let mut x = &self.root;
        let mut i: usize = 0;
        for k in pref.chars() {
            loop {
                match *x {
                    None => {
                        return &pref[..length];
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
        return &pref[..length];
    }
    /// get iterator over all values with common prefix
    pub fn iter_prefix(&self, pref: &str) -> Iter<V> {
        let node = Node::get_node(&self.root, pref.chars().collect(), 0);
        match node {
            None => Default::default(),
            Some(ptr) => Iter::<V>::new(ptr, &pref[..pref.len()-1], 0, self.len()),
        }
    }
    pub fn iter(&self) -> Iter<V> {
        let len = self.len();
        Iter::<V>::new(&self.root, "", len, len)
    }
    /*
    pub fn iter_mut(&mut self) -> IterMut<V> {
        let len = self.len();
        IterMut::<V>::new(&mut self.root, "", len, len)
    }*/
    /// An iterator visiting all keys in arbitrary order.
    /// Iterator element type is String
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::tst::TST;
    ///
    /// let mut m = TST::new();
    /// m.insert("a", 1);
    /// m.insert("b", 2);
    /// m.insert("c", 3);
    ///
    /// for key in m.keys() {
    ///     println!("{}", key);
    /// }
    /// ```
    pub fn keys(&self) -> Keys<V> {
        fn first<A, B>((k, _): (A, B)) -> A { k }
        Keys { iter: self.iter().map(first) }
    }
    /// An iterator visiting all keys in arbitrary order.
    /// Iterator element type is String
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::tst::TST;
    ///
    /// let mut m = TST::new();
    /// m.insert("a", 1);
    /// m.insert("b", 2);
    /// m.insert("c", 3);
    ///
    /// for key in m.keys() {
    ///     println!("{}", key);
    /// }
    /// ```
    pub fn values(&self) -> Values<V> {
        fn second<A, B>((_, v): (A, B)) -> B { v }
        Values { iter: self.iter().map(second) }
    }
}

impl<'x, V> ops::Index<&'x str> for TST<V> {
    type Output = V;
    #[inline]
    fn index(&self, idx: &str) -> &V {
        self.get(idx).expect("no entry found for key")
    }
}

impl<'x, V> ops::IndexMut<&'x str> for TST<V> {
    #[inline]
    fn index_mut(&mut self, idx: &str) -> &mut V {
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
    fn insert_node(node: &mut Option<Box<Node<V>>>, key: Vec<char>, i: usize) -> &mut Box<Node<V>> {
        let k = key[i];
        match *node {
            None => {
                *node = Some(Box::new(Node::new(k)));
                Node::insert_node(node, key, i)
            }
            Some(ref mut cur) => {
                if k < cur.c {
                    Node::insert_node(&mut cur.lt, key, i)
                }
                else if k > cur.c {
                    Node::insert_node(&mut cur.gt, key, i)
                }
                else if i+1 < key.len() {
                    Node::insert_node(&mut cur.eq, key, i+1)
                }
                else {
                    cur
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

/// TST iterator.
#[derive(Clone)]
pub struct Iter<'a, V: 'a> {
    stack: Vec<(Option<&'a Option<Box<Node<V>>>>, String, Option<&'a V>)>,
    min_size: usize,
    max_size: usize,
}

impl<'a, V> Iter<'a, V> {
    fn new(ptr: &'a Option<Box<Node<V>>>, prefix: &str, min: usize, max: usize) -> Iter<'a, V> {
        Iter {
            stack: vec![(Some(ptr), prefix.to_string(), None)],
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
                                prefix.push_str(&node.1);
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

/*
/// TST mutable values iterator.
pub struct IterMut<'a, V: 'a> {
    stack: Vec<(Option<&'a mut Option<Box<Node<V>>>>, String, Option<&'a mut V>)>,
    min_size: usize,
    max_size: usize,
}

impl<'a, V> IterMut<'a, V> {
    fn new(ptr: &'a mut Option<Box<Node<V>>>, prefix: &str, min: usize, max: usize) -> IterMut<'a, V> {
        IterMut {
            stack: vec![(Some(ptr), String::from_str(prefix), None)],
            min_size: min,
            max_size: max,
        }
    }
}

impl<'a, V> Default for IterMut<'a, V> {
    fn default() -> IterMut<'a, V> {
        IterMut {
            stack: vec![],
            min_size: 0,
            max_size: 0,
        }
    }
}

impl<'a, V> Iterator for IterMut<'a, V> {
    type Item = (String, &'a mut V);
    fn next(&mut self) -> Option<(String, &'a mut V)> {
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
                                self.stack.push((Some(&mut cur.gt), node.1.clone(), None));
                            }
                            if cur.eq.is_some() || cur.val.is_some() {
                                prefix.push_str(node.1.as_slice());
                                prefix.push(cur.c);
                            }
                            if cur.eq.is_some() {
                                self.stack.push((Some(&mut cur.eq), prefix.clone(), None));
                            }
                            if cur.val.is_some() {
                                self.stack.push((None, prefix, Some(cur.val.as_mut().unwrap())));
                            }
                            if cur.lt.is_some() {
                                self.stack.push((Some(&mut cur.lt), node.1.clone(), None));
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
*/

/// TST keys iterator
#[derive(Clone)]
pub struct Keys<'a, V: 'a> {
    iter: Map<Iter<'a, V>, fn((String, &'a V)) -> String>,
}

impl<'a, V:'a> Iterator for Keys<'a, V> {
    type Item = String;
    fn next(&mut self) -> Option<String> { self.iter.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}


/// TST values iterator
pub struct Values<'a, V:'a> {
    iter: Map<Iter<'a, V>, fn((String, &'a V)) -> &'a V>,
}

impl<'a, V:'a> Iterator for Values<'a, V> {
    type Item = &'a V;
    fn next(&mut self) -> Option<&'a V> { self.iter.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

/// A view into a single occupied location in a TST.
pub struct OccupiedEntry<'a, V: 'a> {
    node: &'a mut Box<Node<V>>,
    cont_size: &'a mut usize,
}

/// A view into a single empty location in a TST.
pub struct VacantEntry<'a, V: 'a> {
    node: &'a mut Box<Node<V>>,
}

/// A view into a single location in a TST, which may be vacant or occupied.
pub enum Entry<'a, V: 'a> {
    Occupied(OccupiedEntry<'a, V>),
    Vacant(VacantEntry<'a, V>),
}

impl<'a, V> Entry<'a, V> {
    fn new(node: &'a mut Box<Node<V>>, size: &'a mut usize) -> Entry<'a, V> {
        match node.val {
            None => Vacant(VacantEntry::new(node)),
            Some(_) => Occupied(OccupiedEntry::new(node, size)),
        }
    }
    pub fn get(self) -> Result<&'a mut V, VacantEntry<'a, V>> {
        match self {
            Occupied(entry) => Ok(entry.into_mut()),
            Vacant(entry) => Err(entry),
        }
    }
    /// Ensures a value is in the entry by inserting the default if empty, and returns
    /// a mutable reference to the value in the entry.
    pub fn or_insert(self, default: V) -> &'a mut V {
        match self {
            Occupied(entry) => entry.into_mut(),
            Vacant(entry) => entry.insert(default),
        }
    }
    /// Ensures a value is in the entry by inserting the result of the default function if empty,
    /// and returns a mutable reference to the value in the entry.
    pub fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V {
        match self {
            Occupied(entry) => entry.into_mut(),
            Vacant(entry) => entry.insert(default()),
        }
    }
}

impl<'a, V> OccupiedEntry<'a, V> {
    fn new(node: &'a mut Box<Node<V>>, size: &'a mut usize) -> OccupiedEntry<'a, V> {
        OccupiedEntry {node: node, cont_size: size}
    }
    /// Gets a reference to the value in the entry.
    pub fn get(&self) -> &V {
        self.node.val.as_ref().unwrap()
    }
    /// Gets a mutable reference to the value in the entry.
    pub fn get_mut(&mut self) -> &mut V {
        self.node.val.as_mut().unwrap()
    }
    /// Converts the OccupiedEntry into a mutable reference to the value in the entry
    /// with a lifetime bound to the tst itself
    pub fn into_mut(self) -> &'a mut V {
        self.node.val.as_mut().unwrap()
    }
    /// Sets the value of the entry, and returns the entry's old value
    pub fn insert(&mut self, value: V) -> V {
        mem::replace(&mut self.node.val, Some(value)).unwrap()
    }
    /// Takes the value out of the entry, and returns it
    pub fn remove(self) -> V {
        *self.cont_size -= 1;
        mem::replace(&mut self.node.val, None).unwrap()
    }
}

impl<'a, V> VacantEntry<'a, V> {
    fn new(node: &'a mut Box<Node<V>>) -> VacantEntry<'a, V> {
        VacantEntry {node: node}
    }
    /// Sets the value of the entry with the VacantEntry's key,
    /// and returns a mutable reference to it
    pub fn insert(self, value: V) -> &'a mut V {
        self.node.val = Some(value);
        self.node.val.as_mut().unwrap()
    }
}


