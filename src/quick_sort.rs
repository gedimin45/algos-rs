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
