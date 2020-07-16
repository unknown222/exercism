use std::fmt;
use std::iter::FromIterator;

pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node")
    }
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None
        }
    }

    pub fn len(&self) -> usize {
        let mut len= 0;
        let mut node = &self.head;
        while node.is_some() {
            node = node.as_ref().map(|boxed| &boxed.next).unwrap();
            len += 1;
        }
        len
    }

    pub fn push(&mut self, _element: T) {
        let node = Some(Box::new(Node {
            data: _element,
            next: self.head.take(),
        }));

        self.head = node;
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut head = self.head.take();
        self.head = match head {
            Some(ref mut node) => node.next.take(),
            None => None,
        };
        head.map(|boxed| boxed.data)
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|boxed| &boxed.data)
    }


}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut list = SimpleLinkedList::new();
        let mut node = &self.head;
        while let &Some(ref boxed) = node {
            list.push(boxed.data.clone());
            node = &boxed.next;
        }
        list
    }
}


impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(item: &[T]) -> Self {
        item.into_iter().fold(SimpleLinkedList::new(), |mut list,
                                                        element| {
            list.push(element.clone());
            list
        })
    }
}


impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut vec = Vec::new();
        while let Some(data) = self.pop() {
            vec.insert(0, data);
        }
        vec
    }
}
