use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type BstNodeLink = Rc<RefCell<BstNode>>;
pub type WeakBstNodeLink = Weak<RefCell<BstNode>>;

//this package implement BST wrapper
#[derive(Debug, Clone)]
pub struct BstNode {
    pub key: Option<i32>,
    pub parent: Option<WeakBstNodeLink>,
    pub left: Option<BstNodeLink>,
    pub right: Option<BstNodeLink>,
}

impl BstNode {
    //private interface
    fn new(key: i32) -> Self {
        BstNode {
            key: Some(key),
            left: None,
            right: None,
            parent: None,
        }
    }

    pub fn new_bst_nodelink(value: i32) -> BstNodeLink {
        let currentnode = BstNode::new(value);
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    /**
     * Get a copy of node link
     */
    pub fn get_bst_nodelink_copy(&self) -> BstNodeLink {
        Rc::new(RefCell::new(self.clone()))
    }

    fn downgrade(node: &BstNodeLink) -> WeakBstNodeLink {
        Rc::<RefCell<BstNode>>::downgrade(node)
    }

    //private interface
    fn new_with_parent(parent: &BstNodeLink, value: i32) -> BstNodeLink {
        let mut currentnode = BstNode::new(value);
        //currentnode.add_parent(Rc::<RefCell<BstNode>>::downgrade(parent));
        currentnode.parent = Some(BstNode::downgrade(parent));
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    //add new left child, set the parent to current_node_link
    pub fn add_left_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.left = Some(new_node);
    }

    //add new left child, set the parent to current_node_link
    pub fn add_right_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.right = Some(new_node);
    }

    //search the current tree which node fit the value
    pub fn tree_search(&self, value: &i32) -> Option<BstNodeLink> {
        if let Some(key) = self.key {
            if key == *value {
                return Some(self.get_bst_nodelink_copy());
            }
            if *value < key && self.left.is_some() {
                return self.left.as_ref().unwrap().borrow().tree_search(value);
            } else if self.right.is_some() {
                return self.right.as_ref().unwrap().borrow().tree_search(value);
            }
        }
        //default if current node is NIL
        None
    }

    /**seek minimum by recurs
     * in BST minimum always on the left
     */
    pub fn minimum(&self) -> BstNodeLink {
        if self.key.is_some() {
            if let Some(left_node) = &self.left {
                return left_node.borrow().minimum();
            }
        }
        self.get_bst_nodelink_copy()
    }

    pub fn maximum(&self) -> BstNodeLink {
        if self.key.is_some() {
            if let Some(right_node) = &self.right {
                return right_node.borrow().maximum();
            }
        }
        self.get_bst_nodelink_copy()
    }

    /**
     * Return the root of a node, return self if not exist
     */
    pub fn get_root(node: &BstNodeLink) -> BstNodeLink {
        let parent = BstNode::upgrade_weak_to_strong(node.borrow().parent.clone());
        if parent.is_none() {
            return node.clone();
        }
        return BstNode::get_root(&parent.unwrap());
    }

    /**
     * NOTE: Buggy from pull request
     * Find node successor according to the book
     * Should return None, if x_node is the highest key in the tree
     */
    pub fn tree_successor(x_node: &BstNodeLink) -> Option<BstNodeLink> {
        // directly check if the node has a right child, otherwise go to the next block
        if let Some(right_node) = &x_node.borrow().right {
            return Some(right_node.borrow().minimum());
        } 
        
        // empty right child case
        else { 
            let mut x_node = x_node;
            let mut y_node = BstNode::upgrade_weak_to_strong(x_node.borrow().parent.clone());
            let mut temp: BstNodeLink;

            while let Some(ref exist) = y_node {
                if let Some(ref left_child) = exist.borrow().left {
                    if BstNode::is_node_match(left_child, x_node) {
                        return Some(exist.clone());
                    }
                }

                temp = y_node.unwrap();
                x_node = &temp;
                y_node = BstNode::upgrade_weak_to_strong(temp.borrow().parent.clone());
            }

            None    
        }
    }

