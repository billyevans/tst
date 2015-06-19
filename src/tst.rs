
use std::mem;
use std::ops;
use std::fmt::{self, Debug};
use std::default::Default;
use self::Entry::*;
use std::iter::{Map, FromIterator};

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
/// for (key, value) in m.prefix_iter("first") {
///     println!("{}: {}", key, value);
/// }
/// ```

// by design TST depends on order of inserts in it, not only on keys and data itself

/// Root struct for TST, which holds root and size
#[derive(Clone, PartialEq, Eq)]
pub struct TST<V> {
    root: Option<Box<Node<V>>>,
    size: usize,
}

impl<V> TST<V> {
    /// Constructs a new, empty `TST<V>`.
    /// # Examples
    ///
    /// ```
    /// use tst::tst::TST;
    /// let mut t: TST<i64> = TST::new();
    /// ```
    #[inline]
    pub fn new() -> TST<V> {
        TST { root: None, size: 0 }
    }

    /// Returns the number of elements in the container.
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::tst::TST;
    ///
    /// let mut m = TST::new();
    /// assert_eq!(0, m.len());
    /// m.insert("ab", 2);
    /// m.insert("x", 1);
    /// assert_eq!(2, m.len());
    /// ```
    #[inline]
    pub fn len(&self) -> usize { self.size }

    /// Inserts an element at key `key` with value `val`.
    ///
    /// # Panics
    ///
    /// Panics if `key` is empty or more then 2000 symbols(because of reccursion).
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::tst::TST;
    ///
    /// let mut m = TST::new();
    /// m.insert("SOmeWOrd", 2);
    /// m.insert("SOmeOtherWOrd", 4);
    /// assert_eq!(2, m.len());
    /// ```
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
        let node = Node::get(&self.root, key.chars().collect(), 0);
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
        let node = Node::get_mut(&mut self.root, key.chars().collect(), 0);
        match node {
            None => None,
            Some(ptr) => {
                match *ptr {
                    None => None,
                    Some(ref mut cur) => {
                        match cur.val {
                            None => None,
                            Some(ref mut r) => Some(r)
                        }
                    }
                }
            }
        }
    }

    /// Returns true if the TST contains a value for the specified key.
    /// # Examples
    ///
    /// ```
    /// use tst::tst::TST;
    ///
    /// let mut m = TST::new();
    /// m.insert("abc", 1);
    /// assert!(!m.contains_key("ab"));
    /// assert!(m.contains_key("abc"))
    /// ```
     #[inline]
    pub fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// Returns true if the TST contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::tst::TST;
    ///
    /// let mut m = TST::new();
    /// assert!(m.is_empty());
    ///
    /// m.insert("abc", 1);
    /// assert!(!m.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool { self.size == 0 }

    /// Clears the TST.
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::tst::TST;
    ///
    /// let mut m = TST::new();
    /// m.insert("abc", 1);
    /// m.insert("abd", 100);
    /// m.clear();
    ///
    /// assert!(m.is_empty());
    /// assert_eq!(None, m.get("abc"));
    /// ```
    pub fn clear(&mut self) { *self = TST::<V>::new(); }

    /// An iterator returning all nodes matching wildcard pattern.
    /// Iterator element type is (String, V)
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
    /// for (k, v) in m.wildcard_iter(".") {
    ///     println!("{} -> {}", k, v);
    /// }
    /// ```
    pub fn wildcard_iter(&self, pat: &str) -> WildCardIter<V> {
        WildCardIter::<V>::new(&self.root, pat, self.len())
    }

    /// Method returns longest prefix in TST
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::tst::TST;
    /// let mut m = TST::new();
    /// m.insert("abc", 1);
    /// m.insert("abcd", 1);
    /// m.insert("abce", 1);
    /// m.insert("abca", 1);
    /// m.insert("zxd", 1);
    /// m.insert("add", 1);
    /// m.insert("abcdef", 1);
    ///
    /// assert_eq!("abcd", m.longest_prefix("abcde"));
    /// ```
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
    pub fn prefix_iter(&self, pref: &str) -> Iter<V> {
        let node = Node::get(&self.root, pref.chars().collect(), 0);

        match node {
            None => Default::default(),
            Some(ptr) => Iter::<V>::new_with_prefix(ptr, pref, self.len())
        }
    }
    pub fn prefix_iter_mut(&mut self, pref: &str) -> IterMut<V> {
        let len = self.len();
        let node = Node::get_mut(&mut self.root, pref.chars().collect(), 0);
        match node {
            None => Default::default(),
            Some(ptr) => IterMut::<V>::new_with_prefix(ptr, pref, len),
        }
    }
    pub fn iter(&self) -> Iter<V> {
        let len = self.len();
        Iter::<V>::new(&self.root, len, len)
    }

    pub fn iter_mut(&mut self) -> IterMut<V> {
        let len = self.len();
        IterMut::<V>::new(&mut self.root, len, len)
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
    pub fn keys(&self) -> KeysIter<V> {
        fn first<A, B>((k, _): (A, B)) -> A { k }
        KeysIter { iter: self.iter().map(first) }
    }

    /// An iterator visiting all values in arbitrary order.
    /// Iterator element type is &V
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
    /// for value in m.values() {
    ///     println!("{}", value);
    /// }
    /// ```
    pub fn values(&self) -> ValuesIter<V> {
        fn second<A, B>((_, v): (A, B)) -> B { v }
        ValuesIter { iter: self.iter().map(second) }
    }
}

