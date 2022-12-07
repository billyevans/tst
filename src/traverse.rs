use std::cmp::Ordering;
use std::mem;

use super::node::{Node, NodeRef, NodeRefMut, BoxedNodeRefMut, BoxedNode};

enum CompareResult<Handle> {
    GoLeftOrRight(Handle),
    GoDown(Handle),
    NotFound,
}

#[derive(Clone)]
struct Trace<Ref> {
    stack: Vec<Ref>,
}

impl<Ref> Trace<Ref> {
    fn new(size: usize) -> Self {
        Trace {
            stack: Vec::with_capacity(size),
        }
    }
    fn push(&mut self, entry: Ref) {
        self.stack.push(entry)
    }
    fn pop(&mut self) -> Option<Ref> {
        self.stack.pop()
    }
}

impl<Ref> Default for Trace<Ref> {
    fn default() -> Self {
        Trace {
            stack: vec![],
        }
    }
}

#[derive(Clone)]
enum TraverseEntry<NodeRef, ValueRef> {
    Node(NodeRef),
    Value(ValueRef)
}

#[derive(Clone)]
pub struct Traverse<'x, Value: 'x> {
    stack: Trace<TraverseEntry<(String, NodeRef<'x, Value>), (String, &'x Value)>>,
    min_size: usize,
    max_size: usize,
}

impl<'x, Value> Traverse<'x, Value> {
    pub fn new(node: NodeRef<'x, Value>, min: usize, max: usize) -> Self {
        Traverse {
            stack: Trace {
                stack: vec![TraverseEntry::Node(("".to_string(), node))],
            },
            min_size: min,
            max_size: max,
        }
    }

    pub fn with_prefix(node: Option<&'x Node<Value>>, prefix: &str, max: usize) -> Self {
        let mut iter: Traverse<Value> = Default::default();
        match node {
            None => (),
            Some(ptr) => {
                iter.max_size = max;
                if ptr.value.is_some() {
                    iter.min_size += 1;
                    iter.stack.push(TraverseEntry::Value((prefix.to_string(), ptr.value.as_ref().unwrap())));
                }
                if ptr.eq.ptr.is_some() {
                    iter.stack.push(TraverseEntry::Node((prefix.to_string(), ptr.eq.as_ref())));
                }
            }
        }
        iter
    }

    pub fn next(&mut self) -> Option<(String, &'x Value)> {
        while let Some(entry) = self.stack.pop() {
            match entry {
                TraverseEntry::Value((prefix, value)) => {
                    if self.min_size == self.max_size {
                        self.min_size -= 1;
                    }
                    self.max_size -= 1;
                    return Some((prefix, value));
                }
                TraverseEntry::Node((prefix, node)) => {
                    match node.as_option() {
                        None => {}
                        Some(cur) => {
                            if cur.gt.is_some() {
                                self.stack.push(TraverseEntry::Node((prefix.clone(), cur.gt.as_ref())));
                            }
                            if cur.eq.is_some() {
                                let mut new_prefix = String::with_capacity(prefix.len() + 1);
                                new_prefix.push_str(&prefix);
                                new_prefix.push(cur.c);
                                self.stack.push(TraverseEntry::Node((new_prefix, cur.eq.as_ref())));
                            }
                            if cur.value.is_some() {
                                let mut new_prefix = String::with_capacity(prefix.len() + 1);
                                new_prefix.push_str(&prefix);
                                new_prefix.push(cur.c);
                                self.stack.push(TraverseEntry::Value((new_prefix, cur.value.as_ref().unwrap())));
                            }
                            if cur.lt.is_some() {
                                self.stack.push(TraverseEntry::Node((prefix, cur.lt.as_ref())));
                            }
                        }
                    }
                }
            }
        }
        None
    }

    pub fn size_hint(&self) -> (usize, Option<usize>) {
        (self.min_size, Some(self.max_size))
    }
}

