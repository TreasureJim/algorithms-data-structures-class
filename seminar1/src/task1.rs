#![allow(unused)]

use std::{mem, ptr, vec};

pub fn quick_sort_simple_recursive<T: PartialOrd + Copy>(mut arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let len = arr.len();

    let pivot_index = 0;
    let pivot = arr[pivot_index];

    let mut n_pivot = 0;
    let mut smaller: Vec<T> = vec![];
    let mut bigger: Vec<T> = vec![];

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

pub fn insertion_sort_iter<T: PartialOrd>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in (0..i).rev() {
            if arr[j] >= arr[j + 1] {
                arr.swap(j, j + 1);
            } else {
                break;
            }
        }
    }
}

pub fn insertion_sort_recursive<T: PartialOrd>(arr: &mut[T]) {
    let len = arr.len();
    if len > 2 {
        insertion_sort_recursive(&mut arr[..len - 1]);
    }

    for i in (0..len - 1).rev() {
        if arr[i] >= arr[i + 1] {
            arr.swap(i, i + 1);
        } else {
            break;
        }
    }
}

const INSERTION_LIMIT: usize = 15;


pub fn quick_sort_recursive<T: PartialOrd + Copy>(arr: &mut [T], pivot_fn: &dyn Fn(&[T]) -> usize) {
    if arr.len() <= INSERTION_LIMIT {
        return insertion_sort_iter(arr);
    }

    let len = arr.len();

    // choosing pivot
    let pivot_index = pivot_fn(arr);
    let pivot = arr[pivot_index];

    // partition

    // 	1. swap pivot element with last element
    arr.swap(pivot_index, len - 1);
    // 2. LP = 0, RP = len - 2 (one before the last element)
    let mut lp: isize = 0;
    let mut rp: isize = len as isize - 2;

    while (lp <= rp) {
        // (LP and RP have stopped now) LP is point at large element and RP is pointing at small element
        if arr[lp as usize] > pivot && arr[rp as usize] < pivot {
            arr.swap(lp as usize, rp as usize);
            lp += 1;
            rp = rp.saturating_sub(1);
        }
        // otherwise move pointers
        else if arr[lp as usize] <= pivot {
            lp += 1;
        } else if arr[rp as usize] >= pivot {
            rp = rp.saturating_sub(1);
        }
    }

    dbg!(lp, rp);

    // swap pivot back into place
    arr.swap(len - 1, lp as usize);

    // recursive
    quick_sort_recursive(&mut arr[..lp as usize], pivot_fn);
    quick_sort_recursive(&mut arr[lp as usize + 1..], pivot_fn);
}

fn median_pivot<T: PartialOrd>(arr: &[T]) -> usize {
    assert!(arr.len() >= 3);

    let low = &arr[0];
    let middle_i = arr.len() / 2;
    let middle = &arr[middle_i];
    let high_i = arr.len() - 1;
    let high = arr.last().unwrap();
    
    if *low > *middle {
        if *middle > *high {
            middle_i
        } else if *low > *high {
            high_i
        } else {
            0
        }
    } else {
        if *low > *high {
            0
        } else if *middle > *high {
            high_i
        } else {
            middle_i
        }
    }
}

#[cfg(test)]
mod test {
    use super::quick_sort_simple_recursive;
    use crate::{
        task1::{insertion_sort_iter, quick_sort_recursive, median_pivot, insertion_sort_recursive},
        test_helpers::generate_random_list,
    };

    #[test]
    fn median_pivot_test() {
        assert_eq!(median_pivot(&[10, 3, 5]), 2);
        assert_eq!(median_pivot(&[1, 20, 5, 10, 30]), 2);
        assert_eq!(median_pivot(&[20, 1, 1, 50, 5]), 4);
    }


    #[test]
    fn insertion_iter_test() {
        let mut r = generate_random_list(1_000_000, 0, 10);
        // dbg!(&r.0);
        insertion_sort_iter(&mut r.0);
        assert_eq!(r.0, r.1);
    }

    #[test]
    fn insertion_recur_test() {
        let mut r = generate_random_list(1_000_000, 0, 1000);
        // dbg!(&r.0);
        insertion_sort_recursive(&mut r.0);
        assert_eq!(r.0, r.1);
    }

    #[test]
    fn quick_sort_recursive_test1() {
        let mut unsort = [8, 4, 2, 4, 0, 9, 9, 4, 3, 0];
        let mut sorted = unsort.clone();
        sorted.sort();
        quick_sort_recursive(&mut unsort, &median_pivot);
        assert_eq!(unsort, sorted);
    }

    #[test]
    fn quicksort_recursive_random_test() {
        let mut r = generate_random_list(1_000_000, 0, 10);
        // dbg!(&r.0);
        quick_sort_recursive(&mut r.0, &median_pivot);
        assert_eq!(r.0, r.1);
    }
}
