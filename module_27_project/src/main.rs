
#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Self {
            value: value,
            next: None
        }
    }
}

#[derive(Debug)]
struct List {
    head: Option<Box<Node>>,
    length: i32,
}

impl List {
    fn new() -> Self {
        Self {
            head: None,
            length: 0
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

    fn get_length(&self) -> i32 {
        self.length
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

}
