//TODO: make it iterable

use std::fmt::Display;

struct SinglyLinkedList<T: Display + Clone> {
    start: Option<Box<Node<T>>>,
    size: u32,
}

impl<T: Display + Clone> SinglyLinkedList<T> {
    fn new() -> Self {
        Self {
            start: None,
            size: 0,
        }
    }

    // push to the front of the list
    fn push(&mut self, elt: T) {
        self.start = Some(Box::new(Node {
            value: elt,
            next: self.start.take(),
        }));
        self.size += 1;
    }

    // pop from the front of the list
    fn pop(&mut self) -> Option<T> {
        if self.size > 0 {
            self.size -= 1;
        }

        self.start.take().map(|node| {
            self.start = node.next;
            node.value
        })
    }

    // add at the end of the list
    fn enqueue(&mut self, elt: T) {
        let last_node = self.get_last_element();
        if last_node.is_none() {
            self.start = Some(Box::new(Node {
                value: elt,
                next: None,
            }));
        } else {
            last_node.unwrap().next = Some(Box::new(Node {
                value: elt,
                next: None,
            }));
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
                if node.next.as_ref().unwrap().next.is_none() {
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

    // remove an element by index
    fn remove(&mut self, index: u32) {
        if index == 0 {
            self.pop();
        } else if index == self.size - 1 {
            self.dequeue();
        } else if index > self.size - 1 {
            panic!("Index out of Range");
        } else {
            let mut counter = 0;
            let mut node = self.start.as_mut().unwrap();
            while !node.next.is_none() {
                if counter + 1 == index {
                    node.next = node.next.as_mut().unwrap().next.take();
                    break;
                }
                node = node.next.as_mut().unwrap();
                counter += 1;
            }

            self.size -= 1;
        }
    }

    // insert at an index
    fn insert(&mut self, elt: T, index: u32) {
        if index == 0 {
            self.push(elt);
        } else if index > self.size - 1 {
            self.enqueue(elt);
        } else {
            let mut counter = 0;
            let mut node = self.start.as_mut().unwrap();
            while !node.next.is_none() {
                if counter + 1 == index {
                    node.next = Some(Box::new(Node {
                        value: elt,
                        next: node.next.take(),
                    }));
                    break;
                }

                node = node.next.as_mut().unwrap();
                counter += 1;
            }

            self.size += 1;
        }
    }

    fn iter(&self) -> NodeIter<'_, T> {
        NodeIter {
            next: self.start.as_deref(),
        }
    }

    fn iter_mut(&mut self) -> NodeIterMut<'_, T> {
        NodeIterMut {
            next: self.start.as_deref_mut(),
        }
    }
}

impl<T: Display + Clone> Drop for SinglyLinkedList<T> {
    fn drop(&mut self) {
        let mut node = self.start.take();

        while let Some(mut inside_node) = node {
            node = inside_node.next.take();
        }
    }
}

struct Node<T: Display + Clone> {
    value: T,
    next: Option<Box<Node<T>>>,
}

struct NodeIter<'a, T: Display + Clone> {
    next: Option<&'a Node<T>>,
}

impl<'a, T: Display + Clone> Iterator for NodeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.value
        })
    }
}

struct NodeIterMut<'a, T: Display + Clone> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T: Display + Clone> Iterator for NodeIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.value
        })
    }
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

    #[test]
    fn singly_linked_list_remove_works() {
        let mut list = SinglyLinkedList::new();
        list.enqueue(1);
        list.enqueue(1);
        list.enqueue(2);
        list.enqueue(3);
        list.enqueue(5);
        list.enqueue(8);
        list.enqueue(13);

        assert_eq!(list.to_string(), "1,1,2,3,5,8,13,");
        list.remove(4);
        assert_eq!(list.to_string(), "1,1,2,3,8,13,");
        list.remove(2);
        assert_eq!(list.to_string(), "1,1,3,8,13,");
        list.remove(0);
        assert_eq!(list.to_string(), "1,3,8,13,");
        list.remove(3);
        assert_eq!(list.to_string(), "1,3,8,");
        list.remove(1);
        assert_eq!(list.to_string(), "1,8,");
    }

    #[test]
    fn singly_linked_list_insert_works() {
        let mut list = SinglyLinkedList::new();
        list.enqueue(20);
        list.enqueue(22);
        list.enqueue(24);

        assert_eq!(list.to_string(), "20,22,24,");
        list.insert(21, 1);
        assert_eq!(list.to_string(), "20,21,22,24,");
        list.insert(23, 3);
        assert_eq!(list.to_string(), "20,21,22,23,24,");
        list.insert(19, 0);
        assert_eq!(list.to_string(), "19,20,21,22,23,24,");
        list.insert(25, 6);
        assert_eq!(list.to_string(), "19,20,21,22,23,24,25,");
    }

    #[test]
    fn iterator_works() {
        let mut list = SinglyLinkedList::new();
        list.push(20);
        list.push(30);
        list.push(40);

        assert_eq!(list.iter().sum::<i32>(), 90);
    }

    #[test]
    fn mutable_iterator_works() {
        let mut list = SinglyLinkedList::new();
        list.push(20);
        list.push(30);
        list.push(40);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 40));
        assert_eq!(iter.next(), Some(&mut 30));
        assert_eq!(iter.next(), Some(&mut 20));
        assert_eq!(iter.next(), None);
    }
}
