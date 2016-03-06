use std::mem;
use std::fmt::{self, Debug};
use std::default::Default;
use std::ops::Deref;
use core::marker::PhantomData;
use core::ptr;

#[derive(Clone, PartialEq, Eq)]
pub struct Node<Value> {
    pub lt: BoxedNode<Value>,
    pub eq: BoxedNode<Value>,
    pub gt: BoxedNode<Value>,
    pub value: Option<Value>,
    pub c: char,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoxedNode<Value> {
    pub ptr: Option<Box<Node<Value>>>,
}

pub struct NodeRef<MutType, Value> {
    node: *const Node<Value>,
    _marker: PhantomData<MutType>,
}

pub struct NodeRefMut<'x, Value: 'x> {
    node: *mut BoxedNode<Value>,
    _marker: PhantomData<&'x Value>,
}

impl<Value> Default for BoxedNode<Value> {
    fn default() -> BoxedNode<Value> {
        BoxedNode {
            ptr: None,
        }
    }
}

impl<Value> BoxedNode<Value> {
    pub fn new(ch: char) -> BoxedNode<Value> {
        BoxedNode {
            ptr: Some(Box::new(Node::new(ch))),
        }
    }

    fn as_ptr(&self) -> *const Node<Value> {
        match self.ptr {
            Some(ref ptr) => {
                &**ptr as *const Node<Value>
            },
            None => {
                ptr::null()
            }
        }
    }

    fn as_ptr_mut(&self) -> *const Node<Value> {
        match self.ptr {
            Some(ref ptr) => {
                &**ptr as *const Node<Value>
            },
            None => {
                ptr::null()
            }
        }
    }
    fn as_node_ref_mut<'x>(&'x mut self) -> &'x mut Node<Value> {
        match self.ptr {
            None => unreachable!(),
            Some(ref mut ptr) => ptr,
        }
    }

    pub fn as_ref<'x>(&self) -> NodeRef<marker::Immut<'x>, Value> {
        NodeRef {
            node: self.as_ptr(),
            _marker: PhantomData,
        }
    }

    pub fn as_ref_mut<'x>(&mut self) -> NodeRef<marker::Mut<'x>, Value> {
        NodeRef {
            node: self.as_ptr_mut(),
            _marker: PhantomData,
        }
    }

    pub fn as_mut<'x>(&'x mut self) -> NodeRefMut<'x, Value> {
        NodeRefMut {
            node: self as *mut BoxedNode<Value>,
            _marker: PhantomData,
        }
    }

    pub fn is_some(&self) -> bool {
        self.ptr.is_some()
    }

    pub fn take(&mut self) -> Option<Box<Node<Value>>> {
        self.ptr.take()
    }
}

impl<MutType, Value> NodeRef<MutType, Value> {
    // we have to be shure about valid ptr, before calling
    pub fn is_value(&self) -> bool {
        unsafe {
            let r = &*self.node;
            r.value.is_some()
        }
    }
}

impl<'x, Value> NodeRef<marker::Immut<'x>, Value> {
    pub fn as_option(&self) -> Option<&'x Node<Value>> {
        if self.node.is_null() {
            None
        } else {
            unsafe {
                Some(&*self.node)
            }
        }
    }
}

impl<'x, Value> Deref for NodeRef<marker::Immut<'x>, Value> {
    type Target = Node<Value>;

    fn deref(&self) -> &Node<Value> {
        unsafe {
            &*self.node
        }
    }
}

impl<'x, Value> NodeRef<marker::Mut<'x>, Value> {
    pub fn as_immut(self) -> NodeRef<marker::Immut<'x>, Value> {
        NodeRef {
            node: self.node,
            _marker: PhantomData,
        }
    }
}

impl<MutType, Value> Clone for NodeRef<MutType, Value> {
    fn clone(&self) -> Self {
        NodeRef {
            node: self.node,
            _marker: PhantomData,
        }
    }
}

impl<'x, Value> NodeRefMut<'x, Value> {
    pub fn as_node_ref(&self) -> &'x mut Node<Value> {
        unsafe {
            let mut r: &mut BoxedNode<Value> = &mut *self.node;
            r.as_node_ref_mut()
        }
    }

    pub fn as_mut(&self) -> &'x mut BoxedNode<Value> {
        unsafe {
            &mut *self.node
        }
    }

    pub fn assign(&mut self, node: BoxedNode<Value>) {
        unsafe {
            *self.node = node;
        }
    }
}

impl<'x, Value> Clone for NodeRefMut<'x, Value> {
    fn clone(&self) -> Self {
        NodeRefMut {
            node: self.node,
            _marker: PhantomData,
        }
    }
}

impl<Value> Node<Value> {
    fn new(c: char) -> Node<Value> {
        Node {
            lt: Default::default(),
            eq: Default::default(),
            gt: Default::default(),
            value: None,
            c: c,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.lt.ptr.is_none() && self.gt.ptr.is_none() && self.eq.ptr.is_none() && self.value.is_none()
    }

    pub fn replace(&mut self, value: Option<Value>) -> Option<Value> {
        mem::replace(&mut self.value, value)
    }
}

impl<Value: Debug> Debug for Node<Value> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{{\n"));
        try!(write!(f, "lt = {:?}, eq = {:?}, gt = {:?}, val = {:?}, c = {:?}",
            self.lt, self.eq, self.gt, self.value, self.c));
        (write!(f, "}}\n"))
    }
}

pub mod marker {
    use core::marker::PhantomData;

    pub struct Immut<'x>(PhantomData<&'x ()>);
    pub struct Mut<'x>(PhantomData<&'x mut ()>);
}
