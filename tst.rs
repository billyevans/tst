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
			left : ptr::mut_null(),
			mid : ptr::mut_null(),
			right : ptr::mut_null(),
			val : ptr::mut_null(),
			c : '\0'
		}
	}
}

pub struct TST<Value> {
	root: *mut Node<Value>,
	size: int
}

impl<Value> TST<Value> {
	pub fn new() -> TST<Value> {
		TST { root : ptr::mut_null(), size : 0 }
	}

	pub fn size(&self) -> int { self.size }

	pub fn put(&self, key: &str, val: Value) {
		self.root = self.put_(self.root, key, val, 0);
	}

	fn put_(&self, x : *mut Node<Value>, key: &str, val: Value, d: int) -> *mut Node<Value> {
		let c = key[d];
		if (x.is_null()) {
			x = box Node::new();
			x.c = c;
		}
		if (c < x.c) {
			x.left  = self.put_(x.left, key, val, d);
		}
		else if (c > x.c) {
			x.right = self.put_(x.right, key, val, d);
		}
		else if (d < key.len() - 1) {
			x.mid = self.put_(x.mid, key, val, d+1);
		}
		else {
			if (x.val == ptr::null()) {
				self.size = self.size + 1;
			}
			x.val   = val;
		}
		x;
	}

	/*
	pub fn Value get(String key) {
		if (key == null) throw new NullPointerException();
		if (key.length() == 0) throw new IllegalArgumentException("key must have length >= 1");
		Node x = get(root, key, 0);
		if (x == null) return null;
		return (Value) x.val;
	}*/
}
