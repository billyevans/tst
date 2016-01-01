use tst_map::TSTMap;

/// A set based on a TSTMap.
#[derive(Clone, PartialEq, Eq)]
pub struct TSTSet {
    map: TSTMap<()>,
}

impl TSTSet {
    /// Makes a new empty TSTSet.
    ///
    /// # Examples
    ///
    /// ```
    /// use tst::TSTSet;
    ///
    /// let mut s: TSTSet = TSTSet::new();
    /// ```
    pub fn new() -> TSTSet {
        TSTSet { map: TSTMap::new() }
    }

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

    /// Returns `true` if the set contains a value.
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
}
