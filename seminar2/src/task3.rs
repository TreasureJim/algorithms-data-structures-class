#![allow(unused)]

// Task 3
// Implement an address book using a LinkedList to store the contacts.
// Each contact shall be represented by one node in the linked list and shall store the name and address of the contact. The linked list must be implemented by the student manually, i.e. it is not allowed to use Javas build in implementations.
// The linked list must have at least the following functionality:
// 1) Add node
// 2) Remove node
// 3) Get node using index
// The program must use all of the above functionality and must print the complete list of contacts to the screen using a loop.

use std::ops::DerefMut;

pub struct LinkedList<T> {
    head: Option<Box<LinkedListNode<T>>>,
    length: usize,
}

impl<T> LinkedList<T> {
    pub fn new(val: T) -> Self {
        Self {
            head: Some(Box::new(LinkedListNode::new(val))),
            length: 1,
        }
    }

    pub fn from_list(list: Vec<T>) -> Self {
        let mut linkedlist = Self {
            head: None,
            length: list.len(),
        };

        let mut last_node = &mut linkedlist.head;
        for e in list.into_iter() {
            *last_node = Some(Box::new(LinkedListNode {
                val: e,
                next_node: None,
            }));
            last_node = &mut last_node.as_mut().unwrap().next_node;
        }
        linkedlist
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn iter(&mut self) -> LinkedListIterator<T> {
        LinkedListIterator {
            current: self.head.as_deref(),
        }
    }

    pub fn get(&self, index: usize) -> Option<&LinkedListNode<T>> {
        let mut last_node = self.head.as_deref().unwrap();

        for _ in 1..=index {
            last_node = last_node.next_node.as_deref()?;
        }

        Some(last_node)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut LinkedListNode<T>> {
        let mut last_node = self.head.as_deref_mut().unwrap();

        for _ in 1..=index {
            last_node = last_node.next_node.as_deref_mut()?;
        }

        Some(last_node)
    }

    pub fn insert<'a>(&mut self, index: usize, val: T) -> Option<&LinkedListNode<T>> {
        if index > self.length {
            return None;
        }
        self.length += 1;

        let mut new_node = LinkedListNode::new(val);

        if index == 0 {
            new_node.next_node = Some(Box::new(*self.head.take().unwrap()));
            self.head = Some(Box::new(new_node));
            return self.head.as_deref();
        }

        let mut node_before = self.get_mut(index - 1)?;

        let next_node = node_before.next_node.take();
        new_node.next_node = next_node;
        node_before.next_node = Some(Box::new(new_node));

        node_before.next_node.as_deref()
    }

    pub fn remove(&mut self, index: usize) -> Option<LinkedListNode<T>> {
        if index >= self.length {
            return None;
        }
        self.length -= 1;

        if index == 0 {
            let mut removed = self.head.take();
            self.head = removed.as_mut().unwrap().next_node.take();
            return Some(*removed.unwrap());
        }

        let node_before = self.get_mut(index - 1)?;

        let node_before = node_before;

        let current_node = node_before.next_node.take();
        current_node.as_ref()?;
        let mut current_node = current_node.unwrap();

        let next_node = current_node.next_node.take();
        node_before.next_node = next_node;

        Some(*current_node)
    }
}

pub struct LinkedListIterator<'a, T> {
    current: Option<&'a LinkedListNode<T>>,
}

impl<'a, T> Iterator for LinkedListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(node) => {
                self.current = node.next_node.as_deref();
                Some(&node.val)
            }
            None => None,
        }
    }
}

#[derive(Debug)]
pub struct LinkedListNode<T> {
    pub val: T,
    next_node: Option<Box<LinkedListNode<T>>>,
}

impl<T> LinkedListNode<T> {
    pub fn new(x: T) -> Self {
        Self {
            val: x,
            next_node: None,
        }
    }
}

#[derive(Debug)]
struct Contact {
    name: String,
    address: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_list() {
        let vec = vec![1, 2, 3];
        let linkedlist = LinkedList::from_list(vec);
        dbg!(&linkedlist.head);
        assert_eq!(linkedlist.get(0).unwrap().val, 1);
        assert_eq!(linkedlist.get(1).unwrap().val, 2);
        assert_eq!(linkedlist.get(2).unwrap().val, 3);
    }

