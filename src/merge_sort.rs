pub fn merge_sort(array: &mut [u8]) {
    if array.len() <= 1 {
        return;
    }

    let pivot = array.len() / 2;

    merge_sort(&mut array[0..pivot]);
    merge_sort(&mut array[pivot..]);

    let merged = merge(&array[0..pivot], &array[pivot..]);

    array.copy_from_slice(&merged);
}

fn merge(left: &[u8], right: &[u8]) -> Vec<u8> {
    let mut left_idx = 0;
    let mut right_idx = 0;
    let mut temp = Vec::new();

    while left_idx < left.len() && right_idx < right.len() {
        if left[left_idx] < right[right_idx] {
            temp.push(left[left_idx]);
            left_idx += 1;
        } else {
            temp.push(right[right_idx]);
            right_idx += 1;
        }
    }

    temp.extend_from_slice(&left[left_idx..]);
    temp.extend_from_slice(&right[right_idx..]);

    temp
}

#[cfg(test)]
mod tests {
    use crate::merge_sort;

    #[test]
    fn it_works() {
        let mut array = [8, 1, 10, 2, 3];
        merge_sort(&mut array);

        assert_eq!(array, [1, 2, 3, 8, 10]);
    }
}
