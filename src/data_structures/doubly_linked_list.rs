use std::{cell::RefCell, rc::Rc};

type Link<T> = Rc<RefCell<Node<T>>>;

struct Node<T> {
    pub value: T,
    pub next: Option<Link<T>>,
    pub previous: Option<Link<T>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            next: None,
            previous: None,
        }
    }
}

pub struct DoublyLinkedList<T> {
    length: usize,
    head: Option<Link<T>>,
    tail: Option<Link<T>>,
}

impl<T> Default for DoublyLinkedList<T> {
    fn default() -> Self {
        Self {
            length: Default::default(),
            head: Default::default(),
            tail: Default::default(),
        }
    }
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(array: Vec<T>) -> Self {
        array
            .into_iter()
            .fold(Self::new(), |mut linked_list, value| {
                linked_list.push(value);

                linked_list
            })
    }
}

impl<T> DoublyLinkedList<T> {
    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node::new(value)));

        if let Some(old_tail) = self.tail.take() {
            old_tail.borrow_mut().next = Some(new_node.clone());
            new_node.borrow_mut().previous = Some(old_tail);
            self.tail = Some(new_node);
        } else {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
        }

        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().previous.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next = None;
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head = None;
                }
            }

            self.length -= 1;
            Rc::into_inner(old_tail)
                .expect("Attempting to extract value in pop from old tail failed")
                .into_inner()
                .value
        })
    }

    pub fn shift(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node::new(value)));

        if let Some(old_head) = self.head.take() {
            old_head.borrow_mut().previous = Some(new_node.clone());
            new_node.borrow_mut().next = Some(old_head);
            self.head = Some(new_node);
        } else {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
        }

        self.length += 1;
    }

    pub fn unshift(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().previous = None;
                    self.head = Some(new_head);
                }
                None => {
                    self.tail = None;
                }
            };

            self.length -= 1;
            Rc::into_inner(old_head)
                .expect("Attempting to extract value in unshift from old head failed")
                .into_inner()
                .value
        })
    }

    pub fn set(&mut self, index: usize, value: T) -> Option<T> {
        if index > self.len() {
            return None;
        }

        let mut current_index = 0;
        let mut current = self.head.clone();

        while let Some(next) = current.clone() {
            if current_index == index {
                break;
            }
            current = next.borrow().next.clone();
            current_index += 1;
        }

        if let Some(node) = current {
            return Some(std::mem::replace(&mut node.borrow_mut().value, value));
        }

        None
    }

    pub fn insert(&mut self, _index: usize, _value: T) {
        todo!()
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index == 0 {
            return self.unshift();
        } else if index == self.len() - 1 {
            return self.pop();
        }

        let mut current_index = 0;
        let mut current = self.head.clone();

        while let Some(next) = current.clone() {
            if current_index == index {
                break;
            }
            current = next.borrow().next.clone();
            current_index += 1;
        }

        current.take().map(|node| {
            {
                let mut borrowed_node = node.borrow_mut();
                let previous = borrowed_node.previous.take();
                let next = borrowed_node.next.take();

                if let Some(prev) = previous.clone() {
                    prev.borrow_mut().next = next.clone();
                }

                if let Some(next) = next {
                    next.borrow_mut().previous = previous;
                }
            }

            self.length -= 1;

            Rc::into_inner(node)
                .expect("Attempting to extract value in remove from old head failed")
                .into_inner()
                .value
        })
    }
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        while let Some(node) = self.head.take() {
            node.borrow_mut().previous.take();
            self.head = node.borrow_mut().next.take();
        }
        self.tail.take();
    }
}

impl<T> Iterator for DoublyLinkedList<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.unshift()
    }
}