    /**
     * Alternate simpler version of tree_successor that made use of is_nil checking
     */
    #[allow(dead_code)]
    pub fn tree_successor_simpler(x_node: &BstNodeLink) -> Option<BstNodeLink>{
        //create a shadow of x_node so it can mutate
        let mut x_node = x_node;
        let right_node = &x_node.borrow().right.clone();
        if BstNode::is_nil(right_node)!=true{
            return Some(right_node.clone().unwrap().borrow().minimum());
        }

        let mut y_node = BstNode::upgrade_weak_to_strong(x_node.borrow().parent.clone());
        let y_node_right = &y_node.clone().unwrap().borrow().right.clone();
        let mut y_node2: Rc<RefCell<BstNode>>;
        while BstNode::is_nil(&y_node) && BstNode::is_node_match_option(Some(x_node.clone()), y_node_right.clone()) {
            y_node2 = y_node.clone().unwrap();
            x_node = &y_node2;
            let y_parent = y_node.clone().unwrap().borrow().parent.clone().unwrap();
            y_node = BstNode::upgrade_weak_to_strong(Some(y_parent));
        }

        //in case our sucessor traversal yield root, means self is the highest key
        if BstNode::is_node_match_option(y_node.clone(), Some(BstNode::get_root(&x_node))) {
            return None;
        }

        //default return self / x_node
        return Some(y_node.clone().unwrap())
    }

    /**
     * private function return true if node doesn't has parent nor children nor key
     */
    fn is_nil(node: &Option<BstNodeLink>) -> bool {
        match node {
            None => true,
            Some(x) => {
                if x.borrow().parent.is_none()
                    || x.borrow().left.is_none()
                    || x.borrow().right.is_none()
                {
                    return true;
                }
                return false;
            }
        }
    }

    //helper function to compare both nodelink
    fn is_node_match_option(node1: Option<BstNodeLink>, node2: Option<BstNodeLink>) -> bool {
        if node1.is_none() && node2.is_none() {
            return true;
        }
        if let Some(node1v) = node1 {
            return node2.is_some_and(|x: BstNodeLink| x.borrow().key == node1v.borrow().key);
        }
        return false;
    }

    fn is_node_match(anode: &BstNodeLink, bnode: &BstNodeLink) -> bool {
        if anode.borrow().key == bnode.borrow().key {
            return true;
        }
        return false;
    }

    /**
     * As the name implied, used to upgrade parent node to strong nodelink
     */
    fn upgrade_weak_to_strong(node: Option<WeakBstNodeLink>) -> Option<BstNodeLink> {
        match node {
            None => None,
            Some(x) => Some(x.upgrade().unwrap()),
        }
    }

    /**
     * Insert a node with key into the BST
     * If the key already exists in the tree, do nothing
     */
    pub fn tree_insert(&mut self, root_link: &BstNodeLink, key: i32) {
        // If the current node is NIL (root is empty tree)
        if self.key.is_none() {
            self.key = Some(key);
            return;
        }
        
        let current_key = self.key.unwrap();
        
        if key < current_key {
            // Key should be inserted in the left subtree
            if let Some(left_link) = &self.left {
                // Recursively insert into left subtree
                left_link.borrow_mut().tree_insert(left_link, key);
            } else {
                // Create new left child
                self.add_left_child(root_link, key);
            }
        } else if key > current_key {
            // Key should be inserted in the right subtree
            if let Some(right_link) = &self.right {
                // Recursively insert into right subtree
                right_link.borrow_mut().tree_insert(right_link, key);
            } else {
                // Create new right child
                self.add_right_child(root_link, key);
            }
        }
        // If key == current_key, do nothing (no duplicates allowed)
    }

