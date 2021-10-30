mod merge_sort;
mod quick_sort;

use merge_sort::merge_sort;
use quick_sort::quick_sort;

fn main() {
    for right in 0..5 {
        println!("{}", right)
    }

    let mut array_for_merge_sort = [8, 1, 10, 2, 3];
    merge_sort(&mut array_for_merge_sort);
    println!("{:?}", array_for_merge_sort);

    let mut array_for_quick_sort = [8, 1, 10, 2, 3];
    quick_sort(&mut array_for_quick_sort);
    println!("{:?}", array_for_quick_sort);
}
