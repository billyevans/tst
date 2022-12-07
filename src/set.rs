use crate::tst_map::{self, TSTMap};
use std::fmt::{self, Debug};
use std::iter::{Map, FromIterator};

/// A set based on a `TSTMap`.
#[derive(Clone, PartialEq, Eq)]
pub struct TSTSet {
    map: TSTMap<()>,
}

/// An iterator over a `TSTSet`'s items.
#[derive(Clone)]
pub struct Iter<'a> {
    iter: Map<tst_map::Iter<'a, ()>, fn((String, &'a ())) -> String>
}

/// An owning iterator over a `TSTSet`'s items.
pub struct IntoIter {
    iter: Map<tst_map::IntoIter<()>, fn((String, ())) -> String>
}

/// `TSTSet` wild-card iterator.
#[derive(Clone)]
pub struct WildCardIter<'a> {
    iter: Map<tst_map::WildCardIter<'a, ()>, fn( (String, &'a () )) -> String>,
}

impl TSTSet {
    /// Makes a new empty `TSTSet`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::TSTSet;
    ///
    /// let mut s: TSTSet = TSTSet::new();
    /// ```
    pub fn new() -> Self { Default::default() }

    /// Returns the number of elements in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::TSTSet;
    ///
    /// let mut s: TSTSet = TSTSet::new();
    /// assert_eq!(s.len(), 0);
    /// s.insert("xxx");
    /// assert_eq!(s.len(), 1);
    /// ```
    pub fn len(&self) -> usize { self.map.len() }

    /// Returns true if the set contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::TSTSet;
    ///
    /// let mut s: TSTSet = TSTSet::new();
    /// assert!(s.is_empty());
    /// s.insert("yyyx");
    /// assert!(!s.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    /// Clears the set, removing all values.
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::TSTSet;
    ///
    /// let mut s: TSTSet = TSTSet::new();
    /// s.insert("abc");
    /// s.insert("abd");
    /// s.clear();
    ///
    /// assert!(s.is_empty());
    /// assert!(!s.contains("abc"));
    /// ```
    pub fn clear(&mut self) {
        self.map.clear()
    }

    /// Returns `true` if the set contains a `key`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::TSTSet;
    ///
    /// let mut s: TSTSet = TSTSet::new();
    /// s.insert("abc");
    /// assert!(!s.contains("ab"));
    /// assert!(s.contains("abc"));
    /// ```
    pub fn contains(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    /// Adds a value to the set.
    ///
    /// If the set did not have a value present, `true` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::TSTSet;
    ///
    /// let mut s: TSTSet = TSTSet::new();
    ///
    /// assert!(s.insert("abcd"));
    /// assert!(!s.insert("abcd"));
    /// assert_eq!(s.len(), 1);
    /// ```
    pub fn insert(&mut self, key: &str) -> bool {
        self.map.insert(key, ()).is_none()
    }

    /// Removes a value from the set. Returns `true` if the value was
    /// present in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::TSTSet;
    ///
    /// let mut s: TSTSet = TSTSet::new();
    ///
    /// s.insert("acde");
    /// assert!(s.remove("acde"));
    /// assert!(!s.remove("acde"));
    /// ```
    pub fn remove(&mut self, key: &str) -> bool {
        self.map.remove(key).is_some()
    }

    /// Gets an iterator over the TSTSet's contents.
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::TSTSet;
    ///
    /// let mut s: TSTSet = TSTSet::new();
    /// s.insert("abc");
    /// s.insert("bde");
    /// s.insert("cfgx");
    /// for x in s.iter() {
    ///     println!("{}", x);
    /// }
    /// ```
    pub fn iter(&self) -> Iter {
        fn first<A, B>((a, _): (A, B)) -> A { a }
        Iter { iter: self.map.iter().map(first) }

        //Iter { iter: self.map.keys() }
    }

    /// An iterator returning all nodes matching wildcard pattern.
    /// Iterator element type is (String)
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::TSTSet;
    ///
    /// let mut s = TSTSet::new();
    /// s.insert("a");
    /// s.insert("b");
    /// s.insert("c");
    ///
    /// for x in s.wildcard_iter(".") {
    ///     println!("{}", x);
    /// }
    /// ```
    pub fn wildcard_iter(&self, pat: &str) -> WildCardIter {
        fn first<A, B>((a, _): (A, B)) -> A { a }
        WildCardIter { iter: self.map.wildcard_iter(pat).map(first) }
    }

    /// Method returns longest prefix in the TSTSet.
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::TSTSet;
    /// let mut set = TSTSet::new();
    /// set.insert("abc");
    /// set.insert("abcd");
    /// set.insert("abce");
    /// set.insert("abca");
    /// set.insert("zxd");
    /// set.insert("add");
    /// set.insert("abcdef");
    ///
    /// assert_eq!("abcd", set.longest_prefix("abcde"));
    /// ```
    pub fn longest_prefix<'a>(&self, pref: &'a str) -> &'a str {
        self.map.longest_prefix(pref)
    }

    /// Method returns iterator over all values with common prefix in the TSTSet.
    /// # Examples
    ///
    /// ```
    /// use tst::TSTSet;
    /// let mut set = TSTSet::new();
    /// set.insert("abc");
    /// set.insert("abcd");
    /// set.insert("abce");
    /// set.insert("abca");
    /// set.insert("zxd");
    /// set.insert("add");
    /// set.insert("abcdef");
    ///
    /// for key in set.prefix_iter("abc") {
    ///     println!("{}", key);
    /// }
    ///
    /// let first_key = set.iter().next().unwrap();
    /// assert_eq!("abc".to_string(), first_key);
    /// ```
    pub fn prefix_iter(&self, pref: &str) -> Iter {
        fn first<A, B>((a, _): (A, B)) -> A { a }
        Iter { iter: self.map.prefix_iter(pref).map(first) }
    }
}

impl IntoIterator for TSTSet {
    type Item = String;
    type IntoIter = IntoIter;

    /// Creates a consuming iterator, that is, one that moves each key-value
    /// pair out of the TSTMap in arbitrary order. The TSTMap cannot be used after
    /// calling this.
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::TSTSet;
    ///
    /// let mut set = TSTSet::new();
    /// set.insert("a");
    /// set.insert("b");
    /// set.insert("c");
    ///
    /// let vec: Vec<String> = set.into_iter().collect();
    /// ```
    fn into_iter(self) -> IntoIter {
        fn first<A, B>((a, _): (A, B)) -> A { a }
        IntoIter { iter: self.map.into_iter().map(first) }
    }
}

impl<'x> FromIterator<&'x str> for TSTSet {
    fn from_iter<I: IntoIterator<Item = &'x str>>(iter: I) -> TSTSet {
        let mut set = TSTSet::new();
        for item in iter {
            set.insert(item);
        }
        set
    }
}

impl<'x> Extend<&'x str> for TSTSet {
    #[inline]
    fn extend<I: IntoIterator<Item=&'x str>>(&mut self, iter: I) {
        for k in iter {
            self.insert(k);
        }
    }
}

impl Default for TSTSet {
    /// Makes a new empty `TSTSet`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::TSTSet;
    ///
    /// let mut s: TSTSet = TSTSet::new();
    /// ```

    fn default() -> Self {
        TSTSet { map: Default::default() }
    }
}

impl Debug for TSTSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> { self.iter.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

impl Iterator for IntoIter {
    type Item = String;

    fn next(&mut self) -> Option<String> { self.iter.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

impl ExactSizeIterator for IntoIter {
    fn len(&self) -> usize { self.iter.len() }
}

impl<'a> Iterator for WildCardIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> { self.iter.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}
