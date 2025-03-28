use std::cell::RefCell;
use std::rc::Rc;
use std::marker::PhantomData;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Link<T>,
    prev: Link<T>,
}

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: None }
    }

    pub fn push_front(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node {
            data,
            next: None,
            prev: None,
        }));

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(old_head);
                self.head = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
    }

    pub fn push_back(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node {
            data,
            next: None,
            prev: None,
        }));

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head)
                .ok()
                .unwrap()
                .into_inner()
                .data
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next = None;
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail)
                .ok()
                .unwrap()
                .into_inner()
                .data
        })
    }

    pub fn iter(&self) -> Box<dyn Iterator<Item = T> + '_> where T: Clone {
        Box::new(ListIterator {
            current: self.head.as_ref().map(|head| Rc::clone(head)),
            phantom: PhantomData,
        })
    }
} 

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}
pub struct ListIterator<'a, T> {
    current: Option<Rc<RefCell<Node<T>>>>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: Clone> Iterator for ListIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node_rc| {
            let node = node_rc.borrow();
            self.current = node.next.as_ref().map(|next| Rc::clone(next));
            node.data.clone()
        })
    }
}

fn main() {
    let mut list = List::new();
    list.push_front(1);
    list.push_back(2);
    list.push_front(3);

    for element in list.iter() {
        println!("{}", element);
    }

    list.pop_front();
    list.pop_back();

    println!("After popping:");
    for element in list.iter() {
        println!("{}", element);
    }
}
