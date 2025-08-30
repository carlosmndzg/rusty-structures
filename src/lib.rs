#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None, len: 0 }
    }

    pub fn push(&mut self, value: T) {
        self.len += 1;

        if let None = self.head {
            self.head = Some(Box::new(Node { value, next: None }));

            return;
        }

        let mut current = &mut self.head;

        while let Some(_) = &current.as_mut().unwrap().next {
            current = &mut current.as_mut().unwrap().next;
        }

        current.as_mut().unwrap().next = Some(Box::new(Node { value, next: None }))
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len() {
            return None;
        }

        let mut head = &self.head;
        let mut current_index = 0;

        while current_index < index {
            head = &head.as_ref().unwrap().next;
            current_index += 1;
        }


        Some(&head.as_ref().unwrap().value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_list() {
        let linked_list: LinkedList<i32> = LinkedList::new();

        assert_eq!(0, linked_list.len());
    }

    #[test]
    fn one_item() {
        let mut linked_list = LinkedList::new();

        linked_list.push(5);

        assert_eq!(Some(&5), linked_list.get(0));
        assert_eq!(1, linked_list.len());
    }

    #[test]
    fn many_items() {
        let mut linked_list = LinkedList::new();

        linked_list.push(5);
        linked_list.push(25);
        linked_list.push(75);

        assert_eq!(Some(&5), linked_list.get(0));
        assert_eq!(Some(&25), linked_list.get(1));
        assert_eq!(Some(&75), linked_list.get(2));
        assert_eq!(3, linked_list.len());
    }
}
