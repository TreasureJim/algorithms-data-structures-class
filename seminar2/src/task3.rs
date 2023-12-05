// Task 3
// Implement an address book using a LinkedList to store the contacts.
// Each contact shall be represented by one node in the linked list and shall store the name and address of the contact. The linked list must be implemented by the student manually, i.e. it is not allowed to use Javas build in implementations.
// The linked list must have at least the following functionality:
// 1) Add node
// 2) Remove node
// 3) Get node using index
// The program must use all of the above functionality and must print the complete list of contacts to the screen using a loop.

struct LinkedList<T> {
    head: LinkedListNode<T>
}

impl<T> LinkedList<T> {
    pub fn new(val: T) -> Self {
        Self { head: LinkedListNode::new(val) }
    }

    pub fn get(&self, index: usize) -> Option<&LinkedListNode<T>> {
        let mut last_node = Some(&self.head);
        let mut last_index = 0;
        while last_index < index{
            if last_node.unwrap().next_node.is_none() {
                return None;
            }
            last_node = last_node.unwrap().next_node.as_deref();
            last_index += 1;
        }

        last_node
    }

    pub fn add_node(&self, index: usize, val: T) -> Result<&LinkedListNode<T>, ()> {
        todo!()
    }

    pub fn remove_node(&self, index: usize) -> Result<LinkedListNode<T>, ()> {
        todo!()
    }
}

struct LinkedListNode<T> {
    val: T,
    next_node: Option<Box<LinkedListNode<T>>>
}

impl<T> LinkedListNode<T> {
    pub fn new(x: T) -> Self {
        Self { val: x, next_node: None }
    }

    pub fn next(&self) -> Option<&LinkedListNode<T>> {
        if let Some(node) = self.next_node {
            Some(&node)
        } else {
            None
        }
    }
}

struct Contact {
    name: String,
    address: String
}
