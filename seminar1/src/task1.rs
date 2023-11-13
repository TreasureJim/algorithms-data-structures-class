#![allow(unused)]

use std::mem;

fn swap<T>(arr: &mut [T], i1: usize, i2: usize) {
    if arr.len() <= 1 {
        return;
    } else if arr.len() == 2 {
        swap(arr, 0, 1);
    }

    if i1 > i2 {
        let (init, tail) = arr.split_at_mut(i1);
        mem::swap(&mut init[i2], &mut tail[0]);
    } else {
        let (init, tail) = arr.split_at_mut(i2);
        mem::swap(&mut init[i1], &mut tail[0]);
    }
}

pub fn quick_sort_recursive<T: PartialOrd + Copy>(arr: &mut [T]) {
    // TODO: for small arrays use different sorting algorithm
    if arr.len() <= 1 {
        return;
    }

    let len = arr.len();

    let pivot_index = 0;
    let pivot = arr[pivot_index];

    // partition
    // 	1. swap pivot element with last element
    // 2. LP = 0, RP = len - 2 (one before the last element)
    // 3. while LP <= RP && arr[LP] < pivot: LP++
    // 4. while RP >= pivot: RP --
    // 5. if LP < RP: swap LP and RP elements
    // 6. stop when LP and RP pass each other
    // every element before LP is now small
    // for n < 10 || 20 use insertion sort

    swap(arr, pivot_index, len - 1);
    let mut lp = 0;
    let mut rp = len - 2;
    let mut change = true;

    while (lp <= rp) {
        while arr[lp] < pivot && lp <= rp {
            lp += 1;
        }
        while arr[rp] >= pivot {
            rp -= 1;
        }

        // (LP and RP have stopped now) LP is point at large element and RP is pointing at small element
        if lp < rp {
            swap(arr, lp, rp);
        }
    }

    swap(arr, len - 1, lp);

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
