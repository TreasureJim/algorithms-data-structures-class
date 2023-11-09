#![allow(unused)]

use std::mem;

pub fn quick_sort_recursive<T: PartialOrd + Copy>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let len = arr.len();
    let mut low = 0;
    let mut high = arr.len() - 1;

    let pivot_index = len / 2;
    let pivot = arr[pivot_index];

    while low < high {
        while arr[low] < pivot {
            low += 1;
        }
        while arr[high] > pivot {
            high -= 1;
        }

        if arr[low] >= pivot && arr[high] <= pivot {
        // swap
        let temp = arr[low];
        arr[low] = arr[high];
        arr[high] = temp;
        low += 1;
        high -= 1;
        }
    }

    assert!(low == high);

    quick_sort_recursive(&mut arr[0..high]);
    quick_sort_recursive(&mut arr[high..len])
}

// fn partition<T: PartialOrd>(mut arr: &Vec<T>, low: usize, high: usize) {
//
// }

#[cfg(test)]
mod test {
    use super::quick_sort_recursive;
    use crate::test_helpers::generate_random_list;

    #[test]
    fn test_file() {
        let mut r = generate_random_list(10, 0, 100);
        quick_sort_recursive(&mut r.0);
        dbg!(&r.0, &r.1);
        assert_eq!(r.0, r.1);
    }
}
