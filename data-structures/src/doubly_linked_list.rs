use std::cell::RefCell;
use std::fmt::Display;
use std::rc::{Rc, Weak};

struct Node<T: Display + Clone> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    previous: Option<Weak<RefCell<Node<T>>>>,
}

struct DoublyLinkedList<T: Display + Clone> {
    head: Option<Rc<RefCell<Node<T>>>>,
    size: u32,
}

impl<T: Display + Clone> DoublyLinkedList<T> {
    fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    // push to the front of the list
    fn push(&mut self, elt: T) {
        let new_node = Rc::new(RefCell::new(Node {
            value: elt,
            next: None,
            previous: None,
        }));

        match self.head.take() {
            None => self.head = Some(new_node),
            Some(old_head) => {
                old_head.borrow_mut().previous = Some(Rc::downgrade(&new_node));
                new_node.borrow_mut().next = Some(old_head);
                self.head = Some(new_node);
            }
        }

        self.size += 1;
    }

    // pop from the front of the list
    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head_node| {
            self.head = head_node.borrow_mut().next.take();
            self.size -= 1;
            Rc::try_unwrap(head_node).ok().unwrap().into_inner().value
        })
    }
}

impl<T: Display + Clone> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubly_linked_list_push_pop() {
        let mut dll = DoublyLinkedList::new();

        dll.push(7);
        dll.push(8);
        dll.push(9);

        assert_eq!(dll.pop(), Some(9));
        assert_eq!(dll.pop(), Some(8));
        assert_eq!(dll.pop(), Some(7));
        assert_eq!(dll.pop(), None);
    }
}
