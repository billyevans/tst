# tst

[![Build Status](https://travis-ci.org/billyevans/tst.svg?branch=master)](https://travis-ci.org/billyevans/tst)
[![Coverage Status](https://coveralls.io/repos/billyevans/tst/badge.svg?branch=master)](https://coveralls.io/r/billyevans/tst?branch=master)
[![crates.io](http://meritbadge.herokuapp.com/tst)](https://crates.io/crates/tst)
[![API](https://docs.rs/tst/badge.svg)](https://docs.rs/tst/)

Ternary search tree collection in rust with similar API to std::collections as it possible.

Ternary search tree is a type of trie (sometimes called a prefix tree) where nodes are arranged in a manner similar to a binary search tree, but with up to three children rather than the binary tree's limit of two. Like other prefix trees, a ternary search tree can be used as an associative map structure with the ability for incremental string search. However, ternary search trees are more space efficient compared to standard prefix trees, at the cost of speed. Common applications for ternary search trees include spell-checking and auto-completion.
TSTMap and TSTSet structures for map and set like usage.

Documentation is available at http://billyevans.github.io/tst/tst

It has special methods:
- wildcard_iter/wildcard_iter_mut - get iterator by wildcard
- prefix_iter/prefix_iter_mut - get iterator by prefix
- longest_prefix - get longest prefix

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
tst = "0.10.*"
```

## Quick Start
```rust
#[macro_use]
extern crate tst;
use tst::TSTMap;

let m = tstmap! {
    "first" =>  1,
    "second" => 2,
    "firstthird" => 3,
    "firstsecond" => 12,
    "xirst" => -13,
};

// iterate
for (key, value) in m.iter() {
    println!("{}: {}", key, value);
}
assert_eq!(Some(&1), m.get("first"));
assert_eq!(5, m.len());

// calculating longest prefix
assert_eq!("firstsecond", m.longest_prefix("firstsecondthird"));

// get values with common prefix
for (key, value) in m.prefix_iter("first") {
    println!("{}: {}", key, value);
}

// get sum by wildcard iterator
assert_eq!(-12, m.wildcard_iter(".irst").fold(0, |sum, (_, val)| sum + val));
```

### Iterating over keys with wildcard
```rust
#[macro_use]
extern crate tst;
use tst::TSTMap;

let m = tstmap! {
    "ac" => 1,
    "bd" => 2,
    "cc" => 3,
};

for (k, v) in m.wildcard_iter(".c") {
    println!("{} -> {}", k, v);
}
```

### Itereting over keys with common prefix
```rust
#[macro_use]
extern crate tst;
use tst::TSTMap;

let m = tstmap! {
    "abc" => 1,
    "abcd" => 1,
    "abce" => 1,
    "abca" => 1,
    "zxd" => 1,
    "add" => 1,
    "abcdef" => 1,
};

for (key, value) in m.prefix_iter("abc") {
    println!("{}: {}", key, value);
}

```

### Search for longest prefix in the tree
```rust
#[macro_use]
extern crate tst;
use tst::TSTMap;

let m = tstmap! {
    "abc" => 1,
    "abcd" => 1,
    "abce" => 1,
    "abca" => 1,
    "zxd" => 1,
    "add" => 1,
    "abcdef" => 1,
};

assert_eq!("abcd", m.longest_prefix("abcde"));
```

### Implementation details

https://en.wikipedia.org/wiki/Ternary_search_tree

# License

TST is distributed under the terms of the MIT license. 

See [LICENSE-MIT](LICENSE-MIT), and [COPYRIGHT](COPYRIGHT) for details.