impl<V> IntoIterator for TST<V> {
    type Item = (String, V);
    type IntoIter = IntoIter<V>;

    /// Creates a consuming iterator, that is, one that moves each key-value
    /// pair out of the TST in arbitrary order. The TST cannot be used after
    /// calling this.
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
    /// let vec: Vec<(String, isize)> = m.into_iter().collect();
    /// ```
    fn into_iter(self) -> IntoIter<V> {
        IntoIter::new(self)
    }
}

impl<'x, V> FromIterator<(&'x str, V)> for TST<V> {
    fn from_iter<I: IntoIterator<Item = (&'x str, V)>>(iter: I) -> TST<V> {
        let mut m = TST::new();
        for item in iter {
            m.insert(item.0, item.1);
        }
        m
    }
}

impl<'x, V> Extend<(&'x str, V)> for TST<V> {
    #[inline]
    fn extend<I: IntoIterator<Item=(&'x str, V)>>(&mut self, iter: I) {
        for (k, v) in iter {
            self.insert(k, v);
        }
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

#[derive(Clone, PartialEq, Eq)]
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
    fn get(node: &Option<Box<Node<V>>>, key: Vec<char>, i: usize) ->
            Option<&Option<Box<Node<V>>>>
    {
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
                    Some(node)
                }
            }
        }
    }

    fn get_mut(node: &mut Option<Box<Node<V>>>, key: Vec<char>, i: usize) ->
            Option<&mut Option<Box<Node<V>>>>
    {
        unsafe { mem::transmute(Node::get(node, key, i)) }
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

impl<V: Debug> Debug for Node<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{{\n"));
        try!(write!(f, "lt = {:?}, eq = {:?}, gt = {:?}, val = {:?}, c = {:?}",
            self.lt, self.eq, self.gt, self.val, self.c));
        (write!(f, "}}\n"))
    }
}

//
// iterators section
//

/// TST iterator.
#[derive(Clone)]
pub struct Iter<'a, V: 'a> {
    stack: Vec<(Option<&'a Option<Box<Node<V>>>>, String, Option<&'a V>)>,
    min_size: usize,
    max_size: usize,
}

impl<'a, V> Iter<'a, V> {
    fn new(ptr: &'a Option<Box<Node<V>>>, min: usize, max: usize) -> Iter<'a, V> {
        Iter {
            stack: vec![(Some(ptr), "".to_string(), None)],
            min_size: min,
            max_size: max,
        }
    }
    fn new_with_prefix(ptr: &'a Option<Box<Node<V>>>, prefix: &str, max: usize) -> Iter<'a, V> {
        let mut iter: Iter<V> = Default::default();
        match *ptr {
            None => (),
            Some(ref cur) => {
                iter.max_size = max;
                if cur.val.is_some() {
                    iter.min_size += 1;
                    iter.stack.push((None, prefix.to_string(), Some(cur.val.as_ref().unwrap())));
                }
                if cur.eq.is_some() {
                    iter.stack.push((Some(&cur.eq), prefix.to_string(), None));
                }

            }
        }
        iter
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

/// TST mutable iterator.
pub struct IterMut<'a, V: 'a> {
    iter: Iter<'a, V>,
}

impl<'a, V> IterMut<'a, V> {
    fn new(ptr: &'a mut Option<Box<Node<V>>>, min: usize, max: usize) -> IterMut<'a, V> {
        IterMut {
            iter : Iter::<V>::new(ptr, min, max),
        }
    }
    fn new_with_prefix(ptr: &'a Option<Box<Node<V>>>, prefix: &str, max: usize) -> IterMut<'a, V> {
        IterMut {
            iter : Iter::<V>::new_with_prefix(ptr, prefix, max),
        }
    }
}

impl<'a, V> Default for IterMut<'a, V> {
    fn default() -> IterMut<'a, V> {
        IterMut {
            iter : Default::default(),
        }
    }
}

impl<'a, V> Iterator for IterMut<'a, V> {
    type Item = (String, &'a mut V);
    fn next(&mut self) -> Option<(String, &'a mut V)> {
        // just add mut, avoid copy-paste
        unsafe { mem::transmute(self.iter.next()) }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

/// TST keys iterator
#[derive(Clone)]
pub struct KeysIter<'a, V: 'a> {
    iter: Map<Iter<'a, V>, fn((String, &'a V)) -> String>,
}

impl<'a, V:'a> Iterator for KeysIter<'a, V> {
    type Item = String;
    fn next(&mut self) -> Option<String> { self.iter.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}


/// TST values iterator
pub struct ValuesIter<'a, V:'a> {
    iter: Map<Iter<'a, V>, fn((String, &'a V)) -> &'a V>,
}

impl<'a, V:'a> Iterator for ValuesIter<'a, V> {
    type Item = &'a V;
    fn next(&mut self) -> Option<&'a V> { self.iter.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

/// TST wild-card iterator.
#[derive(Clone)]
pub struct WildCardIter<'a, V: 'a> {
    stack: Vec<(Option<&'a Option<Box<Node<V>>>>, String, Option<&'a V>, usize)>,
    max_size: usize,
    pat: Vec<char>,
}

impl<'a, V> WildCardIter<'a, V> {
    fn new(ptr: &'a Option<Box<Node<V>>>, pat: &str, max: usize) -> WildCardIter<'a, V> {
        WildCardIter {
            stack: vec![(Some(ptr), "".to_string(), None, 0)],
            max_size: max,
            pat: pat.chars().collect(),
        }
    }
}

impl<'a, V> Iterator for WildCardIter<'a, V> {
    type Item = (String, &'a V);
    fn next(&mut self) -> Option<(String, &'a V)> {
        while !self.stack.is_empty() {
            let node = self.stack.pop().unwrap();
            match node.0 {
                None => {
                    self.max_size -= 1;
                    return Some((node.1, node.2.unwrap()));
                }
                Some(n) => {
                    match *n {
                        None => {}
                        Some(ref cur) => {
                            let idx = node.3;
                            let ch = self.pat[idx];
                            let mut prefix = String::new();
                            if (ch == '.' || ch > cur.c) && cur.gt.is_some() {
                                self.stack.push((Some(&cur.gt), node.1.clone(), None, idx));
                            }
                            if ch == '.' || ch == cur.c {
                                if
                                    (idx+1 < self.pat.len() && cur.eq.is_some()) ||
                                    (idx+1 == self.pat.len() && cur.val.is_some())
                                {
                                    prefix.push_str(&node.1);
                                    prefix.push(cur.c);
                                }
                                if idx+1 < self.pat.len() && cur.eq.is_some() {
                                    self.stack.push((Some(&cur.eq), prefix.clone(), None, idx+1));
                                }

                                if idx+1 == self.pat.len() && cur.val.is_some() {
                                    self.stack.push((None, prefix, Some(cur.val.as_ref().unwrap()), idx));
                                }
                            }
                            if (ch == '.' || ch < cur.c) && cur.lt.is_some() {
                                self.stack.push((Some(&cur.lt), node.1.clone(), None, idx));
                            }
                        }
                    }
                }
            }
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.max_size))
    }
}

/// TST consuming iterator
pub struct IntoIter<V> {
    stack: Vec<(Option<Box<Node<V>>>, String, Option<V>)>,
    size: usize,
}

impl<V> IntoIter<V> {
    fn new(tst: TST<V>) -> IntoIter<V> {
        let size = tst.len();
        IntoIter {
            stack: vec![(tst.root, "".to_string(), None)],
            size: size,
        }
    }
}

impl<V> Iterator for IntoIter<V> {
    type Item = (String, V);

    fn next(&mut self) -> Option<(String, V)> {
        while !self.stack.is_empty() {
            let mut node = self.stack.pop().unwrap();
            match node.2 {
                Some(value) => {
                    self.size -= 1;
                    return Some((node.1, value));
                }
                None => {
                    match node.0 {
                        None => {}
                        Some(ref mut cur) => {
                            let mut prefix = String::new();
                            if cur.gt.is_some() {
                                self.stack.push((mem::replace(&mut cur.gt, None), node.1.clone(), None));
                            }
                            if cur.eq.is_some() || cur.val.is_some() {
                                prefix.push_str(&node.1);
                                prefix.push(cur.c);
                            }
                            if cur.eq.is_some() {
                                self.stack.push((mem::replace(&mut cur.eq, None), prefix.clone(), None));
                            }
                            if cur.val.is_some() {
                                self.stack.push((None, prefix, mem::replace(&mut cur.val, None)));
                            }
                            if cur.lt.is_some() {
                                self.stack.push((mem::replace(&mut cur.lt, None), node.1.clone(), None));
                            }
                        }
                    }
                }
            }
        }
        None
    }
fn size_hint(&self) -> (usize, Option<usize>) { (self.size, Some(self.size)) }
}

impl<V> ExactSizeIterator for IntoIter<V> {
    fn len(&self) -> usize { self.size }
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
