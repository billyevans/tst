use std::mem;
use std::ops;
use std::fmt::{self, Debug};
use std::default::Default;
use self::Entry::*;
use std::iter::{Map, FromIterator};
use super::node::{Node, NodeRef, NodeRefMut, BoxedNode};
use super::traverse::{self, Traverse, ValuesTraverse, IntoTraverse, WildCardTraverse, DropTraverse};

///
/// Symbol table with string keys, implemented using a ternary search
/// trie (`TSTMap`).
///
/// There is character on each node of the trie, value and links for children.
/// Each node has 3 children: smaller (lt), equal (eq), larger (gt).
/// It could be used as associative array for strings as keys.
/// Also it provides extra features, like getting all keys, values with common prefix.
/// # Examples
///
/// ```rust
/// use tst::TSTMap;
///
/// let mut m = TSTMap::new();
///
/// m.insert("first", 1);
/// m.insert("second", 2);
/// m.insert("firstthird", 3);
/// m.insert("firstsecond", 12);
/// m.insert("xirst", -13);
///
/// // iterate
/// for (key, value) in m.iter() {
///     println!("{}: {}", key, value);
/// }
/// assert_eq!(Some(&1), m.get("first"));
/// assert_eq!(5, m.len());
///
/// // calculating longest prefix
/// assert_eq!("firstsecond", m.longest_prefix("firstsecondthird"));
///
/// // get values with common prefix
/// for (key, value) in m.prefix_iter("first") {
///     println!("{}: {}", key, value);
/// }
///
/// // get sum by wildcard iterator
/// assert_eq!(-12, m.wildcard_iter(".irst").fold(0, |sum, (_, val)| sum + val));
/// ```

// by design TSTMap depends on order of inserts in it, not only on keys and data itself

/// Root struct for `TSTMap`, which holds root and size.
#[derive(Clone, PartialEq, Eq)]
pub struct TSTMap<Value> {
    root: BoxedNode<Value>,
    size: usize,
}

impl<Value> TSTMap<Value> {
    /// Constructs a new, empty `TSTMap<Value>`.
    /// # Examples
    ///
    /// ```
    /// use tst::TSTMap;
    /// let mut t: TSTMap<i64> = TSTMap::new();
    /// ```
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns the number of elements in the container.
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::TSTMap;
    ///
    /// let mut m = TSTMap::new();
    /// assert_eq!(0, m.len());
    /// m.insert("ab", 2);
    /// m.insert("x", 1);
    /// assert_eq!(2, m.len());
    /// ```
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
    /// use tst::TSTMap;
    ///
    /// let mut m = TSTMap::new();
    /// m.insert("SOmeWOrd", 2);
    /// m.insert("SOmeOtherWOrd", 4);
    /// assert_eq!(2, m.len());
    /// ```
    pub fn insert(&mut self, key: &str, value: Value) -> Option<Value> {
        assert!(!key.is_empty(), "Empty key");
        match self.entry(key) {
            Occupied(mut entry) => Some(entry.insert(value)),
            Vacant(entry) => {
                entry.insert(value);
                None
            }
        }
    }

    /// Gets the given `key`'s corresponding entry in the TSTMap for in-place manipulation.
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::TSTMap;
    ///
    /// let mut count: TSTMap<usize> = TSTMap::new();
    ///
    /// for x in vec!["abc","bad","abd","cdddd","abc","bade"] {
    ///     *count.entry(x).or_insert(0) += 1;
    /// }
    ///
    /// assert_eq!(2, count["abc"]);
    /// assert_eq!(1, count["abd"]);
    /// ```
    pub fn entry(&mut self, key: &str) -> Entry<Value> {
        assert!(!key.is_empty(), "Empty key");
        let l = &mut self.size;
        let cur = traverse::insert(self.root.as_mut(), key);
        Entry::<Value>::new(cur, l)
    }

