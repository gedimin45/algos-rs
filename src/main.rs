mod merge_sort;
mod quick_sort;
mod count_min_sketch;

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

#[cfg(test)]
mod tests {
    fn test<F: Fn(Vec<u8>) -> Vec<u8>>(test_name: &str, array: Vec<u8>, expected: Vec<u8>, algorithm: F, algorithm_name: &str) {
        assert_eq!(algorithm(array), expected, "Algorithm '{}' test failed for test case '{}'", algorithm_name, test_name);
    }

    #[test]
    fn it_works() {
        let merge_sort = Box::new(|input: Vec<u8>| -> Vec<u8> {
            let mut input = input.clone();
            crate::merge_sort::merge_sort(&mut input);
            input
        }) as Box<dyn Fn(Vec<u8>) -> Vec<u8>>;
        let quick_sort = Box::new(|input: Vec<u8>| -> Vec<u8> {
            let mut input = input.clone();
            crate::quick_sort::quick_sort(&mut input);
            input
        }) as Box<dyn Fn(Vec<u8>) -> Vec<u8>>;

        let suts = std::collections::HashMap::from([
            ("merge_sort", &merge_sort),
            ("quick_sort", &quick_sort),
        ]);

        for (name, algo) in suts {
            test("single_element", vec![8], vec![8], algo, name);
            test("two_elements", vec![8, 1], vec![1, 8], algo, name);
            test("two_same_elements", vec![8, 1, 1], vec![1, 1, 8], algo, name);
            test("odd_no_of_elements", vec![8, 1, 10, 2, 3], vec![1, 2, 3, 8, 10], algo, name);
            test("four_elements_with_two_same", vec![10, 2, 3, 2], vec![2, 2, 3, 10], algo, name);
            test("even_no_of_elements", vec![8, 1, 10, 2, 3, 2], vec![1, 2, 2, 3, 8, 10], algo, name);
        }
    }
}
