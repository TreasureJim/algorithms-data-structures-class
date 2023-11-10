#![allow(unused)]

use std::mem;

pub fn quick_sort_recursive<T: PartialOrd + Copy>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let len = arr.len();

    let pivot_index = 0;
    let pivot = arr[0];

    let mut lp = pivot_index + 1;

    for i in (pivot_index + 1)..len {
        if arr[i] <= pivot {
            // swap i and lp
            let temp = arr[i];
            arr[i] = arr[lp];
            arr[lp] = temp;
            lp += 1;
        }
    }
    //swap pivot index, lp - 1
    let temp = arr[lp - 1];
    arr[lp - 1] = arr[pivot_index];
    arr[pivot_index] = temp;

    quick_sort_recursive(&mut arr[0..lp]);
    quick_sort_recursive(&mut arr[lp..len])
}

// fn partition<T: PartialOrd>(mut arr: &Vec<T>, low: usize, high: usize) {
//
// }

#[cfg(test)]
mod test {
    use super::quick_sort_recursive;
    use crate::test_helpers::generate_random_list;

    #[test]
    fn test1() {
        let mut unsort = [5, 8, 10, 1, 2];
        let mut sorted = unsort.clone();
        sorted.sort();
        quick_sort_recursive(&mut unsort);
        assert_eq!(unsort, sorted);
    }

    #[test]
    fn test_random() {
        let mut r = generate_random_list(10, 0, 100);
        quick_sort_recursive(&mut r.0);
        assert_eq!(r.0, r.1);
    }
}
