mod merge_sort;

use merge_sort::merge_sort;

fn main() {
    let mut array = [8, 1, 10, 2, 3];
    merge_sort(&mut array);

    println!("{:?}", array);
}
