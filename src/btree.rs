#[derive(Debug)]
struct BTreeNode {
    keys: Vec<String>,
}

impl BTreeNode {
    pub fn new() -> Self {
        BTreeNode { keys: Vec::new() }
    }

    pub fn set(&mut self, s: String) {
        self.keys.push(s);
    }
}

#[derive(Debug)]
pub struct BTree {
    min_degree: i32,
    root: BTreeNode,
}

impl BTree {
    pub fn new(min_degree: Option<i32>) -> Self {
        BTree {
            min_degree: min_degree.unwrap_or(32),
            root: BTreeNode::new(),
        }
    }

    pub fn insert(&mut self, val: String) {
        self.root.set(val);
    }
}

