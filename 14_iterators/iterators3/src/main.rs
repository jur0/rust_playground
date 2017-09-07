
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

// The state of in-order traversal
struct TreeIter<'a, T: 'a> {
    // A stack of references to tree nodes.
    //
    // Since we use Vec's push and pop, the top of the stack is the end of the
    // vector. The node the iterator will visit next is at the top of the
    // stack, with those ancestors still unvisited below it. If the stack is
    // empty, the iteration is over.
    unvisited: Vec<&'a TreeNode<T>>
}

// TODO

fn main() {
    println!("Hello, world!");
}
