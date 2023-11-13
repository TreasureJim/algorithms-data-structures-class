#![allow(unused)]

use std::{mem, vec, ptr};

pub fn quick_sort_simple_recursive<T: PartialOrd + Copy>(mut arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let len = arr.len();

    let pivot_index = 0;
    let pivot = arr[0];

    let mut n_pivot = 0;
    let mut smaller: Vec<T> = vec![];
    let mut bigger : Vec<T> = vec![];

    for x in arr.iter() {
        if *x < pivot {
            smaller.push(*x);
        } else if *x > pivot {
            bigger.push(*x);
        } else {
            n_pivot += 1;
        }
    }
    
    quick_sort_simple_recursive(&mut smaller);
    quick_sort_simple_recursive(&mut bigger);


    let mut index = 0;
    arr[..smaller.len()].copy_from_slice(&smaller);
    index += smaller.len();
    arr[index..n_pivot + index].fill(pivot);
    index += n_pivot;
    arr[index..].clone_from_slice(&bigger);
}

// fn partition<T: PartialOrd>(mut arr: &Vec<T>, low: usize, high: usize) {
//
// }

#[cfg(test)]
mod test {
    use super::quick_sort_simple_recursive;
    use crate::test_helpers::generate_random_list;

    #[test]
    fn test1() {
        let mut unsort = [5, 8, 10, 1, 2];
        let mut sorted = unsort.clone();
        sorted.sort();
        quick_sort_simple_recursive(&mut unsort);
        assert_eq!(unsort, sorted);
    }

    #[test]
    fn test_random() {
        let mut r = generate_random_list(10_000_000, 0, 100);
        quick_sort_simple_recursive(&mut r.0);
        assert_eq!(r.0, r.1);
    }
}
