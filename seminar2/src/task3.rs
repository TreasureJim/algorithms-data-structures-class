#![allow(unused)]

// Task 3
// Implement an address book using a LinkedList to store the contacts.
// Each contact shall be represented by one node in the linked list and shall store the name and address of the contact. The linked list must be implemented by the student manually, i.e. it is not allowed to use Javas build in implementations.
// The linked list must have at least the following functionality:
// 1) Add node
// 2) Remove node
// 3) Get node using index
// The program must use all of the above functionality and must print the complete list of contacts to the screen using a loop.

struct LinkedList<T> {
    head: LinkedListNode<T>,
}

impl<T> LinkedList<T> {
    pub fn new(val: T) -> Self {
        Self {
            head: LinkedListNode::new(val),
        }
    }

    pub fn iter(&self) -> LinkedListIterator<T> {
        LinkedListIterator { current: Some(&self.head) }
    }

    pub fn get(&self, index: usize) -> Option<&LinkedListNode<T>> {
        let mut last_node = Some(&self.head);
        let mut last_index = 0;
        while last_index < index {
            last_node.unwrap().next_node.as_ref()?;
            last_node = last_node.unwrap().next_node.as_deref();
            last_index += 1;
        }

        last_node
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut LinkedListNode<T>> {
        let mut last_node = Some(&mut self.head);
        let mut last_index = 0;
        while last_index < index {
            if last_node.as_ref().unwrap().next_node.is_none() {
                return None;
            }
            last_node = last_node.unwrap().next_node.as_deref_mut();
            last_index += 1;
        }

        last_node
    }

    pub fn add_node(&mut self, index: usize, val: T) -> Option<&LinkedListNode<T>> {
        let node_before = self.get_mut(index);
        if let Some(node_before) = node_before {
            let next_node = node_before.next_node.take();
            let mut current_node = LinkedListNode::new(val);
            current_node.next_node = next_node;
            node_before.next_node = Some(Box::new(current_node));
        }

        None
    }

    pub fn remove_node(&mut self, index: usize) -> Option<LinkedListNode<T>> {
        let node_before = self.get_mut(index - 1);
        if node_before.is_none() {
            return None;
        }
        let node_before = node_before.unwrap();

        let current_node = node_before.next_node.take();
        if current_node.is_none() {
            return None;
        }
        let mut current_node = current_node.unwrap();

        let next_node = current_node.next_node.take();
        node_before.next_node = next_node;

        Some(*current_node)
    }
}

struct LinkedListIterator<'a, T> {
    current: Option<&'a LinkedListNode<T>>,
}

impl<'a, T> Iterator for LinkedListIterator<'a ,T> {
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

struct LinkedListNode<T> {
    val: T,
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
    fn test_add_node() {
        let mut address_book = LinkedList::new(Contact {
            name: "Homer".to_string(),
            address: "742 Evergreen Terrace".to_string(),
        });

        address_book.add_node(0, Contact {
            name: "Marge".to_string(),
            address: "742 Evergreen Terrace".to_string(),
        });

        assert_eq!(address_book.get(0).unwrap().val.name, "Homer");
        assert_eq!(address_book.get(1).unwrap().val.name, "Marge");
    }

    #[test]
    fn test_remove_node() {
        let mut address_book = LinkedList::new(Contact {
            name: "Homer".to_string(),
            address: "742 Evergreen Terrace".to_string(),
        });

        address_book.add_node(0, Contact {
            name: "Marge".to_string(),
            address: "742 Evergreen Terrace".to_string(),
        });

        address_book.add_node(1, Contact {
            name: "Bart".to_string(),
            address: "742 Evergreen Terrace".to_string(),
        });

        let removed_node = address_book.remove_node(1);
        assert_eq!(removed_node.unwrap().val.name, "Marge");
        assert_eq!(address_book.get(1).unwrap().val.name, "Bart");
    }

    #[test]
    fn test_get_node() {
        let mut address_book = LinkedList::new(Contact {
            name: "Homer".to_string(),
            address: "742 Evergreen Terrace".to_string(),
        });

        address_book.add_node(0, Contact {
            name: "Marge".to_string(),
            address: "742 Evergreen Terrace".to_string(),
        });

        address_book.add_node(1, Contact {
            name: "Bart".to_string(),
            address: "742 Evergreen Terrace".to_string(),
        });

        assert_eq!(address_book.get(0).unwrap().val.name, "Homer");
        assert_eq!(address_book.get(1).unwrap().val.name, "Marge");
        assert_eq!(address_book.get(2).unwrap().val.name, "Bart");
    }

    #[test]
    fn test_iterator() {
        let mut address_book = LinkedList::new(Contact {
            name: "Homer".to_string(),
            address: "742 Evergreen Terrace".to_string(),
        });

        address_book.add_node(0, Contact {
            name: "Marge".to_string(),
            address: "742 Evergreen Terrace".to_string(),
        });

        address_book.add_node(1, Contact {
            name: "Bart".to_string(),
            address: "742 Evergreen Terrace".to_string(),
        });

        let mut iterator = address_book.iter();
        assert_eq!(iterator.next().unwrap().name, "Homer");
        assert_eq!(iterator.next().unwrap().name, "Marge");
        assert_eq!(iterator.next().unwrap().name, "Bart");
        assert!(iterator.next().is_none());
    }

    #[test]
    fn test_print_for_loop() {
        let mut address_book = LinkedList::new(Contact {
            name: "Homer".to_string(),
            address: "742 Evergreen Terrace".to_string(),
        });

        address_book.add_node(0, Contact {
            name: "Marge".to_string(),
            address: "742 Evergreen Terrace".to_string(),
        });

        address_book.add_node(1, Contact {
            name: "Bart".to_string(),
            address: "742 Evergreen Terrace".to_string(),
        });

        for contact in address_book.iter() {
            println!("{contact:?}");
        }
    }
}
