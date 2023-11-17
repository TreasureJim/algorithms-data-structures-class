#![allow(unused)]

use std::{mem, ptr, vec};

use rand::Rng;

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

pub fn insertion_sort_recursive<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    if len == 0 {
        return;
    } else if len > 2 {
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

fn partition<T: PartialOrd + Copy>(arr: &mut [T], pivot_index: usize) -> (isize, isize) {
    let len = arr.len();
    let pivot = arr[pivot_index];

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

    // swap pivot back into place
    arr.swap(len - 1, lp as usize);
    (lp, rp)
}

pub fn quick_sort_recursive<T: PartialOrd + Copy>(arr: &mut [T], pivot_fn: &dyn Fn(&[T]) -> usize) {
    let len = arr.len();

    if len <= INSERTION_LIMIT {
        return insertion_sort_iter(arr);
    }

    // choosing pivot
    let pivot_index = pivot_fn(arr);

    let (lp, rp) = partition(arr, pivot_index);

    // recursive
    quick_sort_recursive(&mut arr[..lp as usize], pivot_fn);
    quick_sort_recursive(&mut arr[lp as usize + 1..], pivot_fn);
}

pub fn quicksort_iterative<T: PartialOrd + Copy>(arr: &mut [T], pivot_fn: &dyn Fn(&[T]) -> usize) {
    let mut stack = vec![arr];

    while let Some(sub_arr) = stack.pop() {
        if sub_arr.len() <= INSERTION_LIMIT {
            insertion_sort_iter(sub_arr);
            continue;
        }

        let (lp, rp) = partition(sub_arr, pivot_fn(sub_arr));

        let (tail, head) = sub_arr.split_at_mut(lp as usize);

        // if elements on left side of pivot
        if tail.len() > 0 {
            stack.push(tail);
        }

        // if elements on right side of pivot
        if head.len() > 1 {
            stack.push(&mut head[1..]);
        }
    }
}

pub fn median_pivot<T: PartialOrd>(arr: &[T]) -> usize {
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

pub fn first_pivot<T>(arr: &[T]) -> usize {
    0
}

pub fn random_pivot<T>(arr: &[T]) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..arr.len())
}

#[cfg(test)]
mod benchmarks {
    use std::time::Instant;

    use crate::test_helpers;

    use super::*;

    #[test]
    fn insertion_iter() {
        let test_arr = test_helpers::read_test_file();

        let mut size = 10;
        while size < 1_000_000 {
            let mut sub_arr = Vec::from(&test_arr[..size]);

            let start = Instant::now();
            insertion_sort_iter(&mut sub_arr);
            let length = start.elapsed();
            dbg!(size, length);

            size *= 10;
        }
    }

