use std::rc::Rc;
use std::cell::RefCell;

// cannot be implemented with standard borrowed references
#[derive(Debug, PartialEq)]
pub struct TreeNode {
    value: Option<u32>,
    children: Vec<Rc<RefCell<TreeNode>>>,
    parent: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new() -> TreeNode {
        return TreeNode { value: None, children: Vec::new(), parent: None };
    }

    pub fn add_child(&mut self, new_node: Rc<RefCell<TreeNode>>) {
        self.children.push(new_node);
    }

    pub fn to_string(&self) -> String {
        if let Some(value) = self.value {
            return value.to_string();
        } else {
            let string_format = &self.children
                .iter()
                .map(|tn| tn.borrow().to_string())
                .collect::<Vec<String>>()
                .join(",");
            return String::from("[") + string_format + "]";
        }
    }
}

pub fn init_numerical_tree(s: String) -> Rc<RefCell<TreeNode>> {
    let root = Rc::new(RefCell::new(TreeNode::new()));
    let mut current = Rc::clone(&root);
    let chars = s.chars().collect::<Vec<char>>();

    let char_iterator = chars.iter().enumerate();
    for (_, c) in char_iterator.filter(|(indx, _)| *indx > 0 && *indx + 1 < chars.len()) {
        if *c == '[' || c.is_numeric() {
            let child = Rc::new(RefCell::new(TreeNode::new()));
            current.borrow_mut().children.push(Rc::clone(&child));
            {
                let mut mut_child = child.borrow_mut();
                mut_child.parent = Some(Rc::clone(&current));
                if c.is_numeric() {
                    mut_child.value = c.to_digit(10);
                }
            }
            current = child;
        } else if *c == ',' || *c == ']' || *c == ' ' {
            let current_clone = Rc::clone(&current);
            current = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
        } else {
            panic!("Unknown character: {}", c);
        }
    }

    return root;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numerical_tree_simple() {
        let tree = init_numerical_tree(String::from("[1,2]"));
        assert_eq!(tree.borrow().children[1].borrow().value.unwrap(), 2);
    }

    #[test]
    fn test_numerical_tree_space_divider() {
        let tree = init_numerical_tree(String::from("[8 1 2 5]"));
        assert_eq!(tree.borrow().children[2].borrow().value.unwrap(), 2);
    }

    #[test]
    fn test_numerical_tree_print() {
        assert_eq!(
            init_numerical_tree(String::from("[1,2]")).borrow().to_string(),
            String::from("[1,2]")
        );
        assert_eq!(
            init_numerical_tree(String::from("[1 2]")).borrow().to_string(),
            String::from("[1,2]")
        );
    }
}