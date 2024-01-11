pub fn sort<T: PartialOrd>(array: &mut [T]) {
    for i in (0..array.len()).rev() {
        let mut no_swap = true;
        for j in 0..i {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
                no_swap = false;
            }
        }

        if no_swap {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sort::bubble_sort;

    #[test]
    fn sort_sorted_array() {
        let mut array_1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut array_2 = array_1.clone();

        array_2.sort();
        bubble_sort::sort(array_1.as_mut_slice());

        assert_eq!(array_1, array_2);
    }

    #[test]
    fn sort_backwards_array() {
        let mut array_1 = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let mut array_2 = array_1.clone();

        array_2.sort();
        bubble_sort::sort(array_1.as_mut_slice());

        assert_eq!(array_1, array_2);
    }

    #[test]
    fn sort_random_array() {
        let mut array_1 = vec![10, 9, 22, 10, 15, 5, 20, 1, 2, 1];
        let mut array_2 = array_1.clone();

        array_2.sort();
        bubble_sort::sort(array_1.as_mut_slice());

        assert_eq!(array_1, array_2);
    }
}
