# tst

[![Build Status](https://travis-ci.org/billyevans/tst.svg?branch=master)](https://travis-ci.org/billyevans/tst)

Ternary search tree collection in rust with similar API to std::collections as it possible.
Now it's first simplest implementation, that's the reason why it's reccursive.

It has special methods:
- wildcard_iter() - get iterator by wildcard
- prefix_iter()/prefix_iter_mut() - get iterator by prefix
- longest_prefix() - get longest prefix

TODO:
- into_iter
- split into map and set
- make non-reccursive
- remove key.chars().collect()
- add more docs
