#![allow(unused)]

// Task 4
// Implement the following exercise from the course literature: [1] Ex.3.6, p.116.
// The Josephus problem is the following game: N people, numbered 1 to N, are sitting in a circle. Starting at person 1, a hot potato is passed. After M passes, the person holding the hot potato is eliminated, the circle closes ranks, and the game continues with the person who was sitting after the eliminated person picking up the hot potato. The last remaining person wins.
// Thus, if M = 0 and N = 5, players are eliminated in order and player 5 wins. If M = 1 and N = 5, the order of elimination is 2, 4, 1, 5.
// a. Write a program to solve the Josephus problem for general values of M and N.
//  Try to make your program as efficient as possible. Make sure you dispose of
//  cells.
//  Implement the program using three methods:
//  - one with ArrayList
//  - one using ArrayList with Iterator
//  - one with your own LinkedList
//  - one using Linked list with Iterator
// b. Compare the running time of your program depending on both data structures.
// Present the results in a table and a diagram (plot). Think about different input
// sizes! Can you find a mathematical formula to predict the output for any input?
// c. Write a reflection: When should LinkedList be used over ArrayList and vice-
// versa?

use crate::task3::{LinkedList, LinkedListNode};

fn generate_arraylist(n: usize) -> Vec<usize> {
    assert!(n > 0);

    let mut arr = vec![0; n];
    for (index, elem) in arr.iter_mut().enumerate() {
        *elem = index + 1;
    }
    arr
}

//  - one with ArrayList
pub fn josephus_arraylist(mut arr: Vec<usize>, m: usize) -> usize {
    if m == 0 {
        return *arr.last().unwrap();
    }

    let mut count = 0;
    while arr.len() > 1 && !arr.is_empty() {
        let mut index = 0;
        while index < arr.len() {
            if count == m {
                count = 0;
                arr.remove(index);
                continue;
            }

            count += 1;
            index += 1;
        }
    }
    arr[0]
}

//  - one using ArrayList with Iterator
pub fn josephus_arraylist_iter(mut arr: Vec<usize>, m: usize) -> usize {
    if m == 0 {
        return *arr.last().unwrap();
    }

    let mut count = 0;
    while arr.len() > 1 && !arr.is_empty() {
        arr.retain(|_| {
            if count == m {
                count = 0;
                return false;
            }
            count += 1;
            true
        });
    }
    arr[0]
}

fn generate_linkedlist(n: usize) -> LinkedList<usize> {
    LinkedList::from_list(generate_arraylist(n))
}

//  - one with your own LinkedList
pub fn josephus_linkedlist(mut arr: LinkedList<usize>, m: usize) -> usize {
    if m == 0 {
        return *arr.iter().last().unwrap();
    }

    let mut count = 0;
    while arr.len() > 1 {
        let mut index = 0;
        while index < arr.len() {
            if count == m {
                count = 0;
                arr.remove(index);
                continue;
            }

            count += 1;
            index += 1;
        }
    }
    arr.get(0).unwrap().val
}

//  - one using Linked list with Iterator
pub fn josephus_linkedlist_iter(mut arr: LinkedList<usize>, m: usize) -> usize {
    if m == 0 {
        return *arr.iter().last().unwrap();
    }

    let mut count = 0;
    while arr.len() > 1 {
        arr = LinkedList::from_list(
            arr.iter()
                .filter_map(|x| {
                    if count == m {
                        count = 0;
                        return None;
                    }
                    count += 1;
                    Some(*x)
                })
                .collect(),
        );
    }

    arr.get(0).unwrap().val
}

#[cfg(test)]
mod test {
    use crate::task4::{generate_linkedlist, josephus_linkedlist, josephus_linkedlist_iter};

    use super::{generate_arraylist, josephus_arraylist, josephus_arraylist_iter};

    #[test]
    fn test_arraylist() {
        let arr = generate_arraylist(5);
        assert_eq!(josephus_arraylist(arr, 0), 5);

        let arr = generate_arraylist(5);
        assert_eq!(josephus_arraylist(arr, 1), 3);

        let arr = generate_arraylist(5);
        assert_eq!(josephus_arraylist(arr, 5), 4);
    }

    #[test]
    fn test_arraylist_iter() {
        let arr = generate_arraylist(5);
        assert_eq!(josephus_arraylist_iter(arr, 0), 5);

        let arr = generate_arraylist(5);
        assert_eq!(josephus_arraylist_iter(arr, 1), 3);

        let arr = generate_arraylist(5);
        assert_eq!(josephus_arraylist_iter(arr, 5), 4);
    }

    #[test]
    fn test_linkedlist() {
        let arr = generate_linkedlist(5);
        assert_eq!(josephus_linkedlist(arr, 0), 5);

        let arr = generate_linkedlist(5);
        assert_eq!(josephus_linkedlist(arr, 1), 3);

        let arr = generate_linkedlist(5);
        assert_eq!(josephus_linkedlist(arr, 5), 4);
    }

    #[test]
    fn test_linkedlist_iter() {
        let arr = generate_linkedlist(5);
        assert_eq!(josephus_linkedlist_iter(arr, 0), 5);

        let arr = generate_linkedlist(5);
        assert_eq!(josephus_linkedlist_iter(arr, 1), 3);

        let arr = generate_linkedlist(5);
        assert_eq!(josephus_linkedlist_iter(arr, 5), 4);
    }
}