impl<'x, Value> Default for Traverse<'x, Value> {
    fn default() -> Self {
        Traverse {
            stack: Default::default(),
            min_size: 0,
            max_size: 0,
        }
    }
}

pub struct IntoTraverse<Value> {
    stack: Trace<TraverseEntry<(String, Option<Box<Node<Value>>>), (String, Value)>>,
    pub size: usize,
}

impl<Value> IntoTraverse<Value> {
    pub fn new(node: Option<Box<Node<Value>>>, size: usize) -> Self {
        IntoTraverse {
            stack: Trace {
                stack: vec![TraverseEntry::Node(("".to_string(), node))],
            },
            size,
        }
    }

    pub fn next(&mut self) -> Option<(String, Value)> {
        while let Some(entry) = self.stack.pop() {
            match entry {
                TraverseEntry::Value((prefix, value)) => {
                    self.size -= 1;
                    return Some((prefix, value));
                }
                TraverseEntry::Node((prefix, mut node)) => {
                    match node {
                        None => {}
                        Some(ref mut cur) => {
                            if cur.gt.is_some() {
                                self.stack.push(TraverseEntry::Node((prefix.clone(), cur.gt.take())));
                            }
                            if cur.eq.is_some() {
                                let mut new_prefix = String::with_capacity(prefix.len() + 1);
                                new_prefix.push_str(&prefix);
                                new_prefix.push(cur.c);
                                self.stack.push(TraverseEntry::Node((new_prefix, cur.eq.take())));
                            }
                            if cur.value.is_some() {
                                let mut new_prefix = String::with_capacity(prefix.len() + 1);
                                new_prefix.push_str(&prefix);
                                new_prefix.push(cur.c);
                                self.stack.push(TraverseEntry::Value((new_prefix, cur.value.take().unwrap())));
                            }
                            if cur.lt.is_some() {
                                self.stack.push(TraverseEntry::Node((prefix.clone(), cur.lt.take())));
                            }
                        }
                    }
                }
            }
        }
        None
    }
}

pub struct DropTraverse<Value> {
    stack: Trace<TraverseEntry<Option<Box<Node<Value>>>, Value>>,
}

impl<Value> DropTraverse<Value> {
    pub fn new(node: Option<Box<Node<Value>>>) -> Self {
        DropTraverse {
            stack: Trace {
                stack: vec![TraverseEntry::Node(node)],
            },
        }
    }

    pub fn next(&mut self) -> Option<Value> {
        while let Some(entry) = self.stack.pop() {
            match entry {
                TraverseEntry::Value(value) => {
                    return Some(value);
                }
                TraverseEntry::Node(mut node) => {
                    match node {
                        None => {}
                        Some(ref mut cur) => {
                            if cur.gt.is_some() {
                                self.stack.push(TraverseEntry::Node(cur.gt.take()));
                            }
                            if cur.eq.is_some() {
                                self.stack.push(TraverseEntry::Node(cur.eq.take()));
                            }
                            if cur.value.is_some() {
                                self.stack.push(TraverseEntry::Value(cur.value.take().unwrap()));
                            }
                            if cur.lt.is_some() {
                                self.stack.push(TraverseEntry::Node(cur.lt.take()));
                            }
                        }
                    }
                }
            }
        }
        None
    }
}

#[derive(Clone)]
pub struct ValuesTraverse<'x, Value: 'x> {
    stack: Trace<TraverseEntry<NodeRef<'x, Value>, &'x Value>>,
    min_size: usize,
    max_size: usize,
}

impl<'x, Value> ValuesTraverse<'x, Value> {
    pub fn new(node: NodeRef<'x, Value>, min: usize, max: usize) -> Self {
        ValuesTraverse {
            stack: Trace {
                stack: vec![TraverseEntry::Node(node)],
            },
            min_size: min,
            max_size: max,
        }
    }

