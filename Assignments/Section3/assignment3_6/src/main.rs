// Assignment 3.6: Reference-Counted DAG (Directed Acyclic Graph)
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    id: i32,
    downstream: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(id: i32) -> Node {
        Node {
            id: id,
            downstream: Vec::new(),
        }
    }
}

fn main() {
    let leaf = Rc::new(RefCell::new(Node::new(3)));
    let mut node1 = Node::new(1);
    node1.downstream.push(Rc::clone(&leaf));

    let mut node2 = Node::new(2);
    node2.downstream.push(Rc::clone(&leaf));

    leaf.borrow_mut().id = 4;

    println!("{:?}", node1);
    println!("{:?}", node2);

    let ref1 = leaf.borrow_mut();
    let ref2 = leaf.borrow_mut();
}