impl<T: Clone> DoublyLinkedList<T> {
    pub fn get(&self, index: usize) -> Option<T> {
        if index > self.len() {
            return None;
        }

        let mut current_index = 0;
        let mut current = self.head.clone();

        while let Some(next) = current.clone() {
            if current_index == index {
                break;
            }
            current = next.borrow().next.clone();
            current_index += 1;
        }

        current.map(|inner| inner.borrow().value.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod push {
        use super::*;

        #[test]
        fn can_push_on_empty_linked_list() {
            let mut linked_list = DoublyLinkedList::new();

            linked_list.push(5);
            assert_eq!(linked_list.len(), 1);
            let borrowed_head = linked_list.head.as_ref().unwrap().borrow();
            let borrowed_tail = linked_list.tail.as_ref().unwrap().borrow();

            assert_eq!(borrowed_head.value, 5);
            assert_eq!(borrowed_tail.value, 5);
            assert!(borrowed_tail.next.is_none());
            assert!(borrowed_tail.previous.is_none());
            assert!(borrowed_head.next.is_none());
            assert!(borrowed_head.previous.is_none());
        }

        #[test]
        fn can_push_on_linked_list_with_1_item() {
            let mut linked_list = DoublyLinkedList::from(Vec::from([1]));

            linked_list.push(2);
            assert_eq!(linked_list.len(), 2);
            let borrowed_head = linked_list.head.as_ref().unwrap().borrow();
            let borrowed_tail = linked_list.tail.as_ref().unwrap().borrow();

            assert_eq!(borrowed_head.value, 1);
            assert_eq!(borrowed_tail.value, 2);

            let borrowed_head_next = borrowed_head.next.as_ref().unwrap().borrow();
            assert_eq!(borrowed_head_next.value, 2);
            assert!(borrowed_head.previous.is_none());

            let borrowed_tail_previous = borrowed_tail.previous.as_ref().unwrap().borrow();
            assert_eq!(borrowed_tail_previous.value, 1);
            assert!(borrowed_tail.next.is_none());
        }

        #[test]
        fn can_push_on_linked_list_multiple_times() {
            let mut linked_list = DoublyLinkedList::new();

            linked_list.push(1);
            linked_list.push(2);
            linked_list.push(3);

            assert_eq!(linked_list.len(), 3);
            let borrowed_head = linked_list.head.as_ref().unwrap().borrow();
            let borrowed_tail = linked_list.tail.as_ref().unwrap().borrow();

            assert_eq!(borrowed_head.value, 1);
            assert_eq!(borrowed_tail.value, 3);

            let borrowed_head_next = borrowed_head.next.as_ref().unwrap().borrow();
            assert_eq!(borrowed_head_next.value, 2);
            assert!(borrowed_head.previous.is_none());

            let borrowed_head_next_next = borrowed_head_next.next.as_ref().unwrap().borrow();
            assert_eq!(borrowed_head_next_next.value, 3);
            let borrowed_head_next_previous =
                borrowed_head_next.previous.as_ref().unwrap().borrow();
            assert_eq!(borrowed_head_next_previous.value, 1);

            let borrowed_tail_previous = borrowed_tail.previous.as_ref().unwrap().borrow();
            assert_eq!(borrowed_tail_previous.value, 2);
            assert!(borrowed_tail.next.is_none());
        }
    }

    mod pop {
        use super::*;

        #[test]
        fn returns_none_when_called_on_empty_list() {
            let mut linked_list: DoublyLinkedList<u8> = DoublyLinkedList::new();

            let results = linked_list.pop();

            assert_eq!(linked_list.len(), 0);

            assert!(results.is_none());
            assert!(linked_list.head.is_none());
            assert!(linked_list.tail.is_none());
        }

        #[test]
        fn returns_value_and_empty_list_if_only_1_value() {
            let mut linked_list = DoublyLinkedList::from(Vec::from([1_u8]));

            let results = linked_list.pop();
            assert_eq!(linked_list.len(), 0);
            assert_eq!(results, Some(1));
            assert!(linked_list.head.is_none());
            assert!(linked_list.tail.is_none());
        }

        #[test]
        fn returns_value_and_multiple_times() {
            let mut linked_list = DoublyLinkedList::from(Vec::from([1_u8, 2, 3, 4]));

            let result = linked_list.pop();
            let borrowed_tail = linked_list.tail.as_ref().unwrap().borrow();

            assert_eq!(linked_list.len(), 3);
            assert_eq!(result, Some(4));
            assert_eq!(borrowed_tail.value, 3);
            drop(borrowed_tail);

            let result = linked_list.pop();
            let borrowed_tail = linked_list.tail.as_ref().unwrap().borrow();

            assert_eq!(linked_list.len(), 2);
            assert_eq!(result, Some(3));
            assert_eq!(borrowed_tail.value, 2);
            drop(borrowed_tail);

            let result = linked_list.pop();
            let borrowed_tail = linked_list.tail.as_ref().unwrap().borrow();

            assert_eq!(linked_list.len(), 1);
            assert_eq!(result, Some(2));
            assert_eq!(borrowed_tail.value, 1);
            drop(borrowed_tail);

            let result = linked_list.pop();

            assert_eq!(linked_list.len(), 0);
            assert_eq!(result, Some(1));
            assert!(linked_list.tail.is_none());
            assert!(linked_list.head.is_none());
        }
    }

    mod shift {
        use super::*;

        #[test]
        fn can_shift_on_empty_linked_list() {
            let mut linked_list = DoublyLinkedList::new();

            linked_list.shift(5);
            assert_eq!(linked_list.len(), 1);
            let borrowed_head = linked_list.head.as_ref().unwrap().borrow();
            let borrowed_tail = linked_list.tail.as_ref().unwrap().borrow();

            assert_eq!(borrowed_head.value, 5);
            assert_eq!(borrowed_tail.value, 5);
            assert!(borrowed_tail.next.is_none());
            assert!(borrowed_tail.previous.is_none());
            assert!(borrowed_head.next.is_none());
            assert!(borrowed_head.previous.is_none());
        }

        #[test]
        fn can_shift_on_linked_list_with_1_item() {
            let mut linked_list = DoublyLinkedList::from(Vec::from([2]));

            linked_list.shift(1);
            assert_eq!(linked_list.len(), 2);
            let borrowed_head = linked_list.head.as_ref().unwrap().borrow();
            let borrowed_tail = linked_list.tail.as_ref().unwrap().borrow();

            assert_eq!(borrowed_head.value, 1);
            assert_eq!(borrowed_tail.value, 2);

            let borrowed_head_next = borrowed_head.next.as_ref().unwrap().borrow();
            assert_eq!(borrowed_head_next.value, 2);
            assert!(borrowed_head.previous.is_none());

            let borrowed_tail_previous = borrowed_tail.previous.as_ref().unwrap().borrow();
            assert_eq!(borrowed_tail_previous.value, 1);
            assert!(borrowed_tail.next.is_none());
        }

        #[test]
        fn can_shift_on_linked_list_multiple_times() {
            let mut linked_list = DoublyLinkedList::new();

            linked_list.shift(3);
            linked_list.shift(2);
            linked_list.shift(1);

            assert_eq!(linked_list.len(), 3);
            let borrowed_head = linked_list.head.as_ref().unwrap().borrow();
            let borrowed_tail = linked_list.tail.as_ref().unwrap().borrow();

            assert_eq!(borrowed_head.value, 1);
            assert_eq!(borrowed_tail.value, 3);

            let borrowed_head_next = borrowed_head.next.as_ref().unwrap().borrow();
            assert_eq!(borrowed_head_next.value, 2);
            assert!(borrowed_head.previous.is_none());

            let borrowed_head_next_next = borrowed_head_next.next.as_ref().unwrap().borrow();
            assert_eq!(borrowed_head_next_next.value, 3);
            let borrowed_head_next_previous =
                borrowed_head_next.previous.as_ref().unwrap().borrow();
            assert_eq!(borrowed_head_next_previous.value, 1);

            let borrowed_tail_previous = borrowed_tail.previous.as_ref().unwrap().borrow();
            assert_eq!(borrowed_tail_previous.value, 2);
            assert!(borrowed_tail.next.is_none());
        }
    }

    mod unshift {
        use super::*;

        #[test]
        fn returns_none_when_called_on_empty_list() {
            let mut linked_list: DoublyLinkedList<u8> = DoublyLinkedList::new();

            let results = linked_list.unshift();

            assert_eq!(linked_list.len(), 0);

            assert!(results.is_none());
            assert!(linked_list.head.is_none());
            assert!(linked_list.tail.is_none());
        }

        #[test]
        fn returns_value_and_empty_list_if_only_1_value() {
            let mut linked_list = DoublyLinkedList::from(Vec::from([1_u8]));

            let results = linked_list.unshift();
            assert_eq!(linked_list.len(), 0);
            assert_eq!(results, Some(1));
            assert!(linked_list.head.is_none());
            assert!(linked_list.tail.is_none());
        }

        #[test]
        fn returns_value_and_multiple_times() {
            let mut linked_list = DoublyLinkedList::from(Vec::from([1_u8, 2, 3, 4]));

            let result = linked_list.unshift();
            let borrowed_head = linked_list.head.as_ref().unwrap().borrow();

            assert_eq!(linked_list.len(), 3);
            assert_eq!(result, Some(1));
            assert_eq!(borrowed_head.value, 2);
            drop(borrowed_head);

            let result = linked_list.unshift();
            let borrowed_head = linked_list.head.as_ref().unwrap().borrow();

            assert_eq!(linked_list.len(), 2);
            assert_eq!(result, Some(2));
            assert_eq!(borrowed_head.value, 3);
            drop(borrowed_head);

            let result = linked_list.unshift();
            let borrowed_head = linked_list.head.as_ref().unwrap().borrow();

            assert_eq!(linked_list.len(), 1);
            assert_eq!(result, Some(3));
            assert_eq!(borrowed_head.value, 4);
            drop(borrowed_head);

            let result = linked_list.unshift();

            assert_eq!(linked_list.len(), 0);
            assert_eq!(result, Some(4));
            assert!(linked_list.tail.is_none());
            assert!(linked_list.head.is_none());
        }
    }

    mod get {

        use super::*;

        #[test]
        fn returns_none_when_called_on_empty_list() {
            let linked_list: DoublyLinkedList<u8> = DoublyLinkedList::new();

            let results = linked_list.get(0);

            assert_eq!(linked_list.len(), 0);

            assert!(results.is_none());
            assert!(linked_list.head.is_none());
            assert!(linked_list.tail.is_none());
        }

        #[test]
        fn returns_none_when_called_with_index_greater_then_len() {
            let linked_list: DoublyLinkedList<u8> =
                DoublyLinkedList::from(Vec::from([1_u8, 2, 3, 4]));

            let results = linked_list.get(5);
            let borrowed_head = linked_list.head.as_ref().unwrap().borrow();
            let borrowed_tail = linked_list.tail.as_ref().unwrap().borrow();

            assert_eq!(linked_list.len(), 4);

            assert!(results.is_none());
            assert_eq!(borrowed_head.value, 1);
            assert_eq!(borrowed_tail.value, 4);
        }

        #[test]
        fn returns_head_value_when_index_0_passed_in() {
            let linked_list: DoublyLinkedList<u8> =
                DoublyLinkedList::from(Vec::from([1_u8, 2, 3, 4]));

            let results = linked_list.get(0);
            let borrowed_head = linked_list.head.as_ref().unwrap().borrow();
            let borrowed_tail = linked_list.tail.as_ref().unwrap().borrow();

            assert_eq!(linked_list.len(), 4);

            assert_eq!(results, Some(1));
            assert_eq!(borrowed_head.value, 1);
            assert_eq!(borrowed_tail.value, 4);
        }

        #[test]
        fn returns_tail_value_when_len_minus_1_passed_in() {
            let linked_list: DoublyLinkedList<u8> =
                DoublyLinkedList::from(Vec::from([1_u8, 2, 3, 4]));

            let results = linked_list.get(linked_list.len() - 1);
            let borrowed_head = linked_list.head.as_ref().unwrap().borrow();
            let borrowed_tail = linked_list.tail.as_ref().unwrap().borrow();

            assert_eq!(linked_list.len(), 4);

            assert_eq!(results, Some(4));
            assert_eq!(borrowed_head.value, 1);
            assert_eq!(borrowed_tail.value, 4);
        }

        #[test]
        fn returns_proper_value_at_index() {
            let linked_list: DoublyLinkedList<u8> =
                DoublyLinkedList::from(Vec::from([1_u8, 2, 3, 4]));

            let results = linked_list.get(2);
            let borrowed_head = linked_list.head.as_ref().unwrap().borrow();
            let borrowed_tail = linked_list.tail.as_ref().unwrap().borrow();

            assert_eq!(linked_list.len(), 4);

            assert_eq!(results, Some(3));
            assert_eq!(borrowed_head.value, 1);
            assert_eq!(borrowed_tail.value, 4);
        }
    }

    mod set {
        use super::*;

        #[test]
        fn returns_none_when_called_on_empty_list() {
            let mut linked_list: DoublyLinkedList<u8> = DoublyLinkedList::new();

            let results = linked_list.set(0, 1);

            assert_eq!(linked_list.len(), 0);

            assert!(results.is_none());
            assert!(linked_list.head.is_none());
            assert!(linked_list.tail.is_none());
        }

        #[test]
        fn returns_none_when_called_with_index_greater_then_len() {
            let mut linked_list: DoublyLinkedList<u8> =
                DoublyLinkedList::from(Vec::from([1_u8, 2, 3, 4]));

            let results = linked_list.set(5, 6);
            let borrowed_head = linked_list.head.as_ref().unwrap().borrow();
            let borrowed_tail = linked_list.tail.as_ref().unwrap().borrow();

            assert_eq!(linked_list.len(), 4);

            assert!(results.is_none());
            assert_eq!(borrowed_head.value, 1);
            assert_eq!(borrowed_tail.value, 4);
        }

        #[test]
        fn returns_head_value_when_index_0_passed_in() {
            let mut linked_list: DoublyLinkedList<u8> =
                DoublyLinkedList::from(Vec::from([1_u8, 2, 3, 4]));

            let results = linked_list.set(0, 6);
            let borrowed_head = linked_list.head.as_ref().unwrap().borrow();
            let borrowed_tail = linked_list.tail.as_ref().unwrap().borrow();

            assert_eq!(linked_list.len(), 4);

            assert_eq!(results, Some(1));
            assert_eq!(borrowed_head.value, 6);
            assert_eq!(borrowed_tail.value, 4);
        }

        #[test]
        fn returns_tail_value_when_len_minus_1_passed_in() {
            let mut linked_list: DoublyLinkedList<u8> =
                DoublyLinkedList::from(Vec::from([1_u8, 2, 3, 4]));

            let results = linked_list.set(linked_list.len() - 1, 5);
            let borrowed_head = linked_list.head.as_ref().unwrap().borrow();
            let borrowed_tail = linked_list.tail.as_ref().unwrap().borrow();

            assert_eq!(linked_list.len(), 4);

            assert_eq!(results, Some(4));
            assert_eq!(borrowed_head.value, 1);
            assert_eq!(borrowed_tail.value, 5);
        }

        #[test]
        fn returns_proper_value_at_index() {
            let mut linked_list: DoublyLinkedList<u8> =
                DoublyLinkedList::from(Vec::from([1_u8, 2, 3, 4]));

            let results = linked_list.set(2, 8);
            let borrowed_head = linked_list.head.as_ref().unwrap().borrow();
            let borrowed_tail = linked_list.tail.as_ref().unwrap().borrow();

            assert_eq!(linked_list.len(), 4);

            assert_eq!(results, Some(3));
            assert_eq!(borrowed_head.value, 1);
            assert_eq!(borrowed_tail.value, 4);
        }
    }

    mod remove {
        use super::*;

        #[test]
        fn returns_none_when_called_on_empty_list() {
            let mut linked_list: DoublyLinkedList<u8> = DoublyLinkedList::new();

            let results = linked_list.remove(0);

            assert_eq!(linked_list.len(), 0);

            assert!(results.is_none());
            assert!(linked_list.head.is_none());
            assert!(linked_list.tail.is_none());
        }

        #[test]
        fn returns_none_when_called_with_greater_then_length() {
            let mut linked_list: DoublyLinkedList<u8> = DoublyLinkedList::from(Vec::from([1_u8]));

            let results = linked_list.remove(1);
            let borrowed_head = linked_list.head.as_ref().unwrap().borrow();
            let borrowed_tail = linked_list.tail.as_ref().unwrap().borrow();

            assert_eq!(linked_list.len(), 1);

            assert!(results.is_none());
            assert_eq!(borrowed_head.value, 1);
            assert_eq!(borrowed_tail.value, 1);
        }

        #[test]
        fn returns_value_and_empty_list_if_only_1_value() {
            let mut linked_list = DoublyLinkedList::from(Vec::from([1_u8]));

            let results = linked_list.remove(0);
            assert_eq!(linked_list.len(), 0);
            assert_eq!(results, Some(1));
            assert!(linked_list.head.is_none());
            assert!(linked_list.tail.is_none());
        }

        #[test]
        fn returns_value_and_multiple_times() {
            let mut linked_list = DoublyLinkedList::from(Vec::from([1_u8, 2, 3, 4]));

            let result = linked_list.remove(2);

            assert_eq!(linked_list.len(), 3);
            assert_eq!(result, Some(3));

            let result = linked_list.remove(1);

            assert_eq!(linked_list.len(), 2);
            assert_eq!(result, Some(2));

            let result = linked_list.remove(1);

            assert_eq!(linked_list.len(), 1);
            assert_eq!(result, Some(4));

            let result = linked_list.remove(0);

            assert_eq!(linked_list.len(), 0);
            assert_eq!(result, Some(1));
            assert!(linked_list.tail.is_none());
            assert!(linked_list.head.is_none());
        }
    }
}