    pub fn next(&mut self) -> Option<&'x Value> {
        while let Some(entry) = self.stack.pop() {
            match entry {
                TraverseEntry::Value(value) => {
                    if self.min_size == self.max_size {
                        self.min_size -= 1;
                    }
                    self.max_size -= 1;
                    return Some(value);
                }
                TraverseEntry::Node(node) => {
                    match node.as_option() {
                        None => {}
                        Some(cur) => {
                            if cur.gt.is_some() {
                                self.stack.push(TraverseEntry::Node(cur.gt.as_ref()));
                            }
                            if cur.eq.is_some() {
                                self.stack.push(TraverseEntry::Node(cur.eq.as_ref()));
                            }
                            if cur.value.is_some() {
                                self.stack.push(TraverseEntry::Value(cur.value.as_ref().unwrap()));
                            }
                            if cur.lt.is_some() {
                                self.stack.push(TraverseEntry::Node(cur.lt.as_ref()));
                            }
                        }
                    }
                }
            }
        }
        None
    }

    pub fn size_hint(&self) -> (usize, Option<usize>) {
        (self.min_size, Some(self.max_size))
    }
}

#[derive(Clone)]
pub struct WildCardTraverse<'x, Value: 'x> {
    stack: Trace<TraverseEntry<(String, NodeRef<'x, Value>, usize), (String, &'x Value)>>,
    max_size: usize,
    pat: Vec<char>,
}

impl<'x, Value> WildCardTraverse<'x, Value> {
    pub fn new(node: NodeRef<'x, Value>, pat: &str, max: usize) -> Self {
        WildCardTraverse {
            stack: Trace{
                stack: vec![TraverseEntry::Node(("".to_string(), node, 0))],
            },
            max_size: max,
            pat: pat.chars().collect(),
        }
    }

    pub fn next(&mut self) -> Option<(String, &'x Value)> {
        while let Some(entry) = self.stack.pop() {
            match entry {
                TraverseEntry::Value((prefix, value)) => {
                    self.max_size -= 1;
                    return Some((prefix, value));
                }
                TraverseEntry::Node((prefix, node, idx)) => {
                    match node.as_option() {
                        None => {}
                        Some(cur) => {
                            let ch = self.pat[idx];
                            if (ch == '.' || ch > cur.c) && cur.gt.is_some() {
                                self.stack.push(TraverseEntry::Node((prefix.clone(), cur.gt.as_ref(), idx)));
                            }
                            if ch == '.' || ch == cur.c {
                                if idx+1 < self.pat.len() && cur.eq.is_some() {
                                    let mut new_prefix = String::with_capacity(prefix.len() + 1);
                                    new_prefix.push_str(&prefix);
                                    new_prefix.push(cur.c);
                                    self.stack.push(TraverseEntry::Node((new_prefix, cur.eq.as_ref(), idx+1)));
                                }

                                if idx+1 == self.pat.len() && cur.value.is_some() {
                                    let mut new_prefix = String::with_capacity(prefix.len() + 1);
                                    new_prefix.push_str(&prefix);
                                    new_prefix.push(cur.c);
                                    self.stack.push(TraverseEntry::Value((new_prefix, cur.value.as_ref().unwrap())));
                                }
                            }
                            if (ch == '.' || ch < cur.c) && cur.lt.is_some() {
                                self.stack.push(TraverseEntry::Node((prefix, cur.lt.as_ref(), idx)));
                            }
                        }
                    }
                }
            }
        }
        None
    }
    pub fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.max_size))
    }
}

fn lookup_next<'x, Value>(node: &NodeRef<'x, Value>, ch: char) -> CompareResult<NodeRef<'x, Value>> {
    match node.as_option() {
        None => CompareResult::NotFound,
        Some(cur) => {
            match ch.cmp(&cur.c) {
                Ordering::Less => CompareResult::GoLeftOrRight(cur.lt.as_ref()),
                Ordering::Greater => CompareResult::GoLeftOrRight(cur.gt.as_ref()),
                Ordering::Equal => CompareResult::GoDown(cur.eq.as_ref()),
            }
        }
    }
}

