pub fn quick_sort(array: &mut [u8]) {
    if array.len() <= 1 {
        return;
    }

    let p = partition(array);

    quick_sort(&mut array[0..p]);
    quick_sort(&mut array[(p+1)..]);
}

fn partition(array: &mut [u8]) -> usize {
    let pivot = *array.last().unwrap();

    let mut index = 0;

    for i in 0..array.len() {
        if array[i] < pivot {
            array.swap(i, index);
            index += 1;
        }
    }

    array.swap(index, array.len()-1);

    index
}

#[cfg(test)]
mod tests {
    use crate::quick_sort::quick_sort;

    #[test]
    fn single_element() {
        let mut array = [8];
        quick_sort(&mut array);
        assert_eq!(array, [8]);
    }

    #[test]
    fn two_elements() {
        let mut array = [8, 1];
        quick_sort(&mut array);
        assert_eq!(array, [1, 8]);
    }

    #[test]
    fn two_same_elements() {
        let mut array = [8, 1, 1];
        quick_sort(&mut array);
        assert_eq!(array, [1, 1, 8]);
    }

    #[test]
    fn odd_no_of_elements() {
        let mut array = [8, 1, 10, 2, 3];
        quick_sort(&mut array);
        assert_eq!(array, [1, 2, 3, 8, 10]);
    }

    #[test]
    fn four_elements_with_two_same() {
        let mut array = [10, 2, 3, 2];
        quick_sort(&mut array);
        assert_eq!(array, [2, 2, 3, 10]);
    }

    #[test]
    fn even_no_of_elements() {
        let mut array = [8, 1, 10, 2, 3, 2];
        quick_sort(&mut array);
        assert_eq!(array, [1, 2, 2, 3, 8, 10]);
    }
}
