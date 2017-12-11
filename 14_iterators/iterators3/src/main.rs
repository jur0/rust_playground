enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

// The state of in-order traversal.
struct TreeIter<'a, T: 'a> {
    // A stack of references to tree nodes.
    //
    // Since we use Vec's push and pop, the top of the stack is the end of the
    // vector. The node the iterator will visit next is at the top of the
    // stack, with those ancestors still unvisited below it. If the stack is
    // empty, the iteration is over.
    unvisited: Vec<&'a TreeNode<T>>,
}

impl<'a, T: 'a> TreeIter<'a, T> {
    // Push nodes of a subtree (going down the left edge) onto the stack.
    fn push_left_edge(&mut self, mut tree: &'a BinaryTree<T>) {
        while let BinaryTree::NonEmpty(ref node) = *tree {
            self.unvisited.push(node);
            tree = &node.left;
        }
    }
}

impl<T> BinaryTree<T> {
    // Initial stack has the leftmost nodes on the top of the stack.
    fn iter(&self) -> TreeIter<T> {
        let mut iter = TreeIter {
            unvisited: Vec::new(),
        };
        iter.push_left_edge(self);
        iter
    }
}

// The IntoIter definition establishes TreeIter as the iterator type for
// &BinaryTree.
impl<'a, T: 'a> IntoIterator for &'a BinaryTree<T> {
    type Item = &'a T;
    type IntoIter = TreeIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> Iterator for TreeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        // Find the node this iteration must produce, or finish the iteration.
        let node = match self.unvisited.pop() {
            Some(n) => n,
            None => return None,
        };

        // The next node after this one is the leftmost child of this node's
        // right child, so push the path from here down.
        self.push_left_edge(&node.right);

        // Reference to this node's value.
        Some(&node.element)
    }
}

fn make_node<T>(left: BinaryTree<T>, element: T, right: BinaryTree<T>) -> BinaryTree<T> {
    BinaryTree::NonEmpty(Box::new(TreeNode {
        left,
        element,
        right,
    }))
}

fn main() {
    let subtree_l = make_node(BinaryTree::Empty, "mecha", BinaryTree::Empty);
    let subtree_rl = make_node(BinaryTree::Empty, "droid", BinaryTree::Empty);
    let subtree_r = make_node(subtree_rl, "robot", BinaryTree::Empty);
    let tree = make_node(subtree_l, "Jaeger", subtree_r);

    let mut v = Vec::new();
    for kind in &tree {
        v.push(*kind);
    }

    assert_eq!(v, ["mecha", "Jaeger", "droid", "robot"]);

    assert_eq!(
        tree.iter()
            .map(|name| format!("mega-{}", name))
            .collect::<Vec<_>>(),
        vec!["mega-mecha", "mega-Jaeger", "mega-droid", "mega-robot"]
    );
}
