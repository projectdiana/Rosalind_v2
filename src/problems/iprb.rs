use std::collections::HashMap;

struct ProbTreeNode {
    values: HashMap<String,u64>,
    children: Box<HashMap<String, ProbTreeNode>>
}

impl ProbTreeNode {
    pub fn new(val: HashMap<String, u64>) -> ProbTreeNode {
        ProbTreeNode {
            values: val,
            children: Box::new(HashMap::<String, ProbTreeNode>::new())
        }
    }

    pub fn add_child(&mut self, label: String, child: ProbTreeNode) {
        self.children.insert(label, child);
    }

    pub fn expand(&mut self) {
        let mut map: HashMap<String, ProbTreeNode> = HashMap::new();
        
        for (label, _)  in self.values.iter() {
            let val: HashMap<String, u64> = self.values
                                                .iter()
                                                .map(|(key, val)| if key == label {(key.clone(), *val-1)} else {(key.clone(), *val)}) 
                                                .collect();
            let node = ProbTreeNode::new(val);
            map.insert(label.clone(), node);
        }

        *self.children = map;
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

struct ProbTree {
    root: ProbTreeNode
}

impl ProbTree {
    pub fn new(base: HashMap<String, u64>) -> ProbTree {
        ProbTree {
            root: ProbTreeNode::new(base)
        }
    }

    pub fn leafs(&self) -> Vec<&ProbTreeNode> {
        let leafs = vec![];
        // check if the root node is empty
        let root = self.root;
        fn get_leafs(node: &ProbTreeNode) -> Vec<&ProbTreeNode> {
            // if the node has kids return get leaves on the kids
            if node.children.is_empty() {
                vec![node]
            } else {
                let mut vec: Vec<&ProbTreeNode> = vec![];
                for (_, val) in node.children.iter() {
                    vec.append(get_leafs(val));
                }
                vec
            }
            }
            }
            
}

pub fn solve(dom: u64, het: u64, rec: u64) -> f64 {


    let mut map = HashMap::<String, u64>::new();
    map.insert("DOM".into(),dom);
    map.insert("HET".into(),het);
    map.insert("REC".into(),rec);

    let mut probtree = ProbTree::new(map);

        0.0

}
