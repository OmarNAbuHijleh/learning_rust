#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Self {
            value: value,
            next: None,
        }
    }
}

#[derive(Debug)]
struct List {
    head: Option<Box<Node>>,
    length: usize,
}

impl List {
    fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }

    fn append_node(&mut self, new_node: Node) {
        // New node is the node we're inserting into the linked list
        let mut current_node = &mut self.head;

        while let Some(node) = current_node {
            current_node = &mut (**node).next;
        }

        *current_node = Some(Box::new(new_node));
        self.length += 1;
    }

    fn get_length(&self) -> usize {
        self.length
    }

    fn sum(&self) -> i32 {
        // Iterates through the linked list and calculates the total value of the integers contained therein
        let mut current_node = &self.head;
        let mut total_value = 0;

        while let Some(node) = current_node {
            total_value += node.value;
            // increment to the next node
            current_node = &(node.next);
        }

        return total_value;
    }
}

// Second implementation using Rc
// use std::ops::DerefMut;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node2 {
    value: i32,
    next: Option<Rc<RefCell<Node2>>>,
}

impl Node2 {
    fn new(value: i32) -> Self {
        Self {
            value: value,
            next: None,
        }
    }
}

#[derive(Debug)]
struct List2 {
    head: Option<Rc<RefCell<Node2>>>,
}

impl List2 {
    fn new() -> Self {
        Self { head: None }
    }

    fn append_node(&mut self, node_input: Rc<RefCell<Node2>>) {
        if self.head.is_none() {
            self.head = Some(node_input);
            return;
        }

        let mut current_pointer = self.head.clone();
        let new_node = Some(node_input);
        while let Some(node) = current_pointer {
            if node.borrow().next.is_none() {
                node.borrow_mut().next = new_node;
                return;
            }
            current_pointer = node.borrow().next.clone();
        }
    }
}

fn main() {
    println!("Exercise 1: Construct a List using Box");
    // should be 1 -> 2 -> 3 -> Nil, where you construct the list yourself

    //First instantiate our list:
    let mut our_list = List::new();

    //Now create the nodes and add them to our list
    our_list.append_node(Node::new(1));
    our_list.append_node(Node::new(2));
    our_list.append_node(Node::new(3));
    println!("{our_list:#?}");
    println!("{}", our_list.get_length());

    println!("{}", our_list.sum());

    // RC example
    let node1 = Rc::new(Node::new(1));
    let node2 = Rc::new(Node::new(2));
    let node3 = Rc::new(Node::new(3));
    let mut list_one = vec![node1, node2];
    let mut list_two = vec![node3];

    // A node that can be owned by multiple vectors
    let node4 = Rc::new(Node::new(4));
    list_one.push(Rc::clone(&node4));
    list_two.push(Rc::clone(&node4));

    println!("List one: {:?}", list_one);
    println!("List two: {:?}", list_two);

    println!("Node 4 reference count: {}", Rc::strong_count(&node4));
    println!("{}", Rc::ptr_eq(&list_one[2], &list_two[1]));

    // Trying to create a linked list where the elements can have multiple owners
    let mut list2 = List2::new();
    list2.append_node(Rc::new(RefCell::new(Node2::new(1))));
    list2.append_node(Rc::new(RefCell::new(Node2::new(2))));
    list2.append_node(Rc::new(RefCell::new(Node2::new(3))));

    let mut list3 = List2::new();
    list3.append_node(Rc::new(RefCell::new(Node2::new(10))));
    list3.append_node(Rc::new(RefCell::new(Node2::new(11))));
    list3.append_node(Rc::new(RefCell::new(Node2::new(12))));

    let latest_node = Rc::new(RefCell::new(Node2::new(20)));
    list2.append_node(Rc::clone(&latest_node));
    list3.append_node(Rc::clone(&latest_node));

    println!("{list2:#?}");
    println!("{list3:#?}");
}
