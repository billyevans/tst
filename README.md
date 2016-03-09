# tst

[![Build Status](https://travis-ci.org/billyevans/tst.svg?branch=master)](https://travis-ci.org/billyevans/tst)
[![Coverage Status](https://coveralls.io/repos/billyevans/tst/badge.svg?branch=master)](https://coveralls.io/r/billyevans/tst?branch=master)
[![crates.io](http://meritbadge.herokuapp.com/tst)](https://crates.io/crates/tst)

Ternary search trie collection in rust with similar API to std::collections as it possible.
Now it's first simplest implementation, that's the reason why it's recursive.

Documentation is available at http://billyevans.github.io/tst/tst

It has special methods:
- wildcard_iter - get iterator by wildcard
- prefix_iter/prefix_iter_mut - get iterator by prefix
- longest_prefix - get longest prefix