    /// Removes a `key` from the TSTMap, returning the value at the key if the key
    /// was previously in the TSTMap.
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::TSTMap;
    ///
    /// let mut m = TSTMap::new();
    /// m.insert("abc", 100);
    /// assert_eq!(Some(100), m.remove("abc"));
    /// assert_eq!(None, m.remove("abc"));
    /// ```
    pub fn remove(&mut self, key: &str) -> Option<Value> {
        let ret = traverse::remove(self.root.as_mut(), key);
        if ret.is_some() {
            self.size -= 1;
        }
        ret
    }

    /// Returns a reference to the value corresponding to the `key` or None.
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::TSTMap;
    ///
    /// let mut m = TSTMap::new();
    /// m.insert("first", 13);
    /// assert_eq!(Some(&13), m.get("first"));
    /// assert_eq!(None, m.get("second"));
    /// ```
    pub fn get(&self, key: &str) -> Option<&Value> {
        match traverse::search(self.root.as_ref(), key) {
            None => None,
            Some(ptr) => ptr.value.as_ref(),
        }
    }

    /// Returns a mutable reference to the value corresponding to the `key`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::TSTMap;
    ///
    /// let mut m = TSTMap::new();
    /// m.insert("first", 13);
    /// if let Some(x) = m.get_mut("first") {
    ///     *x = -13;
    /// }
    /// assert_eq!(-13, m["first"]);
    /// ```
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        match traverse::search_mut(self.root.as_ref_mut(), key) {
            None => None,
            Some(ptr) => ptr.value.as_mut(),
        }
    }

    /// Returns true if the `TSTMap` contains a value for the specified `key`.
    /// # Examples
    ///
    /// ```
    /// use tst::TSTMap;
    ///
    /// let mut m = TSTMap::new();
    /// m.insert("abc", 1);
    /// assert!(!m.contains_key("ab"));
    /// assert!(m.contains_key("abc"))
    /// ```
     #[inline]
    pub fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// Returns true if the `TSTMap` contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::TSTMap;
    ///
    /// let mut m = TSTMap::new();
    /// assert!(m.is_empty());
    ///
    /// m.insert("abc", 1);
    /// assert!(!m.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool { self.size == 0 }

    /// Clears the `TSTMap`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::TSTMap;
    ///
    /// let mut m = TSTMap::new();
    /// m.insert("abc", 1);
    /// m.insert("abd", 100);
    /// m.clear();
    ///
    /// assert!(m.is_empty());
    /// assert_eq!(None, m.get("abc"));
    /// ```
    pub fn clear(&mut self) { *self = TSTMap::<Value>::new(); }

    /// An iterator returning all nodes matching wildcard pattern `pat`.
    /// Iterator element type is (String, V)
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::TSTMap;
    ///
    /// let mut m = TSTMap::new();
    /// m.insert("a", 1);
    /// m.insert("b", 2);
    /// m.insert("c", 3);
    ///
    /// for (k, v) in m.wildcard_iter(".") {
    ///     println!("{} -> {}", k, v);
    /// }
    /// ```
    pub fn wildcard_iter(&self, pat: &str) -> WildCardIter<Value> {
        WildCardIter::new(self.root.as_ref(), pat, self.len())
    }

    /// An mutable iterator returning all nodes matching wildcard pattern `pat`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::TSTMap;
    ///
    /// let mut m = TSTMap::new();
    /// m.insert("a", 1);
    /// m.insert("b", 2);
    /// m.insert("c", 3);
    ///
    /// for (k, v) in m.wildcard_iter_mut(".") {
    ///     *v += 10;
    /// }
    /// assert_eq!(11, m["a"]);
    /// assert_eq!(12, m["b"]);
    /// assert_eq!(13, m["c"]);
    /// ```
    pub fn wildcard_iter_mut(&mut self, pat: &str) -> WildCardIterMut<Value> {
        WildCardIterMut::new(self.root.as_ref_mut(), pat, self.len())
    }

    /// Method returns iterator over all values with common prefix `pref` in the `TSTMap`.
    /// # Examples
    ///
    /// ```
    /// use tst::TSTMap;
    /// let mut m = TSTMap::new();
    /// m.insert("abc", 1);
    /// m.insert("abcd", 1);
    /// m.insert("abce", 1);
    /// m.insert("abca", 1);
    /// m.insert("zxd", 1);
    /// m.insert("add", 1);
    /// m.insert("abcdef", 1);
    ///
    /// for (key, value) in m.prefix_iter("abc") {
    ///     println!("{}: {}", key, value);
    /// }
    ///
    /// let (first_key, first_value) = m.iter().next().unwrap();
    /// assert_eq!((first_key, *first_value), ("abc".to_string(), 1));
    /// ```
    pub fn prefix_iter(&self, pref: &str) -> Iter<Value> {
        let node = traverse::search(self.root.as_ref(), pref);
        Iter::with_prefix(node, pref, self.len())
    }

    /// Method returns mutable iterator over all values with common prefix `pref` in the `TSTMap`.
    /// # Examples
    ///
    /// ```
    /// use tst::TSTMap;
    /// let mut m = TSTMap::new();
    /// m.insert("abc", 1);
    /// m.insert("abcd", 1);
    /// m.insert("abce", 1);
    /// m.insert("abca", 1);
    /// m.insert("zxd", 1);
    /// m.insert("add", 1);
    /// m.insert("abcdef", 1);
    ///
    /// for (key, value) in m.prefix_iter_mut("abc") {
    ///     *value += 100;
    /// }
    /// assert_eq!(101, m["abc"]);
    /// assert_eq!(101, m["abcdef"]);
    /// ```
    pub fn prefix_iter_mut(&mut self, pref: &str) -> IterMut<Value> {
        let len = self.len();
        let node = traverse::search(self.root.as_ref(), pref);
        IterMut::with_prefix(node, pref, len)
    }

    /// Gets an iterator over the entries of the TSTMap.
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::TSTMap;
    ///
    /// let mut m = TSTMap::new();
    /// m.insert("abc", 1);
    /// m.insert("bbc", 2);
    /// m.insert("cccda", 3);
    ///
    /// for (key, value) in m.iter() {
    ///     println!("{}: {}", key, value);
    /// }
    ///
    /// let (first_key, first_value) = m.iter().next().unwrap();
    /// assert_eq!((first_key, *first_value), ("abc".to_string(), 1));
    /// ```
    pub fn iter(&self) -> Iter<Value> {
        let len = self.len();
        Iter::new(self.root.as_ref(), len, len)
    }

    /// Gets a mutable iterator over the entries of the `TSTMap`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::TSTMap;
    ///
    /// let mut m = TSTMap::new();
    /// m.insert("a", 1);
    /// m.insert("b", 2);
    /// m.insert("c", 3);
    ///
    /// for (key, value) in m.iter_mut() {
    ///     if key != "a" {
    ///         *value += 10;
    ///     }
    /// }
    /// assert_eq!(1, m["a"]);
    /// assert_eq!(12, m["b"]);
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<Value> {
        let len = self.len();
        IterMut::new(self.root.as_ref_mut(), len, len)
    }

    /// An iterator visiting all keys in arbitrary order.
    /// Iterator element type is String
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::TSTMap;
    ///
    /// let mut m = TSTMap::new();
    /// m.insert("a", 1);
    /// m.insert("b", 2);
    /// m.insert("c", 3);
    ///
    /// for key in m.keys() {
    ///     println!("{}", key);
    /// }
    /// ```
    pub fn keys(&self) -> KeysIter<Value> {
        fn first<A, B>((k, _): (A, B)) -> A { k }
        KeysIter { iter: self.iter().map(first) }
    }

    /// An iterator visiting all values in arbitrary order.
    /// Iterator element type is &V
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::TSTMap;
    ///
    /// let mut m = TSTMap::new();
    /// m.insert("a", 1);
    /// m.insert("b", 2);
    /// m.insert("c", 3);
    ///
    /// for value in m.values() {
    ///     println!("{}", value);
    /// }
    /// ```
    pub fn values(&self) -> ValuesIter<Value> {
        ValuesIter { iter: ValuesTraverse::new(self.root.as_ref(), self.len(), self.len()) }
    }
}

