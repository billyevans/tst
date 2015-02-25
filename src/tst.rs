/*

public class TST<Value> {
    private int N;       // size
    private Node root;   // root of TST

    private class Node {
        private char c;                 // character
        private Node left, mid, right;  // left, middle, and right subtries
        private Value val;              // value associated with string
    }

    // return number of key-value pairs
    public int size() {
        return N;
    }

    public boolean contains(String key) {
        return get(key) != null;
    }

    public Value get(String key) {
        if (key == null) throw new NullPointerException();
        if (key.length() == 0) throw new IllegalArgumentException("key must have length >= 1");
        Node x = get(root, key, 0);
        if (x == null) return null;
        return (Value) x.val;
    }

    // return subtrie corresponding to given key
    private Node get(Node x, String key, int d) {
        if (key == null) throw new NullPointerException();
        if (key.length() == 0) throw new IllegalArgumentException("key must have length >= 1");
        if (x == null) return null;
        char c = key.charAt(d);
        if      (c < x.c)              return get(x.left,  key, d);
        else if (c > x.c)              return get(x.right, key, d);
        else if (d < key.length() - 1) return get(x.mid,   key, d+1);
        else                           return x;
    }


    public void put(String s, Value val) {
        if (!contains(s)) N++;
        root = put(root, s, val, 0);
    }

    private Node put(Node x, String s, Value val, int d) {
        char c = s.charAt(d);
        if (x == null) {
            x = new Node();
            x.c = c;
        }
        if      (c < x.c)             x.left  = put(x.left,  s, val, d);
        else if (c > x.c)             x.right = put(x.right, s, val, d);
        else if (d < s.length() - 1)  x.mid   = put(x.mid,   s, val, d+1);
        else                          x.val   = val;
        return x;
    }


    public String longestPrefixOf(String s) {
        if (s == null || s.length() == 0) return null;
        int length = 0;
        Node x = root;
        int i = 0;
        while (x != null && i < s.length()) {
            char c = s.charAt(i);
            if      (c < x.c) x = x.left;
            else if (c > x.c) x = x.right;
            else {
                i++;
                if (x.val != null) length = i;
                x = x.mid;
            }
        }
        return s.substring(0, length);
    }

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


    // return all keys matching given wilcard pattern
    public Iterable<String> wildcardMatch(String pat) {
        Queue<String> queue = new Queue<String>();
        collect(root, "", 0, pat, queue);
        return queue;
    }
 
    public void collect(Node x, String prefix, int i, String pat, Queue<String> q) {
        if (x == null) return;
        char c = pat.charAt(i);
        if (c == '.' || c < x.c) collect(x.left, prefix, i, pat, q);
        if (c == '.' || c == x.c) {
            if (i == pat.length() - 1 && x.val != null) q.enqueue(prefix + x.c);
            if (i < pat.length() - 1) collect(x.mid, prefix + x.c, i+1, pat, q);
        }
        if (c == '.' || c > x.c) collect(x.right, prefix, i, pat, q);
    }
}*/



/*
 *  Symbol table with string keys, implemented using a ternary search
 *  trie (TST).
 */


#[allow(dead_code)]

use std::ptr;
use std::boxed;

struct Node<Value> {
    left: *mut Node<Value>,
    mid: *mut Node<Value>,
    right: *mut Node<Value>,
    val: *mut Value,
    c: char
}

impl<Value> Node<Value> {
    fn new() -> Node<Value> {
        Node {
            left : ptr::null_mut(),
            mid : ptr::null_mut(),
            right : ptr::null_mut(),
            val : ptr::null_mut(),
            c : '\0'
        }
    }
}

pub struct TST<Value: Clone> {
    root: *mut Node<Value>,
    size: usize
}

impl<Value: Clone> TST<Value> {
    pub fn new() -> TST<Value> {
        TST { root : ptr::null_mut(), size : 0 }
    }

    pub fn len(&self) -> usize { self.size }

    pub fn insert(&mut self, key: &str, val: &Value) {
        unsafe { self.root = self.insert_(self.root, key.chars().collect(), val, 0); }
    }

    unsafe fn insert_(&mut self, mut x: *mut Node<Value>, key: Vec<char>, val: &Value, d: usize) -> *mut Node<Value> {
        let c = key[d];
        if x.is_null() {
            x = boxed::into_raw(Box::new(Node::<Value>::new()));
            (*x).c = c;
        }
        if c < (*x).c {
            (*x).left  = self.insert_((*x).left, key, val, d);
        }
        else if c > (*x).c {
            (*x).right = self.insert_((*x).right, key, val, d);
        }
        else if d+1 < key.len() {
            (*x).mid = self.insert_((*x).mid, key, val, d+1);
        }
        else {
            if (*x).val.is_null() {
                self.size += 1;
            }
            (*x).val = boxed::into_raw(Box::new(val.clone()));
        }
        return x;
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        unsafe {
            match self.get_(self.root, key.chars().collect(), 0) {
                Some(x) => Some(&(*x.val)),
                None => None
            }
        }
    }
    // return subtrie corresponding to given key
    unsafe fn get_(&self, x: *mut Node<Value>, key: Vec<char>, d: usize) -> Option<&Node<Value> > {
        if x.is_null() { return None; }
        if d >= key.len() { return None; }
        let c = key[d];
        if c < (*x).c {
            return self.get_((*x).left, key, d);
        }
        else if c > (*x).c {
            return self.get_((*x).right, key, d);
        }
        else if d + 1< key.len() {
            return self.get_((*x).mid, key, d+1);
        }
        else {
            return Some(&(*x));
        }
    }
    pub fn contains_key(&self, key: &str) -> bool {
        !self.get(key).is_none()
    }
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}
