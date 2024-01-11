struct Layer<T> {
    value: T,
    next_layer: Option<Box<Layer<T>>>,
}

pub struct Stack<T> {
    top_layer: Option<Box<Layer<T>>>,
    size: usize,
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self {
            top_layer: Default::default(),
            size: Default::default(),
        }
    }
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T> From<Vec<T>> for Stack<T> {
    fn from(mut value: Vec<T>) -> Self {
        let mut stack = Stack::new();

        while let Some(val) = value.pop() {
            stack.push(val);
        }

        stack
    }
}

impl<T: Clone> From<&[T]> for Stack<T> {
    fn from(value: &[T]) -> Self {
        let mut stack = Stack::new();

        value.iter().rev().for_each(|val| {
            stack.push(val.clone());
        });

        stack
    }
}

impl<T> Stack<T> {
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, value: T) {
        let new_layer = Box::new(Layer {
            value,
            next_layer: self.top_layer.take(),
        });

        self.top_layer = Some(new_layer);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        let top_layer = self.top_layer.take();

        if let Some(layer) = top_layer {
            self.top_layer = layer.next_layer;
            self.size -= 1;

            Some(layer.value)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod integration {
        use super::*;

        #[test]
        fn pop_and_push() {
            let mut stack: Stack<i32> = Stack::new();
            stack.push(5);
            stack.push(4);
            stack.push(3);
            stack.push(2);
            stack.push(1);

            let result = stack.pop();
            assert_eq!(result, Some(1));

            let result = stack.pop();
            assert_eq!(result, Some(2));

            let result = stack.pop();
            assert_eq!(result, Some(3));

            let result = stack.pop();
            assert_eq!(result, Some(4));

            let result = stack.pop();
            assert_eq!(result, Some(5));

            let result = stack.pop();
            assert!(result.is_none());
        }
    }

    mod pop {
        use super::*;

        #[test]
        fn pop_on_empty_stack_returns_none() {
            let mut stack: Stack<i32> = Stack::new();

            let results = stack.pop();

            assert!(results.is_none());
        }

        #[test]
        fn pop_on_stack_with_1_layer_returns_layer_and_sets_size_to_0() {
            let mut stack: Stack<i32> = Stack::from(vec![5]);

            let results = stack.pop();

            assert_eq!(results, Some(5));
            assert_eq!(stack.size(), 0);
        }

        #[test]
        fn can_push_multiple_times() {
            let mut stack = Stack::new();

            stack.push(5);
            stack.push(6);
            stack.push(7);
            stack.push(8);

            assert_eq!(stack.size, 4);
        }
    }
    mod push {
        use super::*;

        #[test]
        fn can_push_on_empty_stack() {
            let mut stack = Stack::new();

            stack.push(5);

            assert_eq!(stack.size, 1);
        }

        #[test]
        fn can_push_multiple_times() {
            let mut stack = Stack::new();

            stack.push(5);
            stack.push(6);
            stack.push(7);
            stack.push(8);

            assert_eq!(stack.size, 4);
        }
    }
}
