//TODO: add reverse method

use std::fmt::Display;

struct SinglyLinkedList<T : Display + Clone> {
    start: Option<Box<Node<T>>>
}

impl<T : Display + Clone> SinglyLinkedList<T> {
    fn new() -> Self{
        Self {
            start: None
        }
    }

    fn push(&mut self, elt: T) {
        if self.start.is_none() {
            self.start = Some(Box::new(Node {value: elt, next: None}));
        } else {
            let new_node = Some(Box::new(Node {value: elt, next: self.start.take() }));
            self.start = new_node;
        }
    }

    fn pop(&mut self) -> Option<T> {
        if self.start.is_none() {
            None
        } else {
            let val = self.start.as_ref().unwrap().value.clone();
            self.start = self.start.as_mut().unwrap().next.take();
            Some(val)
        }
    }

    fn to_string(&self) -> String {
        let mut elements = "".to_string();
        if !self.start.is_none() {
            let mut node = self.start.as_ref().unwrap();
            elements = elements + node.value.to_string().as_str() + ",";
            while !node.next.is_none() {
                node = node.next.as_ref().unwrap();
                elements = elements + node.value.to_string().as_str() + ",";
            }
        }

        elements
    }
}

struct Node<T : Display + Clone>{
    value: T,
    next: Option<Box<Node<T>>>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn singly_linked_list_integer_works() {
        let mut list = SinglyLinkedList::new();
        list.push(25);
        list.push(46);
        list.push(6567);
        assert_eq!(list.pop(), Some(6567));
        list.push(544);
        assert!(!list.start.is_none());
        assert_eq!(list.to_string().as_str(), "544,46,25,");
    }

    #[test]
    fn singly_linked_list_str_works() {
        let mut list2 = SinglyLinkedList::new();
        list2.push("hello");
        list2.push("goodbye");
        list2.push("hi");

        assert_eq!(list2.to_string().as_str(), "hi,goodbye,hello,");
    }
}
