struct SinglyLinkedList {
    start: Option<Box<Node>>
}

impl SinglyLinkedList {
    fn new() -> Self{
        Self {
            start: None
        }
    }

    fn push(&mut self, elt: u32) {
        if self.start.is_none() {
            self.start = Some(Box::new(Node {value: elt, next: None}));
        } else {
            let new_node = Some(Box::new(Node {value: elt, next: self.start.take() }));
            self.start = new_node;
        }
    }

    fn pop(&mut self) -> Option<u32> {
        if self.start.is_none() {
            None
        } else {
            let val = self.start.as_ref().unwrap().value;
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

struct Node{
    value: u32,
    next: Option<Box<Node>>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut list = SinglyLinkedList::new();
        list.push(25);
        list.push(46);
        list.push(6567);
        assert_eq!(list.pop(), Some(6567));
        list.push(544);
        assert!(!list.start.is_none());
        assert_eq!(list.to_string().as_str(), "544,46,25,");
    }
}
