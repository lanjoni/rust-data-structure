use std::fmt;

pub struct Tree {
    pub value: i32,
    pub left: Option<Box<Tree>>,
    pub right: Option<Box<Tree>>
}

impl Tree {
    pub fn new(value: i32) -> Self {
        Tree {
            value,
            left: None,
            right: None
        }
    }

    pub fn insert(&mut self, value: i32) {
        if value < self.value {
            match &mut self.left {
                None => self.left = Some(Box::new(Tree::new(value))),
                Some(subtree) => subtree.insert(value),
            }
        } else {
            match &mut self.right {
                None => self.right = Some(Box::new(Tree::new(value))),
                Some(subtree) => subtree.insert(value),
            }
        }
    }

    pub fn preorder(&self) -> String {
        let mut result = String::new();
        result.push_str(&self.value.to_string());
        result.push_str("\n");
        if let Some(node) = &self.left {
            result.push_str(&node.preorder());
        }
        if let Some(node) = &self.right {
            result.push_str(&node.preorder());
        }
        result
    }

    pub fn inorder(&self) -> String {
        let mut result = String::new();
        if let Some(node) = &self.left {
            result.push_str(&node.inorder());
        }
        result.push_str(&self.value.to_string());
        result.push_str("\n");
        if let Some(node) = &self.right {
            result.push_str(&node.inorder());
        }
        result
    }

    pub fn posorder(&self) -> String {
        let mut result = String::new();
        if let Some(node) = &self.left {
            result.push_str(&node.posorder());
        }
        if let Some(node) = &self.right {
            result.push_str(&node.posorder());
        }
        result.push_str(&self.value.to_string());
        result.push_str("\n");
        result
    }
    
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)?;
        if self.left.is_some() {
            write!(f, " ({})", self.left.as_ref().unwrap())?;
        }
        if self.right.is_some() {
            write!(f, " ({})", self.right.as_ref().unwrap())?;
        }
        Ok(())
    }
}
