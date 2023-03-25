mod classes;

use classes::tree::Tree;

fn main() {
    let mut tree = Tree::new(2);
    
    tree.insert(5);
    tree.insert(1);
    tree.insert(3);

    println!("\nPre order: \n{}", tree.preorder());
    println!("\nIn order: \n{}", tree.inorder());
    println!("\nPos order: \n{}", tree.posorder());
}