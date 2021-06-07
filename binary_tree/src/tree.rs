#[derive(Debug)]

pub struct TreeNode {
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
    pub value: i32,
}
type ChildNode = Option<Box<TreeNode>>;

impl TreeNode {
    pub fn new(value: i32, left: ChildNode, right: ChildNode) -> ChildNode {
        Some(Box::new(TreeNode {
            left: left,
            right: right,
            value: value,
        }))
    }
    pub fn add_node(&mut self, value: i32) {
        if self.value > value {
            match self.left {
                Some(ref mut node) => node.add_node(value),
                None => {
                    self.add_left(value);
                }
            }
        } else {
            match self.right {
                Some(ref mut node) => node.add_node(value),
                None => {
                    self.add_right(value);
                }
            }
        }
    }
    pub fn add_left(&mut self, value: i32) {
        self.left = TreeNode::new(value, None, None);
    }
    pub fn add_right(&mut self, value: i32) {
        self.right = TreeNode::new(value, None, None);
    }
}

pub fn traverse_tree(root_node: &ChildNode) {
    structure(&root_node, 0);
}

pub fn padding(ch: char, n: i32) {
    for _i in 0..n {
        print!("{}", ch);
    }
}

pub fn structure(root_node: &ChildNode, level: i32) {
    match root_node {
        None => {
            padding('\t', level);
            println!("END");
        }
        Some(node) => {
            structure(&node.right, level + 1);
            padding('\t', level);
            println!("{}", node.value.to_string());
            structure(&node.left, level + 1);
        }
    }
}