impl<'x, Value: 'x> TSTMap<Value> {
    /// Method returns longest prefix `pref` in the `TSTMap`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::TSTMap;
    /// let mut m = TSTMap::new();
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
    pub fn longest_prefix(&self, pref: &'x str) -> &'x str {
        traverse::longest_prefix(self.root.as_ref(), pref)
    }
}

impl<Value> IntoIterator for TSTMap<Value> {
    type Item = (String, Value);
    type IntoIter = IntoIter<Value>;

    /// Creates a consuming iterator, that is, one that moves each key-value
    /// pair out of the `TSTMap` in arbitrary order. The `TSTMap` cannot be used after
    /// calling this.
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::TSTMap;
    ///
    /// let mut m = TSTMap::new();
    /// m.insert("a", 1);
    /// m.insert("b", 2);
    /// m.insert("c", 3);
    ///
    /// let vec: Vec<(String, isize)> = m.into_iter().collect();
    /// ```
    fn into_iter(self) -> IntoIter<Value> {
        IntoIter::new(self)
    }
}

impl<'x, Value> FromIterator<(&'x str, Value)> for TSTMap<Value> {
    fn from_iter<I: IntoIterator<Item = (&'x str, Value)>>(iter: I) -> TSTMap<Value> {
        let mut m = TSTMap::new();
        for item in iter {
            m.insert(item.0, item.1);
        }
        m
    }
}