    #[test]
    fn test_add_node() {
        let mut address_book = LinkedList::new(Contact {
            name: "Homer".to_string(),
            address: "742 Evergreen Terrace".to_string(),
        });

        assert_eq!(address_book.head.as_ref().unwrap().val.name, "Homer");

        let new_node = address_book.insert(
            1,
            Contact {
                name: "Marge".to_string(),
                address: "742 Evergreen Terrace".to_string(),
            },
        );
        assert_eq!(new_node.unwrap().val.name, "Marge");

        assert_eq!(address_book.get(0).unwrap().val.name, "Homer");
        assert_eq!(address_book.get(1).unwrap().val.name, "Marge");
    }

    #[test]
    fn test_remove_node() {
        let node = LinkedListNode {
            val: "Homer".to_string(),
            next_node: Some(Box::new(LinkedListNode {
                val: "Marge".to_string(),
                next_node: Some(Box::new(LinkedListNode {
                    val: "Bart".to_string(),
                    next_node: None,
                })),
            })),
        };
        let mut address_book = LinkedList {
            head: Some(Box::new(node)),
            length: 3,
        };

        let removed_node = address_book.remove(1);
        assert_eq!(removed_node.unwrap().val, "Marge");
        assert_eq!(address_book.get(0).unwrap().val, "Homer");
        assert_eq!(address_book.get(1).unwrap().val, "Bart");
    }

    #[test]
    fn test_get() {
        let node = LinkedListNode {
            val: "Homer".to_string(),
            next_node: Some(Box::new(LinkedListNode {
                val: "Marge".to_string(),
                next_node: Some(Box::new(LinkedListNode {
                    val: "Bart".to_string(),
                    next_node: None,
                })),
            })),
        };
        let address_book = LinkedList {
            head: Some(Box::new(node)),
            length: 3,
        };

        assert_eq!(address_book.get(0).unwrap().val, "Homer");
        assert_eq!(address_book.get(1).unwrap().val, "Marge");
        assert_eq!(address_book.get(2).unwrap().val, "Bart");
    }

    #[test]
    fn test_get_mut() {
        let node = LinkedListNode {
            val: "Homer".to_string(),
            next_node: Some(Box::new(LinkedListNode {
                val: "Marge".to_string(),
                next_node: Some(Box::new(LinkedListNode {
                    val: "Bart".to_string(),
                    next_node: None,
                })),
            })),
        };
        let mut address_book = LinkedList {
            head: Some(Box::new(node)),
            length: 3,
        };

        assert_eq!(address_book.get_mut(0).unwrap().val, "Homer");
        assert_eq!(address_book.get_mut(1).unwrap().val, "Marge");
        assert_eq!(address_book.get_mut(2).unwrap().val, "Bart");
    }

    #[test]
    fn test_iterator() {
        let node = LinkedListNode {
            val: "Homer".to_string(),
            next_node: Some(Box::new(LinkedListNode {
                val: "Marge".to_string(),
                next_node: Some(Box::new(LinkedListNode {
                    val: "Bart".to_string(),
                    next_node: None,
                })),
            })),
        };
        let mut address_book = LinkedList {
            head: Some(Box::new(node)),
            length: 3,
        };

        let mut iterator = address_book.iter();
        assert_eq!(iterator.next().unwrap(), "Homer");
        assert_eq!(iterator.next().unwrap(), "Marge");
        assert_eq!(iterator.next().unwrap(), "Bart");
        assert!(iterator.next().is_none());
    }

    #[test]
    fn test_print_for_loop() {
        let mut address_book = LinkedList::new(Contact {
            name: "Homer".to_string(),
            address: "742 Evergreen Terrace".to_string(),
        });

        address_book.insert(
            1,
            Contact {
                name: "Marge".to_string(),
                address: "742 Evergreen Terrace".to_string(),
            },
        );

        address_book.insert(
            2,
            Contact {
                name: "Bart".to_string(),
                address: "742 Evergreen Terrace".to_string(),
            },
        );

        address_book.remove(1);

        for contact in address_book.iter() {
            println!("{contact:?}");
        }
    }
}