    /**
     * Transplant operation - replaces one subtree as a child of its parent with another subtree
     * Used as a helper function for tree_delete
     */
    pub fn transplant(root: &BstNodeLink, u: &BstNodeLink, v: Option<BstNodeLink>) {
        let u_parent = BstNode::upgrade_weak_to_strong(u.borrow().parent.clone());
        
        if u_parent.is_none() {
            // u is the root
            if let Some(v_node) = &v {
                // Copy all data from v to root
                root.borrow_mut().key = v_node.borrow().key;
                root.borrow_mut().left = v_node.borrow().left.clone();
                root.borrow_mut().right = v_node.borrow().right.clone();
                
                // Update parent pointers for children
                if let Some(left_child) = &root.borrow().left {
                    left_child.borrow_mut().parent = Some(Rc::downgrade(root));
                }
                if let Some(right_child) = &root.borrow().right {
                    right_child.borrow_mut().parent = Some(Rc::downgrade(root));
                }
            } else {
                // Setting root to NIL (empty tree)
                root.borrow_mut().key = None;
                root.borrow_mut().left = None;
                root.borrow_mut().right = None;
            }
        } else {
            let parent = u_parent.unwrap();
            
            // Determine if u is left or right child of its parent
            let is_left_child = parent.borrow().left.as_ref().map_or(false, |child| 
                BstNode::is_node_match(child, u));
            
            // Replace u with v in parent's child pointer
            if is_left_child {
                parent.borrow_mut().left = v.clone();
            } else {
                parent.borrow_mut().right = v.clone();
            }
            
            // Update v's parent pointer if v is not None
            if let Some(v_node) = &v {
                v_node.borrow_mut().parent = Some(Rc::downgrade(&parent));
            }
        }
    }

    /**
    * Delete a node with the given key from the BST
    * Returns true if deletion was successful, false if key not found
    */
    pub fn tree_delete(&mut self, root: &BstNodeLink, key: i32) -> bool {
        // First find the node to delete
        if let Some(z) = self.tree_search(&key) {
            // Case 1: z has no left child
            if z.borrow().left.is_none() {
                BstNode::transplant(root, &z, z.borrow().right.clone());
            } 
            // Case 2: z has left child but no right child
            else if z.borrow().right.is_none() {
                BstNode::transplant(root, &z, z.borrow().left.clone());
            } 
            // Case 3: z has both children
            else {
                // Store values we need before modifying anything
                let z_right = z.borrow().right.clone();
                let z_left = z.borrow().left.clone();
                
                // Find successor (minimum in right subtree)
                let y = z_right.as_ref().unwrap().borrow().minimum();
                
                // Check if y is the immediate right child of z or not
                let y_is_direct_child = BstNode::is_node_match(&y, z_right.as_ref().unwrap());
                
                // If y is not the immediate right child of z
                if !y_is_direct_child {
                    // First get y's right child (y never has a left child as it's a minimum)
                    let y_right = y.borrow().right.clone();
                    
                    // Transplant y with its right child
                    BstNode::transplant(root, &y, y_right);
                    
                    // Set y's right child to z's right child
                    y.borrow_mut().right = z_right;
                    
                    // Update parent pointer of y's new right child
                    if let Some(right) = &y.borrow().right {
                        right.borrow_mut().parent = Some(Rc::downgrade(&y));
                    }
                }
                
                // Now transplant z with y
                BstNode::transplant(root, &z, Some(y.clone()));
                
                // Set y's left child to z's left child
                y.borrow_mut().left = z_left;
                
                // Update parent pointer of y's new left child
                if let Some(left) = &y.borrow().left {
                    left.borrow_mut().parent = Some(Rc::downgrade(&y));
                }
                
                // If y wasn't the immediate right child of z, we already handled the right child pointer above
                // Otherwise, we need to set it here
                if y_is_direct_child {
                    // Preserve y's original right child
                    // y.borrow_mut().right is already set correctly in this case
                }
            }
            return true;
        }
        
        false // Key not found
    }
    
}
