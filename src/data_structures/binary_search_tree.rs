use std::collections::VecDeque;

#[derive(Debug)]
struct Node<T> {
    pub value: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}

impl<T: Ord + Eq> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: T) {
        match &self.value.cmp(&value) {
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Less => {
                if let Some(right) = self.right.as_mut() {
                    right.insert(value);
                } else {
                    self.right = Some(Box::new(Node::new(value)));
                }
            }
            std::cmp::Ordering::Greater => {
                if let Some(left) = self.left.as_mut() {
                    left.insert(value);
                } else {
                    self.left = Some(Box::new(Node::new(value)));
                }
            }
        }
    }

    fn contains(&self, value: T) -> bool {
        match &self.value.cmp(&value) {
            std::cmp::Ordering::Equal => true,
            std::cmp::Ordering::Less => {
                if let Some(right) = self.right.as_ref() {
                    right.contains(value)
                } else {
                    false
                }
            }
            std::cmp::Ordering::Greater => {
                if let Some(left) = self.left.as_ref() {
                    left.contains(value)
                } else {
                    false
                }
            }
        }
    }

    fn traverse_depth_first_pre_order(self, values: &mut Vec<T>) {
        values.push(self.value);

        if let Some(left_node) = self.left {
            left_node.traverse_depth_first_pre_order(values);
        }

        if let Some(right_node) = self.right {
            right_node.traverse_depth_first_pre_order(values);
        }
    }

    fn traverse_depth_first_post_order(self, values: &mut Vec<T>) {
        if let Some(left_node) = self.left {
            left_node.traverse_depth_first_post_order(values);
        }

        if let Some(right_node) = self.right {
            right_node.traverse_depth_first_post_order(values);
        }
        values.push(self.value);
    }

    fn traverse_depth_first_in_order(self, values: &mut Vec<T>) {
        if let Some(left_node) = self.left {
            left_node.traverse_depth_first_in_order(values);
        }

        values.push(self.value);

        if let Some(right_node) = self.right {
            right_node.traverse_depth_first_in_order(values);
        }
    }
}

#[derive(Debug)]
pub struct BinarySearchTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T> Default for BinarySearchTree<T> {
    fn default() -> Self {
        Self {
            root: Default::default(),
        }
    }
}

impl<T> BinarySearchTree<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: Ord + Eq> From<Vec<T>> for BinarySearchTree<T> {
    fn from(vec_value: Vec<T>) -> Self {
        let mut tree = Self::new();

        for value in vec_value.into_iter() {
            tree.insert(value);
        }

        tree
    }
}

impl<T: Ord + Eq> BinarySearchTree<T> {
    pub fn insert(&mut self, value: T) {
        if let Some(node) = &mut self.root {
            node.insert(value);
        } else {
            let node = Box::new(Node::new(value));

            self.root = Some(node);
        }
    }

    pub fn contains(&self, value: T) -> bool {
        match self.root.as_ref() {
            Some(node) => node.contains(value),
            None => false,
        }
    }

    pub fn traverse_breath_first(self) -> Vec<T> {
        let mut results = Vec::new();

        let Some(node) = self.root else {
            return results;
        };

        let mut queue = VecDeque::new();
        queue.push_back(node);

        while let Some(node) = queue.pop_front() {
            results.push(node.value);

            if let Some(left_node) = node.left {
                queue.push_back(left_node);
            }

            if let Some(right_node) = node.right {
                queue.push_back(right_node);
            }
        }

        results
    }

    pub fn traverse_depth_first_pre_order(self) -> Vec<T> {
        let mut results = Vec::new();

        let Some(node) = self.root else {
            return results;
        };

        node.traverse_depth_first_pre_order(&mut results);

        results
    }

    pub fn traverse_depth_first_post_order(self) -> Vec<T> {
        let mut results = Vec::new();

        let Some(node) = self.root else {
            return results;
        };

        node.traverse_depth_first_post_order(&mut results);

        results
    }

    pub fn traverse_depth_first_in_order(self) -> Vec<T> {
        let mut results = Vec::new();

        let Some(node) = self.root else {
            return results;
        };

        node.traverse_depth_first_in_order(&mut results);

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod insert {
        use super::*;

        #[test]
        fn can_insert() {
            let mut tree = BinarySearchTree::new();

            tree.insert(10);
            tree.insert(9);
            tree.insert(11);
            tree.insert(8);
            tree.insert(12);
            tree.insert(7);
            tree.insert(13);

            dbg!(tree);
        }
    }

    mod contains {
        use super::*;

        #[test]
        fn contains_all_expected() {
            let tree = BinarySearchTree::from(vec![1, 2, 3, 4, 5]);

            assert!(tree.contains(1));
            assert!(tree.contains(2));
            assert!(tree.contains(3));
            assert!(tree.contains(4));
            assert!(tree.contains(5));
            assert!(!tree.contains(6));
        }
    }

    mod traverse_breath_first {
        use super::*;

        #[test]
        fn can_insert() {
            let tree = BinarySearchTree::from(vec![10, 6, 15, 3, 8, 20]);

            dbg!(&tree);

            assert_eq!(tree.traverse_breath_first(), vec![10, 6, 15, 3, 8, 20])
        }
    }

    mod traverse_depth_first_pre_order {
        use super::*;

        #[test]
        fn can_insert() {
            let tree = BinarySearchTree::from(vec![10, 6, 15, 3, 8, 20]);

            dbg!(&tree);

            assert_eq!(
                tree.traverse_depth_first_pre_order(),
                vec![10, 6, 3, 8, 15, 20]
            )
        }
    }

    mod traverse_depth_first_post_order {
        use super::*;

        #[test]
        fn can_insert() {
            let tree = BinarySearchTree::from(vec![10, 6, 15, 3, 8, 20]);

            dbg!(&tree);

            assert_eq!(
                tree.traverse_depth_first_post_order(),
                vec![3, 8, 6, 20, 15, 10]
            )
        }
    }

    mod traverse_depth_first_in_order {
        use super::*;

        #[test]
        fn can_insert() {
            let tree = BinarySearchTree::from(vec![10, 6, 15, 3, 8, 20]);

            dbg!(&tree);

            assert_eq!(
                tree.traverse_depth_first_in_order(),
                vec![3, 6, 8, 10, 15, 20]
            )
        }
    }
}
