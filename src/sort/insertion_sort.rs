pub fn sort<T: PartialOrd + Clone>(array: &mut [T]) {
    for i in 1..array.len() {
        let current_value = array[i].clone();

        let mut j = i - 1;
        while array[j] > current_value {
            array.swap(j + 1, j);
            let Some(new_j) = j.checked_sub(1) else {
                break;
            };

            j = new_j;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::insertion_sort;

    #[test]
    fn sort_sorted_array() {
        let mut array_1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut array_2 = array_1.clone();

        array_2.sort();
        insertion_sort::sort(array_1.as_mut_slice());

        assert_eq!(array_1, array_2);
    }

    #[test]
    fn sort_backwards_array() {
        let mut array_1 = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let mut array_2 = array_1.clone();

        array_2.sort();
        insertion_sort::sort(array_1.as_mut_slice());

        assert_eq!(array_1, array_2);
    }

    #[test]
    fn sort_random_array() {
        let mut array_1 = vec![10, 9, 22, 10, 15, 5, 20, 1, 2, 1];
        let mut array_2 = array_1.clone();

        array_2.sort();
        insertion_sort::sort(array_1.as_mut_slice());

        assert_eq!(array_1, array_2);
    }
}
