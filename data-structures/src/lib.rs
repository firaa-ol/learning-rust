//TODO: make it iterable

use std::fmt::Display;

struct SinglyLinkedList<T : Display + Clone> {
    start: Option<Box<Node<T>>>,
    size: u32
}

impl<T : Display + Clone> SinglyLinkedList<T> {
    fn new() -> Self{
        Self {
            start: None,
            size: 0
        }
    }

    // push to the front of the list
    fn push(&mut self, elt: T) {
        if self.start.is_none() {
            self.start = Some(Box::new(Node {value: elt, next: None}));
        } else {
            let new_node = Some(Box::new(Node {value: elt, next: self.start.take() }));
            self.start = new_node;
        }
        self.size += 1;
    }

    // pop from the front of the list
    fn pop(&mut self) -> Option<T> {
        if self.start.is_none() {
            None
        } else {
            let val = self.start.as_ref().unwrap().value.clone();
            self.start = self.start.as_mut().unwrap().next.take();
            self.size -= 1;
            Some(val)
        }
    }

    // add at the end of the list
    fn enqueue(&mut self, elt: T) {
        let last_node = self.get_last_element();
        if last_node.is_none() {
            self.start = Some(Box::new(Node {value: elt, next: None}));
        } else {
            last_node.unwrap().next = Some(Box::new(Node {value: elt, next: None}));
        }
        self.size += 1;
    }

    // remove from end of the list
    fn dequeue(&mut self) -> Option<T> {
        if self.start.is_none() {
            None
        } else {
            self.size -= 1;
            let before_last_node = self.get_before_last_element();          
            Some(before_last_node.unwrap().next.take().unwrap().value)
        }
    }

    fn get_last_element(&mut self) -> Option<&mut Box<Node<T>>> {
        if self.start.is_none() {
            None
        } else {
            let mut node = self.start.as_mut().unwrap();
            while !node.next.is_none() {
                node = node.next.as_mut().unwrap();
            }

            Some(node)
        }
    }

    //get the element before the last element
    fn get_before_last_element(&mut self) -> Option<&mut Box<Node<T>>> {
        if self.start.is_none() {
            None
        } else {
            let mut node = self.start.as_mut().unwrap();
            while !node.next.is_none() {
                if node.next.as_ref().unwrap().next.is_none(){
                    break;
                }

                node = node.next.as_mut().unwrap();
            }

            Some(node)
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

    fn rev(&self) -> SinglyLinkedList<T> {
        let mut new_list = SinglyLinkedList::new();
        if !self.start.is_none() {
            let mut node = self.start.as_ref().unwrap();
            new_list.push(node.value.clone());
            while !node.next.is_none() {
                node = node.next.as_ref().unwrap();
                new_list.push(node.value.clone());
            }
        }

        new_list
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

    #[test]
    fn singly_linked_list_reverse_works() {
        let mut list = SinglyLinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let reversed_list = list.rev();
        assert_eq!(list.to_string().as_str(), "3,2,1,");
        assert_eq!(reversed_list.to_string().as_str(), "1,2,3,");
    }

    #[test]
    fn singly_linked_list_queue_works() {
        let mut list = SinglyLinkedList::new();
        list.enqueue(10);
        list.enqueue(23);
        list.enqueue(345);
        list.enqueue(77);

        assert_eq!(list.size, 4);
        assert_eq!(list.to_string(), "10,23,345,77,");

        list.dequeue();

        assert_eq!(list.size, 3);
        assert_eq!(list.to_string(), "10,23,345,");
    }

}
