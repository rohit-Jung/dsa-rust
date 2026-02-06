// Box<Node> pointer to heap data
// Since End doesn't point so we have Option
// FIX: This is wrong for real world linked lists as one value ends of owning other check this
pub struct Node {
    value: i32,
    next: Option<Box<Node>>,
    prev: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            next: None,
            prev: None,
        }
    }
}

// TODO: write the tree structure

// TODO: write BST Insertion and search code here
// -> recursive, check root, check value, insert

// TODO: write deletion of tree
// -> recursive, check root, check value, insert

// TODO: write post and pre order inorder