impl<'x, Value> Extend<(&'x str, Value)> for TSTMap<Value> {
    #[inline]
    fn extend<I: IntoIterator<Item=(&'x str, Value)>>(&mut self, iter: I) {
        for (k, v) in iter {
            self.insert(k, v);
        }
    }
}

impl<'x, Value> ops::Index<&'x str> for TSTMap<Value> {
    type Output = Value;
    #[inline]
    fn index(&self, idx: &str) -> &Value {
        self.get(idx).expect("no entry found for key")
    }
}

impl<'x, Value> ops::IndexMut<&'x str> for TSTMap<Value> {
    #[inline]
    fn index_mut(&mut self, idx: &str) -> &mut Value {
        self.get_mut(idx).expect("no entry found for key")
    }
}

impl<Value> Drop for TSTMap<Value> {
    fn drop(&mut self) {
        let root = self.root.take();
        let mut iter = DropTraverse::new(root);
        for _ in iter.next() { }
    }
}

impl<Value: Debug> Debug for TSTMap<Value> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<Value> Default for TSTMap<Value> {
    /// Constructs a new, empty `TSTMap<Value>`.
    /// # Examples
    ///
    /// ```
    /// use tst::TSTMap;
    /// let mut t: TSTMap<i64> = Default::default();
    /// ```
    fn default() -> Self {
        TSTMap {
            root: Default::default(),
            size: 0,
        }
    }
}

//
// iterators section
//

/// `TSTMap` iterator.
#[derive(Clone, Default)]
pub struct Iter<'x, Value: 'x> {
    iter: Traverse<'x, Value>,
}

impl<'x, Value> Iter<'x, Value> {
    fn new(node: NodeRef<'x, Value>, min: usize, max: usize) -> Self {
        Iter {
            iter: Traverse::new(node, min, max),
        }
    }
    fn with_prefix(node: Option<&'x Node<Value>>, prefix: &str, max: usize) -> Self {
        Iter {
            iter: Traverse::with_prefix(node, prefix, max),
        }
    }
}