fn lookup_next_mut<'x, Value>(node: &BoxedNodeRefMut<'x, Value>, ch: char) -> CompareResult<BoxedNodeRefMut<'x, Value>> {
    match node.as_mut().ptr {
        None => CompareResult::NotFound,
        Some(ref mut cur) => {
            match ch.cmp(&cur.c) {
                Ordering::Less => CompareResult::GoLeftOrRight(cur.lt.as_mut()),
                Ordering::Greater => CompareResult::GoLeftOrRight(cur.gt.as_mut()),
                Ordering::Equal => CompareResult::GoDown(cur.eq.as_mut()),
            }
        }
    }
}

pub fn search<'x, Value>(mut node: NodeRef<'x, Value>, key: &str) ->
        Option<&'x Node<Value>>
{
    let mut last = Default::default();

    for ch in key.chars() {
        let mut go_next = false;
        while !go_next {
            node = match lookup_next(&node, ch) {
                CompareResult::GoLeftOrRight(next) => next,
                CompareResult::GoDown(next) => {
                    go_next = true;
                    last = node;
                    next
                },
                CompareResult::NotFound => {
                    return None;
                },
            }
        }
    }
    last.as_option()
}

pub fn insert<'x, Value>(mut node: BoxedNodeRefMut<'x, Value>, key: &str) -> &'x mut Node<Value> {
    let mut last = Default::default();

    for ch in key.chars() {
        let mut go_next = false;
        while !go_next {
            node = match lookup_next_mut(&node, ch) {
                CompareResult::GoLeftOrRight(next) => next,
                CompareResult::GoDown(next) => {
                    go_next = true;
                    last = node;
                    next
                },
                CompareResult::NotFound => {
                    node.assign(BoxedNode::new(ch));
                    node
                },
            }
        }
    }
    last.as_node_ref()
}

pub fn search_mut<'x, Value>(node: NodeRefMut<'x, Value>, key: &str) ->
        Option<&'x mut Node<Value>>
{
    unsafe { mem::transmute(search(node.into_immut(), key)) }
}

pub fn longest_prefix<'x, Value>(mut node: NodeRef<'x, Value>, pref: &'x str) -> &'x str {
    let mut length: usize = 0;
    let mut i: usize = 0;
    for ch in pref.chars() {
        let mut go_next = false;
        while !go_next {
            node = match lookup_next(&node, ch) {
                CompareResult::GoLeftOrRight(next) => next,
                CompareResult::GoDown(next) => {
                    go_next = true;
                    i += 1;
                    if node.is_value() { length = i; }
                    next
                },
                CompareResult::NotFound => {
                    return &pref[..length];
                },
            }
        }
    }
    &pref[..length]
}

pub fn remove<Value>(mut node: BoxedNodeRefMut<Value>, key: &str) -> Option<Value> {
    let mut stack = Trace::<BoxedNodeRefMut<Value>>::new(key.len());
    let mut ptr = None;

    for ch in key.chars() {
        let mut go_next = false;
        while !go_next {
            stack.push(node.clone());
            node = match lookup_next_mut(&node, ch) {
                CompareResult::GoLeftOrRight(next) => next,
                CompareResult::GoDown(next) => {
                    go_next = true;
                    ptr = Some(node.as_node_ref());
                    next
                },
                CompareResult::NotFound => {
                    return None;
                },
            }

        }
    }
    let ret = match ptr {
        None => None,
        Some(ptr) => ptr.value.take(),
    };
    // cut the tail
    if ret.is_some() {
        while let Some(mut node_to_drop) = stack.pop() {
            let ptr = node_to_drop.as_node_ref();
            if !ptr.is_leaf() {
                break;
            }
            node_to_drop.assign(Default::default());
        }
    }
    ret
}
