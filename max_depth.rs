// Definition for a binary tree node.
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);
            1 + left_depth.max(right_depth) // Add 1 to account for the current node
        }
        None => 0, // Base case: An empty tree has depth 0
    }
}

fn main() {
    // Construct a sample binary tree
    let mut root = TreeNode::new(3);
    let left = Some(Box::new(TreeNode::new(9)));
    let right = Some(Box::new(TreeNode::new(20)));
    let right_left = Some(Box::new(TreeNode::new(15)));
    let right_right = Some(Box::new(TreeNode::new(7)));
    root.left = left;
    root.right = right;
    root.right.as_mut().unwrap().left = right_left;
    root.right.as_mut().unwrap().right = right_right;

    let depth = max_depth(Some(Box::new(root)));
    println!("Maximum depth of the binary tree: {}", depth);
}
