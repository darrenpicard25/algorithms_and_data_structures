use std::{cell::RefCell, fmt::Debug, rc::Rc};

type Link<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug)]
struct Node<T: Clone> {
    pub value: T,
    pub next: Option<Link<T>>,
}

impl<T: Clone> Node<T> {
    fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

#[derive(Debug)]
pub struct SinglyLinkedList<T: Debug + Clone> {
    length: usize,
    head: Option<Link<T>>,
    tail: Option<Link<T>>,
}

impl<T: Debug + Clone> Default for SinglyLinkedList<T> {
    fn default() -> Self {
        Self {
            length: Default::default(),
            head: Default::default(),
            tail: Default::default(),
        }
    }
}

impl<T: Debug + Clone> SinglyLinkedList<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(values: Vec<T>) -> Self {
        let mut linked_list = Self::new();

        for value in values {
            linked_list.push(value);
        }

        linked_list
    }

    pub fn push(&mut self, value: T) {
        let new = Rc::new(RefCell::new(Node::new(value)));

        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone()),
        };
        self.length += 1;
        self.tail = Some(new);
    }

    pub fn pop(&mut self) -> Option<T> {
        // If the list is empty, return None
        if self.length == 0 {
            return None;
        }

        // If the list has only one element
        if self.length == 1 {
            let old_head = self.head.take();
            self.length -= 1;
            self.tail.take();

            return old_head.and_then(|head| Some(Rc::into_inner(head)?.into_inner().value));
        }

        // Find the second-to-last node
        let mut current = self.head.clone();
        while current
            .as_ref()?
            .borrow()
            .next
            .as_ref()?
            .borrow()
            .next
            .is_some()
        {
            let next = current.as_ref()?.borrow().next.clone();
            current = next;
        }

        // Remove the last node
        let result = current.as_ref()?.borrow_mut().next.take();
        self.tail = current;
        self.length -= 1;

        // Return the value of the popped node
        Some(Rc::into_inner(result?)?.into_inner().value)
    }

    pub fn shift(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::into_inner(head).map(|inner| inner.into_inner().value)
        })?
    }

    pub fn unshift(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node::new(value)));

        match self.head.take() {
            Some(old_head) => {
                new_node.borrow_mut().next = Some(old_head);
                self.head = Some(new_node)
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        };

        self.length += 1;
    }

    pub fn get(&self, requested_index: usize) -> Option<T> {
        if requested_index >= self.len() {
            return None;
        };

        let mut index = 0;
        let mut current = self.head.clone();

        while let Some(node) = current.clone() {
            if index >= requested_index {
                break;
            }

            current = node.as_ref().borrow().next.clone();
            index += 1;
        }

        Some((*current?.clone().borrow()).value.clone())
    }

    pub fn set(&mut self, requested_index: usize, value: T) -> bool {
        if requested_index >= self.len() {
            return false;
        };

        let mut index = 0;
        let mut current = self.head.clone();

        while let Some(node) = current.clone() {
            if index >= requested_index {
                break;
            }

            current = node.as_ref().borrow().next.clone();
            index += 1;
        }

        if let Some(node) = current {
            (*node.borrow_mut()).value = value;
            return true;
        }

        false
    }

    pub fn insert(&mut self, requested_index: usize, value: T) -> bool {
        if requested_index >= self.len() {
            return false;
        };

        if requested_index == 0 {
            self.unshift(value);
            return true;
        }

        if requested_index == self.len() {
            self.push(value);
            return true;
        }

        let mut index = 0;
        let mut current = self.head.clone();

        while let Some(node) = current.clone() {
            if index >= requested_index {
                break;
            }

            current = node.as_ref().borrow().next.clone();
            index += 1;
        }

        if let Some(node) = current {
            (*node.borrow_mut()).value = value;
            return true;
        }

        false
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod push {

        use super::*;

        #[test]
        fn can_push_on_empty_linked_list() {
            let mut linked_list = SinglyLinkedList::new();

            linked_list.push(5);
            assert_eq!(linked_list.len(), 1);
        }

        #[test]
        fn can_push_multiple_times_on_list() {
            let mut linked_list = SinglyLinkedList::new();

            linked_list.push(5);
            linked_list.push(6);
            linked_list.push(7);

            assert_eq!(linked_list.len(), 3);
        }
    }

    mod pop {

        use super::*;

        #[test]
        fn can_pop_on_empty_linked_list() {
            let mut linked_list: SinglyLinkedList<u8> = SinglyLinkedList::new();

            let results = linked_list.pop();

            assert_eq!(results, None);
            assert!(linked_list.head.is_none());
            assert!(linked_list.tail.is_none());
        }

        #[test]
        fn can_pop_on_linked_list_with_1_item() {
            let mut linked_list = SinglyLinkedList::from(Vec::from([2_u8]));

            let results = linked_list.pop();

            assert_eq!(results, Some(2));
            assert!(linked_list.head.is_none());
            assert!(linked_list.tail.is_none());
        }

        #[test]
        fn can_pop_multiple_times_on_list() {
            let mut linked_list = SinglyLinkedList::from(Vec::from([2_u8, 3_u8, 10_u8, 20_u8]));

            assert_eq!(linked_list.len(), 4);

            let value = linked_list.pop();
            assert_eq!(value, Some(20));

            let value = linked_list.pop();
            assert_eq!(value, Some(10));

            let value = linked_list.pop();
            assert_eq!(value, Some(3));

            let value = linked_list.pop();
            assert_eq!(value, Some(2));

            let value = linked_list.pop();
            assert!(value.is_none());
        }
    }

    mod unshift {

        use super::*;

        #[test]
        fn can_unshift_on_empty_linked_list() {
            let mut linked_list = SinglyLinkedList::new();

            linked_list.unshift(5);

            let head = linked_list.head.as_ref().unwrap().borrow();
            let tail = linked_list.tail.as_ref().unwrap().borrow();

            assert_eq!(linked_list.len(), 1);
            assert_eq!(head.value, 5);
            assert_eq!(tail.value, 5);
        }

        #[test]
        fn can_unshift_multiple_times_on_list() {
            let mut linked_list = SinglyLinkedList::new();

            linked_list.unshift(5);
            linked_list.unshift(6);
            linked_list.unshift(7);

            let head = linked_list.head.as_ref().unwrap().borrow();
            let tail = linked_list.tail.as_ref().unwrap().borrow();

            assert_eq!(linked_list.len(), 3);
            assert_eq!(head.value, 7);
            assert_eq!(tail.value, 5);
        }
    }

    mod shift {

        use super::*;

        #[test]
        fn returns_none_on_empty_list() {
            let mut linked_list: SinglyLinkedList<u8> = SinglyLinkedList::new();

            assert_eq!(linked_list.shift(), None);
        }

        #[test]
        fn can_shift_on_linked_list_with_one_node() {
            let mut linked_list: SinglyLinkedList<u8> = SinglyLinkedList::from(Vec::from([4]));

            assert_eq!(linked_list.shift(), Some(4));
            assert_eq!(linked_list.len(), 0);
            assert!(linked_list.head.is_none());
            assert!(linked_list.tail.is_none());
        }

        #[test]
        fn can_shift_multiple_times_on_list() {
            let mut linked_list = SinglyLinkedList::from(Vec::from([5, 6]));

            assert_eq!(linked_list.shift(), Some(5));
            assert_eq!(linked_list.shift(), Some(6));
            assert_eq!(linked_list.shift(), None);
        }
    }

    mod get {

        use super::*;

        #[test]
        fn should_return_none_if_index_greater_then_length() {
            let linked_list: SinglyLinkedList<u8> = SinglyLinkedList::from(Vec::from([4, 5, 6]));

            assert_eq!(linked_list.get(4), None);
        }
        #[test]
        fn should_return_none_if_list_is_empty() {
            let linked_list: SinglyLinkedList<u8> = SinglyLinkedList::from(Vec::from([]));

            assert_eq!(linked_list.get(0), None);
        }

        #[test]
        fn should_return_first_item_in_array_if_index_0_passed_in() {
            let linked_list: SinglyLinkedList<u8> = SinglyLinkedList::from(Vec::from([1]));

            assert_eq!(linked_list.get(0), Some(1));
            let head = linked_list.head.as_ref().unwrap().borrow();
            let tail = linked_list.tail.as_ref().unwrap().borrow();

            assert_eq!(head.value, 1);
            assert_eq!(tail.value, 1);
        }

        #[test]
        fn should_return_last_item_in_array_if_requested() {
            let linked_list: SinglyLinkedList<u8> = SinglyLinkedList::from(Vec::from([1, 2, 3]));

            assert_eq!(linked_list.get(2), Some(3));
            let head = linked_list.head.as_ref().unwrap().borrow();
            let tail = linked_list.tail.as_ref().unwrap().borrow();

            assert_eq!(head.value, 1);
            assert_eq!(tail.value, 3);
        }
    }

    mod set {

        use super::*;

        #[test]
        fn should_return_false_if_attempting_set_on_empty_list() {
            let mut linked_list: SinglyLinkedList<u8> = SinglyLinkedList::from(Vec::from([]));

            assert!(!linked_list.set(0, 4));
        }

        #[test]
        fn should_return_false_if_attempting_set_on_index_greater_then_list_length() {
            let mut linked_list: SinglyLinkedList<u8> =
                SinglyLinkedList::from(Vec::from([1, 2, 3]));

            assert!(!linked_list.set(3, 4));
        }

        #[test]
        fn should_return_true_and_set_last_value() {
            let mut linked_list: SinglyLinkedList<u8> =
                SinglyLinkedList::from(Vec::from([1, 2, 3]));

            assert!(linked_list.set(2, 4));
            let head = linked_list.head.as_ref().unwrap().borrow();
            let tail = linked_list.tail.as_ref().unwrap().borrow();

            assert_eq!(head.value, 1);
            assert_eq!(tail.value, 4);
        }

        #[test]
        fn should_return_true_and_set_first_value() {
            let mut linked_list: SinglyLinkedList<u8> =
                SinglyLinkedList::from(Vec::from([1, 2, 3]));

            assert!(linked_list.set(0, 4));
            let head = linked_list.head.as_ref().unwrap().borrow();
            let tail = linked_list.tail.as_ref().unwrap().borrow();

            assert_eq!(head.value, 4);
            assert_eq!(tail.value, 3);
        }

        #[test]
        fn should_return_true_and_set_value() {
            let mut linked_list: SinglyLinkedList<u8> =
                SinglyLinkedList::from(Vec::from([1, 2, 3]));

            assert!(linked_list.set(1, 4));
            let head = linked_list.head.as_ref().unwrap().borrow();
            let tail = linked_list.tail.as_ref().unwrap().borrow();

            assert_eq!(head.value, 1);
            assert_eq!((*head.next.clone().unwrap().as_ref().borrow()).value, 4);
            assert_eq!(tail.value, 3);
        }
    }
}
