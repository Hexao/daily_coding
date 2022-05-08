/// A unival tree (which stands for "universal value") is a tree where
/// all nodes under it have the same value.
///
/// Given the root to a binary tree, count the number of unival subtrees.
pub fn count_unival_subtree<T>(root: &Tree<T>) -> usize where T: std::cmp::PartialEq {
    inner_count(root).1
}

/// return a tuple as: (bool, usize) where the first part describe if `root`
/// is a unival subtree or not, and the second part count how many
/// unival subtree `root` has under it.
fn inner_count<T>(root: &Tree<T>) -> (bool, usize) where T: std::cmp::PartialEq {
    let Tree { node, left, right } = root;

    let (left_unival, left_count, left_match) = if let Some(leaf) = left {
        let (l, r) = inner_count(leaf);
        let m = leaf.node == *node;

        (l ,r, m)
    } else {
        (true, 0, true)
    };

    let (right_unival, right_count, right_match) = if let Some(leaf) = right {
        let (l, r) = inner_count(leaf);
        let m = leaf.node == *node;

        (l ,r, m)
    } else {
        (true, 0, true)
    };

    let unival_subtree = if left_match && right_match && left_unival && right_unival {
        1
    } else {
        0
    };

    (unival_subtree == 1, left_count + right_count + unival_subtree)
}

type Leaf<T> = Option<Box<Tree<T>>>;

pub struct Tree<T> {
    node: T,
    left: Leaf<T>,
    right: Leaf<T>,
}

impl<T> Tree<T> {
    pub fn new(node: T) -> Self {
        Self { node, left: None, right: None }
    }

    pub fn new_leaf(node: T, left: Leaf<T>, right: Leaf<T>) -> Self {
        Self { node, left, right }
    }
}

#[macro_export]
macro_rules! root {
    ($node:expr) => {
        Some(Box::new(daily_coding::easy::p008::Tree::new($node)))
    };
    [[$($left:tt),+], $node:expr, [$($right:tt),+]] => {
        daily_coding::easy::p008::Tree::new_leaf(
        $node,
        Some(Box::new(daily_coding::root![$($left),+])),
        Some(Box::new(daily_coding::root![$($right),+]))
        )
    };
    [$left:expr, $node:expr, [$($right:tt),+]] => {
        daily_coding::easy::p008::Tree::new_leaf(
            $node,
            daily_coding::root!($left),
            Some(Box::new(daily_coding::root![$($right),+]))
        )
    };
    [[$($left:tt),+], $node:expr, $right:expr] => {
        daily_coding::easy::p008::Tree::new_leaf(
            $node,
            Some(Box::new(daily_coding::root![$($left),+])),
            daily_coding::root!($right)
        )
    };
    [$left:expr, $node:expr, $right:expr] => {
        daily_coding::easy::p008::Tree::new_leaf(
            $node,
            daily_coding::root!($left),
            daily_coding::root!($right)
        )
    };
}