impl<'x, Value> Iterator for Iter<'x, Value> {
    type Item = (String, &'x Value);
    fn next(&mut self) -> Option<(String, &'x Value)> {
        self.iter.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

/// `TSTMap` mutable iterator.
#[derive(Clone, Default)]
pub struct IterMut<'x, Value: 'x> {
    iter: Traverse<'x, Value>,
}

impl<'x, Value> IterMut<'x, Value> {
    fn new(node: NodeRefMut<'x, Value>, min: usize, max: usize) -> Self {
        IterMut {
            iter: Traverse::new(node.into_immut(), min, max),
        }
    }
    fn with_prefix(ptr: Option<&'x Node<Value>>, prefix: &str, max: usize) -> Self {
        IterMut {
            iter: Traverse::with_prefix(ptr, prefix, max),
        }
    }
}

impl<'x, Value> Iterator for IterMut<'x, Value> {
    type Item = (String, &'x mut Value);
    fn next(&mut self) -> Option<(String, &'x mut Value)> {
        // just add mut, avoid copy-paste
        unsafe { mem::transmute(self.iter.next()) }
    }
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

/// `TSTMap` keys iterator
#[derive(Clone)]
pub struct KeysIter<'x, Value: 'x> {
    iter: Map<Iter<'x, Value>, fn((String, &'x Value)) -> String>,
}

impl<'x, Value:'x> Iterator for KeysIter<'x, Value> {
    type Item = String;
    fn next(&mut self) -> Option<String> { self.iter.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

/// `TSTMap` values iterator
#[derive(Clone)]
pub struct ValuesIter<'x, Value:'x> {
    iter: ValuesTraverse<'x, Value>,
}

impl<'x, Value:'x> Iterator for ValuesIter<'x, Value> {
    type Item = &'x Value;
    fn next(&mut self) -> Option<&'x Value> { self.iter.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

/// `TSTMap` wild-card iterator.
#[derive(Clone)]
pub struct WildCardIter<'x, Value: 'x> {
    iter: WildCardTraverse<'x, Value>,
}

impl<'x, Value> WildCardIter<'x, Value> {
    fn new(node: NodeRef<'x, Value>, pat: &str, max: usize) -> Self {
        WildCardIter {
            iter: WildCardTraverse::new(node, pat, max),
        }
    }
}

impl<'x, Value> Iterator for WildCardIter<'x, Value> {
    type Item = (String, &'x Value);
    fn next(&mut self) -> Option<(String, &'x Value)> { self.iter.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

/// `TSTMap` wild-card mutable iterator.
#[derive(Clone)]
pub struct WildCardIterMut<'x, Value: 'x> {
    iter: WildCardTraverse<'x, Value>,
}

impl<'x, Value> WildCardIterMut<'x, Value> {
    fn new(node: NodeRefMut<'x, Value>, pat: &str, max: usize) -> Self {
        WildCardIterMut {
            iter: WildCardTraverse::new(node.into_immut(), pat, max),
        }
    }
}

impl<'x, Value> Iterator for WildCardIterMut<'x, Value> {
    type Item = (String, &'x mut Value);
    fn next(&mut self) -> Option<(String, &'x mut Value)> { unsafe { mem::transmute(self.iter.next()) } }
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

/// `TSTMap` consuming iterator
pub struct IntoIter<Value> {
    iter: IntoTraverse<Value>,
}

impl<Value> IntoIter<Value> {
    fn new(mut tst: TSTMap<Value>) -> Self {
        let size = tst.len();
        let root = tst.root.take();
        IntoIter {
            iter: IntoTraverse::new(root, size),
        }
    }
}

impl<Value> Iterator for IntoIter<Value> {
    type Item = (String, Value);

    fn next(&mut self) -> Option<(String, Value)> {
        self.iter.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) { (self.iter.size, Some(self.iter.size)) }
}

impl<Value> ExactSizeIterator for IntoIter<Value> {
    fn len(&self) -> usize { self.iter.size }
}

//
// Entry section
//

/// A view into a single occupied location in a `TSTMap`.
pub struct OccupiedEntry<'x, Value: 'x> {
    node: &'x mut Node<Value>,
    cont_size: &'x mut usize,
}

/// A view into a single empty location in a `TSTMap`.
pub struct VacantEntry<'x, Value: 'x> {
    node: &'x mut Node<Value>,
    cont_size: &'x mut usize,
}

/// A view into a single location in a `TSTMap`, which may be vacant or occupied.
pub enum Entry<'x, Value: 'x> {
    /// A vacant Entry
    Occupied(OccupiedEntry<'x, Value>),
    /// An occupied Entry
    Vacant(VacantEntry<'x, Value>),
}

impl<'x, Value> Entry<'x, Value> {
    fn new(node: &'x mut Node<Value>, size: &'x mut usize) -> Self {
        match node.value {
            None => Vacant(VacantEntry::new(node, size)),
            Some(_) => Occupied(OccupiedEntry::new(node, size)),
        }
    }
    /// Gets a mut reference to the value in the entry or Err in case for Vacant.
    pub fn get(self) -> Result<&'x mut Value, VacantEntry<'x, Value>> {
        match self {
            Occupied(entry) => Ok(entry.into_mut()),
            Vacant(entry) => Err(entry),
        }
    }
    /// Ensures a value is in the entry by inserting the default if empty, and returns
    /// a mutable reference to the value in the entry.
    pub fn or_insert(self, default: Value) -> &'x mut Value {
        match self {
            Occupied(entry) => entry.into_mut(),
            Vacant(entry) => entry.insert(default),
        }
    }
    /// Ensures a value is in the entry by inserting the result of the default function if empty,
    /// and returns a mutable reference to the value in the entry.
    pub fn or_insert_with<F: FnOnce() -> Value>(self, default: F) -> &'x mut Value {
        match self {
            Occupied(entry) => entry.into_mut(),
            Vacant(entry) => entry.insert(default()),
        }
    }
}

impl<'x, Value> OccupiedEntry<'x, Value> {
    fn new(node: &'x mut Node<Value>, size: &'x mut usize) -> Self {
        OccupiedEntry {
            node: node,
            cont_size: size,
        }
    }
    /// Gets a reference to the value in the entry.
    pub fn get(&self) -> &Value {
        self.node.value.as_ref().unwrap()
    }
    /// Gets a mutable reference to the value in the entry.
    pub fn get_mut(&mut self) -> &mut Value {
        self.node.value.as_mut().unwrap()
    }
    /// Converts the OccupiedEntry into a mutable reference to the value in the entry
    /// with a lifetime bound to the TSTMap itself
    pub fn into_mut(self) -> &'x mut Value {
        self.node.value.as_mut().unwrap()
    }
    /// Sets the `value` of the entry, and returns the entry's old value
    pub fn insert(&mut self, value: Value) -> Value {
        self.node.replace(Some(value)).unwrap()
    }
    /// Takes the value out of the entry, and returns it
    pub fn remove(self) -> Value {
        *self.cont_size -= 1;
        self.node.replace(None).unwrap()
    }
}

impl<'x, Value> VacantEntry<'x, Value> {
    fn new(node: &'x mut Node<Value>, size: &'x mut usize) -> Self {
        VacantEntry {
            node: node,
            cont_size: size,
        }
    }
    /// Sets the `value` of the entry with the VacantEntry's key,
    /// and returns a mutable reference to it
    pub fn insert(self, value: Value) -> &'x mut Value {
        self.node.value = Some(value);
        *self.cont_size += 1;
        self.node.value.as_mut().unwrap()
    }
}

// internal tests
#[cfg(test)]
mod test {
    #[test]
    fn remove_drops_tails() {
        let mut m = tstmap! {
            "BY" => 1,
            "BYGONE" => 3,
            "BYE" => 2,
        };
        m.remove("BY");
        m.remove("BYE");
        m.remove("BYGONE");
        assert_eq!(None, m.root.ptr);
    }
}
