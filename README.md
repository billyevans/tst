# tst

[![Build Status](https://travis-ci.org/billyevans/tst.svg?branch=master)](https://travis-ci.org/billyevans/tst)
[![Coverage Status](https://coveralls.io/repos/billyevans/tst/badge.svg?branch=master)](https://coveralls.io/r/billyevans/tst?branch=master)
[![crates.io](http://meritbadge.herokuapp.com/tst)](https://crates.io/crates/tst)

Ternary search trie collection in rust with similar API to std::collections as it possible.

Documentation is available at http://billyevans.github.io/tst/tst

It has special methods:
- wildcard_iter/wildcard_iter_mut - get iterator by wildcard
- prefix_iter/prefix_iter_mut - get iterator by prefix
- longest_prefix - get longest prefix



```rust
#[macro_use] extern crate tst;

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
