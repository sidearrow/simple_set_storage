use simple_set_storage::btree::BTree;

fn main() {
    let mut bt = BTree::new(Some(3));
    bt.insert(String::from("aaaaaaa"));
    println!("{:?}", &bt);
}
