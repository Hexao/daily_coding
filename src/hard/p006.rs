/// An XOR linked list is a more memory efficient doubly linked list.
/// Instead of each node holding `next` and `prev` fields, it holds a field
/// named `both`, which is an XOR of the `next` node and the `prev` node.
///
/// Implement an XOR linked list; it has an `add(element)` which adds the
/// element to the end, and a `get(index)` which returns the node at index.
pub struct XorList<T> {
    node: T,
    both: usize,
}

impl<T> XorList<T> {
    pub fn new(node: T) -> Self {
        XorList { node, both: 0 }
    }

    pub fn get(&self, mut index: usize) -> Option<&T> {
        let mut node = self;
        let mut from = 0;

        while index > 0 {
            let next = node.both ^ from;
            if next == 0 { return None; }

            from = node as *const _ as usize;
            node = unsafe {
                &*(next as *const XorList<T>)
            };

            index -= 1;
        }

        Some(&node.node)
    }

    pub fn add(&mut self, value: T) {
        let mut node = self;
        let mut from = 0;

        while node.both != from {
            let next = node.both ^ from;
            from = node as *const _ as usize;

            node = unsafe {
                &mut *(next as *mut XorList<T>)
            };
        }

        use std::alloc::{Layout, alloc_zeroed};
        let layout = Layout::new::<XorList<T>>();

        unsafe {
            let ptr = alloc_zeroed(layout) as *mut XorList<T>;
            let both = node as *const _ as usize;
            node.both ^= ptr as usize;

            (*ptr).node = value;
            (*ptr).both = both;
        }
    }
}

impl<T> Drop for XorList<T> {
    fn drop(&mut self) {
        let from = self as *const _ as usize;

        if self.both != 0 {
            unsafe {
                let node = self.both as *mut XorList<T>;
                (*node).both ^= from;

                std::ptr::drop_in_place(node);
            };
        }
    }
}
