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

    pub fn remove(&mut self, index: usize) -> T {
        if index >= self.len {
            panic!("index out of bounds");
        }

        self.len -= 1;

        if index == 0 {
            let Node { value, next } = *self.head.take().unwrap();

            self.head = next;

            return value;
        }

        let mut head = &mut self.head;

        for _ in 0..(index - 1) {
            head = &mut head.as_mut().unwrap().next;
        }

        let Node { value, next } = *head.as_mut().unwrap().next.take().unwrap();

        head.as_mut().unwrap().next = next;

        value
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len() {
            return None;
        }

        let mut head = &self.head;

        for _ in 0..index {
            head = &head.as_ref().unwrap().next;
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

    #[test]
    fn get_out_of_bounds() {
        let mut linked_list = LinkedList::new();

        linked_list.push(5);
        linked_list.push(25);
        linked_list.push(75);

        assert_eq!(None, linked_list.get(3));
    }

    #[test]
    fn remove_first() {
        let mut linked_list = LinkedList::new();

        linked_list.push(5);
        linked_list.push(25);
        linked_list.push(75);

        assert_eq!(5, linked_list.remove(0));
        assert_eq!(Some(&25), linked_list.get(0));
        assert_eq!(Some(&75), linked_list.get(1));
        assert_eq!(2, linked_list.len());
    }

    #[test]
    fn remove_last() {
        let mut linked_list = LinkedList::new();

        linked_list.push(5);
        linked_list.push(25);
        linked_list.push(75);

        assert_eq!(75, linked_list.remove(2));
        assert_eq!(Some(&5), linked_list.get(0));
        assert_eq!(Some(&25), linked_list.get(1));
        assert_eq!(2, linked_list.len());
    }

    #[test]
    fn remove_middle() {
        let mut linked_list = LinkedList::new();

        linked_list.push(5);
        linked_list.push(25);
        linked_list.push(75);

        assert_eq!(25, linked_list.remove(1));
        assert_eq!(Some(&5), linked_list.get(0));
        assert_eq!(Some(&75), linked_list.get(1));
        assert_eq!(2, linked_list.len());
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn remove_out_of_bounds() {
        let mut linked_list = LinkedList::new();

        linked_list.push(5);
        linked_list.push(25);
        linked_list.push(75);

        linked_list.remove(3);
    }
}