use std::cell::RefCell;
use std::fmt::Display;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Default, PartialEq)]
struct Node<T: Display + Default + PartialEq> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}

struct LL<T: Display + Default + PartialEq> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Display + Default + PartialEq> LL<T> {
    fn new() -> LL<T> {
        LL {
            head: None,
            tail: None,
        }
    }

    fn push_back(&mut self, value: T) {
        let node = Rc::new(RefCell::new(Node {
            value,
            prev: self.tail.clone(),
            next: None,
        }));
        match &mut self.tail {
            Some(x) => {
                x.borrow_mut().next = Some(node.clone());
                self.tail.replace(node.clone());
            }
            None => {
                self.tail.replace(node.clone());
                self.head.replace(node.clone());
            }
        };
    }

    fn push_front(&mut self, value: T) {
        let node = Rc::new(RefCell::new(Node {
            value,
            prev: None,
            next: self.head.clone(),
        }));
        match &mut self.head {
            Some(x) => {
                x.borrow_mut().prev = Some(node.clone());
                self.head.replace(node.clone());
            }
            None => {
                self.head = Some(node.clone());
                self.tail = self.head.clone();
            }
        }
    }

    fn pop_back(&mut self) -> Option<T> {
        match self.tail.clone() {
            None => None,
            Some(n) => {
                self.tail = n.borrow().prev.clone();
                self.tail.as_ref().unwrap().borrow_mut().next = None;
                Some(n.as_ref().take().value)
            }
        }
    }

    fn print(&self) {
        fn _print<T: Display + Default + PartialEq>(node: &Option<Rc<RefCell<Node<T>>>>) {
            if let Some(n) = node {
                println!("{}", n.as_ref().borrow().value);
                _print(&n.as_ref().borrow().next);
            }
        }
        _print(&self.head);
    }

    fn replace(&mut self, index: usize, item: T) -> Option<Rc<RefCell<Node<T>>>> {
        fn _replace<T: Display + Default + PartialEq>(node: &Option<Rc<RefCell<Node<T>>>>, index: usize, item: T) -> Option<Rc<RefCell<Node<T>>>> {
            match node {
                None => {
                    None
                }
                Some(n) if index == 0 => {
                    n.borrow_mut().value = item;
                    node.clone()
                }
                Some(n) => {
                    _replace(&n.borrow().next, index - 1, item)
                }
            }
        }
        _replace(&self.head, index, item)
    }

    fn insert(&mut self, index: usize, item: T) -> Option<Rc<RefCell<Node<T>>>> {
        let mut current_node = self.head.clone();
        for _ in [0..index] {
            match current_node {
                None => {
                    return None;
                }
                Some(x) => {
                    current_node = x.as_ref().borrow_mut().next.clone();
                }
            }
        }
        let current_node_unwrapped = current_node.unwrap();
        let node_ref = Rc::new(RefCell::new(Node {
            value: item,
            prev: Some(current_node_unwrapped.clone()),
            next: current_node_unwrapped.borrow().next.clone()
        }));
        if let Some(n) = current_node_unwrapped.borrow().next.clone() {
            n.borrow_mut().prev = Some(node_ref.clone());
        }
        current_node_unwrapped.borrow_mut().next = Some(node_ref.clone());
        Some(node_ref)
    }
}


fn main() {
    let mut n = LL::new();
    n.push_back(2);
    n.push_back(3);
    n.push_front(1);
    n.push_back(4);
    n.print();
    println!("popping values:");
    println!("{}", n.pop_back().unwrap());
    println!("{}", n.pop_back().unwrap());

    n.replace(1, 9999);
    n.insert(1, 2222);
    println!("whats next: ");
    n.print();
}
