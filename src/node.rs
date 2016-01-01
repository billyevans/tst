use std::mem;
use std::str::Chars;
use std::fmt::{self, Debug};

#[derive(Clone, PartialEq, Eq)]
pub struct Node<V> {
    pub lt: Option<Box<Node<V>>>,
    pub eq: Option<Box<Node<V>>>,
    pub gt: Option<Box<Node<V>>>,
    pub val: Option<V>,
    pub c: char
}

impl<V> Node<V> {
    fn new(c: char) -> Node<V> {
        Node {
            lt: None,
            eq: None,
            gt: None,
            val: None,
            c: c
        }
    }

    pub fn insert_node<'a>(node: &'a mut Option<Box<Node<V>>>, op_ch: Option<char>, mut iter: Chars) -> &'a mut Box<Node<V>> {
        match op_ch {
            None => unreachable!(),
            Some(ch) => {
                match *node {
                    None => {
                        *node = Some(Box::new(Node::new(ch)));
                        Node::insert_node(node, op_ch, iter)
                    }
                    Some(ref mut cur) => {
                        if ch < cur.c {
                            Node::insert_node(&mut cur.lt, op_ch, iter)
                        }
                        else if ch > cur.c {
                            Node::insert_node(&mut cur.gt, op_ch, iter)
                        }
                        else if iter.size_hint().0 > 0 {
                            Node::insert_node(&mut cur.eq, iter.next(), iter)
                        }
                        else {
                            cur
                        }
                    }
                }
            }
        }
    }

    pub fn get<'a>(node: &'a Option<Box<Node<V>>>, key: &str) ->
            Option<&'a Option<Box<Node<V>>>>
    {
        let mut iter = key.chars();
        let mut cur_node = node;

        while let Some(ch) = iter.next() {
            let mut go_next = false;
            while go_next == false {
                cur_node = match *cur_node {
                    None => { return None; }
                    Some(ref cur) => {
                        if ch < cur.c {
                            &cur.lt
                        }
                        else if ch > cur.c {
                            &cur.gt
                        }
                        else if iter.size_hint().0 > 0 {
                            go_next = true;
                            &cur.eq
                        }
                        else {
                            return Some(cur_node);
                        }
                    }
                }
            }
        }
        None
    }

    pub fn get_mut<'a>(node: &'a mut Option<Box<Node<V>>>, key: &str) ->
            Option<&'a mut Option<Box<Node<V>>>>
    {
        unsafe { mem::transmute(Node::get(node, key)) }
    }

    pub fn is_leaf(&self) -> bool {
        self.lt.is_none() && self.gt.is_none() && self.eq.is_none() && self.val.is_none()
    }

    pub fn remove(node: &mut Option<Box<Node<V>>>, op_ch: Option<char>, mut iter: Chars) -> Option<V> {
        match op_ch {
            None => None,
            Some(ch) => {
                match *node {
                    None => None,
                    Some(ref mut cur) => {
                        if ch < cur.c {
                            let ret = Node::remove(&mut cur.lt, op_ch, iter);
                            if ret.is_some() && cur.lt.as_ref().unwrap().is_leaf() {
                                mem::replace(&mut cur.lt, None);
                            }
                            ret
                        }
                        else if ch > cur.c {
                            let ret = Node::remove(&mut cur.gt, op_ch, iter);
                            if ret.is_some() && cur.gt.as_ref().unwrap().is_leaf() {
                                mem::replace(&mut cur.gt, None);
                            }
                            ret
                        }
                        else if iter.size_hint().0 > 0 {
                            let ret = Node::remove(&mut cur.eq, iter.next(), iter);
                            if ret.is_some() && cur.eq.as_ref().unwrap().is_leaf() {
                                mem::replace(&mut cur.eq, None);
                            }
                            ret
                        }
                        else {
                            match cur.val {
                                None => None,
                                Some(_) => mem::replace(&mut cur.val, None)
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn replace(&mut self, val: Option<V>) -> Option<V> {
        mem::replace(&mut self.val, val)
    }

}

impl<V: Debug> Debug for Node<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{{\n"));
        try!(write!(f, "lt = {:?}, eq = {:?}, gt = {:?}, val = {:?}, c = {:?}",
            self.lt, self.eq, self.gt, self.val, self.c));
        (write!(f, "}}\n"))
    }
}