    #[test]
    fn quicksort_iter() {
        let test_arr = test_helpers::read_test_file();

        let mut size = 10;
        while size < 1_000_000 {
            let mut sub_arr = Vec::from(&test_arr[..size]);

            let start = Instant::now();
            quicksort_iterative(&mut sub_arr, &median_pivot);
            let length = start.elapsed();
            dbg!(size, length);

            size *= 10;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Insertion Sort Tests
    #[test]
    fn test_insertion_sort_iter_empty() {
        let mut arr: Vec<i32> = Vec::new();
        insertion_sort_iter(&mut arr);
        assert_eq!(arr, Vec::<i32>::new());
    }

    #[test]
    fn test_insertion_sort_iter_odd_elements() {
        let mut arr = vec![3, 2, 1, 4, 5];
        insertion_sort_iter(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_insertion_sort_iter_even_elements() {
        let mut arr = vec![3, 2, 1, 4];
        insertion_sort_iter(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_insertion_sort_iter_large_array() {
        let mut arr: Vec<i32> = (1..=1000).rev().collect();
        insertion_sort_iter(&mut arr);
        assert_eq!(arr, (1..=1000).collect::<Vec<i32>>());
    }

    #[test]
    fn test_insertion_sort_recursive_empty() {
        let mut arr: Vec<i32> = Vec::new();
        insertion_sort_recursive(&mut arr);
        assert_eq!(arr, Vec::<i32>::new());
    }

    #[test]
    fn test_insertion_sort_recursive_odd_elements() {
        let mut arr = vec![3, 2, 1, 4, 5];
        insertion_sort_recursive(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_insertion_sort_recursive_even_elements() {
        let mut arr = vec![3, 2, 1, 4];
        insertion_sort_recursive(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_insertion_sort_recursive_large_array() {
        let mut arr: Vec<i32> = (1..=1000).rev().collect();
        insertion_sort_recursive(&mut arr);
        assert_eq!(arr, (1..=1000).collect::<Vec<i32>>());
    }

    // Quick Sort Tests (Recursive)
    #[test]
    fn test_quick_sort_recursive_odd_elements_median_pivot() {
        let mut arr = vec![3, 2, 1, 4, 5];
        quick_sort_recursive(&mut arr, &median_pivot);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quick_sort_recursive_odd_elements_first_pivot() {
        let mut arr = vec![3, 2, 1, 4, 5];
        quick_sort_recursive(&mut arr, &first_pivot);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quick_sort_recursive_odd_elements_random_pivot() {
        let mut arr = vec![3, 2, 1, 4, 5];
        quick_sort_recursive(&mut arr, &random_pivot);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quick_sort_recursive_even_elements_median_pivot() {
        let mut arr = vec![3, 2, 1, 4];
        quick_sort_recursive(&mut arr, &median_pivot);
        assert_eq!(arr, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_quick_sort_recursive_even_elements_first_pivot() {
        let mut arr = vec![3, 2, 1, 4];
        quick_sort_recursive(&mut arr, &first_pivot);
        assert_eq!(arr, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_quick_sort_recursive_even_elements_random_pivot() {
        let mut arr = vec![3, 2, 1, 4];
        quick_sort_recursive(&mut arr, &random_pivot);
        assert_eq!(arr, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_quick_sort_recursive_large_array_median_pivot() {
        let mut arr: Vec<i32> = (1..=1000).rev().collect();
        quick_sort_recursive(&mut arr, &median_pivot);
        assert_eq!(arr, (1..=1000).collect::<Vec<i32>>());
    }

    #[test]
    fn test_quick_sort_recursive_large_array_first_pivot() {
        let mut arr: Vec<i32> = (1..=1000).rev().collect();
        quick_sort_recursive(&mut arr, &first_pivot);
        assert_eq!(arr, (1..=1000).collect::<Vec<i32>>());
    }

    #[test]
    fn test_quick_sort_recursive_large_array_random_pivot() {
        let mut arr: Vec<i32> = (1..=1000).rev().collect();
        quick_sort_recursive(&mut arr, &random_pivot);
        assert_eq!(arr, (1..=1000).collect::<Vec<i32>>());
    }

    // Quick Sort Tests (Iterative)
    #[test]
    fn test_quick_sort_iterative_odd_elements_median_pivot() {
        let mut arr = vec![3, 2, 1, 4, 5];
        quicksort_iterative(&mut arr, &median_pivot);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quick_sort_iterative_odd_elements_first_pivot() {
        let mut arr = vec![3, 2, 1, 4, 5];
        quicksort_iterative(&mut arr, &first_pivot);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quick_sort_iterative_odd_elements_random_pivot() {
        let mut arr = vec![3, 2, 1, 4, 5];
        quicksort_iterative(&mut arr, &random_pivot);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quick_sort_iterative_even_elements_median_pivot() {
        let mut arr = vec![3, 2, 1, 4];
        quicksort_iterative(&mut arr, &median_pivot);
        assert_eq!(arr, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_quick_sort_iterative_even_elements_first_pivot() {
        let mut arr = vec![3, 2, 1, 4];
        quicksort_iterative(&mut arr, &first_pivot);
        assert_eq!(arr, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_quick_sort_iterative_even_elements_random_pivot() {
        let mut arr = vec![3, 2, 1, 4];
        quicksort_iterative(&mut arr, &random_pivot);
        assert_eq!(arr, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_quick_sort_iterative_large_array_median_pivot() {
        let mut arr: Vec<i32> = (1..=1000).rev().collect();
        quicksort_iterative(&mut arr, &median_pivot);
        assert_eq!(arr, (1..=1000).collect::<Vec<i32>>());
    }

    #[test]
    fn test_quick_sort_iterative_large_array_first_pivot() {
        let mut arr: Vec<i32> = (1..=1000).rev().collect();
        quicksort_iterative(&mut arr, &first_pivot);
        assert_eq!(arr, (1..=1000).collect::<Vec<i32>>());
    }

    #[test]
    fn test_quick_sort_iterative_large_array_random_pivot() {
        let mut arr: Vec<i32> = (1..=1000).rev().collect();
        quicksort_iterative(&mut arr, &random_pivot);
        assert_eq!(arr, (1..=1000).collect::<Vec<i32>>());
    }

    // my tests

    use super::{
        insertion_sort_iter, insertion_sort_recursive, median_pivot, quicksort_iterative,
        quick_sort_recursive,
    };
    use crate::test_helpers::generate_random_list;

    #[test]
    fn median_pivot_test() {
        assert_eq!(median_pivot(&[10, 3, 5]), 2);
        assert_eq!(median_pivot(&[1, 20, 5, 10, 30]), 2);
        assert_eq!(median_pivot(&[20, 1, 1, 50, 5]), 4);
    }

    #[test]
    fn insertion_iter_random_test() {
        let mut r = generate_random_list(100_000, 0, 1000);
        // dbg!(&r.0);
        insertion_sort_iter(&mut r.0);
        assert_eq!(r.0, r.1);
    }

    #[test]
    fn insertion_recur_random_test() {
        let mut r = generate_random_list(100_000, 0, 1000);
        // dbg!(&r.0);
        insertion_sort_recursive(&mut r.0);
        assert_eq!(r.0, r.1);
    }

    #[test]
    fn quicksort_iterative_random_test() {
        let mut r = generate_random_list(1_000_000, 0, 1000);
        // dbg!(&r.0);
        quicksort_iterative(&mut r.0, &median_pivot);
        assert_eq!(r.0, r.1);
    }

    #[test]
    fn quicksort_recursive_random_test() {
        let mut r = generate_random_list(1_000_000, 0, 10);
        // dbg!(&r.0);
        quick_sort_recursive(&mut r.0, &median_pivot);
        assert_eq!(r.0, r.1);
    }
}
