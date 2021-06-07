mod tree;
use rand::Rng;

fn main() {
    let mut first_node = tree::TreeNode::new(10, None, None).unwrap();
    let mut rng = rand::thread_rng();
    for _i in 0..5 {
        first_node.add_node(rng.gen_range(0..100));
    }
    tree::traverse_tree(&Some(first_node));
}
