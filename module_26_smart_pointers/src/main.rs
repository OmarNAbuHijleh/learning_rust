/*
 * Raw Pointers
 * Rust has variables - a name or piece of data - that are assigned a rust type
 * A pointer is a variable that contains an address in memory. By storing an address, a pointer variable
 * indirectly points to a piece of data. A pointer prevents duplication of the piece of data and instead just points to the address
 *
 * Example: Writing down a home address on a piece of paper. The paper is the pointer and the address is the data it points to.
 *
 * Throughout the course, we've worked with a lot of references.
 * A reference is a type of pointer that points to valid data. We create references with the borrow operator "&"
 * Rust's borrow checker guarantees that references point to valid/allocated data. Pointers in other language do not make the same promise.
 *
 *
 * A raw pointer is a variable that stores a memory address without any safety checks. It may point to valid data but it may also point to deallocated memory.
 * Raw pointers are common in languages like C and C++ that do not have a garbage collector. Developers still call them references even though they lack the guarantees
 * of Rust's references.
 *
 * Rust supports raw pointers but references are safer and recommended.
 *
 * To use rust's raw pointers, you have to manually opt out of rust's safety guarantees!
 *
 * In C/C++, a pointer is a variable that stores a memory address of a piece of data while a reference is an alias (another name) for an existing variable.
 *
 * Smart pointers abstract away the complexity of raw pointers and provide a safer alternative.
 */

/*
 * Raw Pointers and Unsafe Code
 */

/*
 * Smart Pointers
 * We explored raw pointers and compared them to regular references. We can't build everything using regular references, so the language gives us smart pointers.
 *
 * A smart pointer is a type that behaves like a pointer
 * A smart pointer can store additional information and perform more actions compared to a plain pointer/reference
 * Most smart pointers are build with structs. Structs grant the capacity to store more data.
 *
 *
 * A pointer/reference in Rust is like an address to a house. A smart pointer is like an address to a house with additional information -- such as property tax records or nearby restaurants
 *
 * When we create a reference with the borrow operator &, we borrow the data. References are not responsible for de-allocating the data, the original owner is responsible for deallocation. Smart pointers often own and manage their
 * own data, TYPICALLY ON THE HEAP. The advantage is that smart pointers behave like pointers but can be treated like owned types. Smart pointers behave like regular references but we can treat them as any owned type.
 *
 * A heap string is an example of a smart pointer. A String stores a pointer to the heap memory where the text data is located. A string also stores extra metadata like the length and the capacity of the text.
 * The string smart pointer handles the complexity of the pointer/reference behind the scenes. We treat the String like a regular owned type. We never have to work with the String's regular, internal raw pointer.
 */

/*
 * The Box Smart Pointer
 * This smart pointer stores a piece of data on the heap. It's an owned type that is a container around the raw pointer that holds the memory address of the allocated heap data.
 */

/*
 * Intro to Linked Lists
 * A recursive data structure is one that stores the structure itself. The compiler doesn't have to worry about nested data structures occupying infinite memory.
 *
 * A linked list is an example. It's not all data stored together in one place, so each element contains it's data and a pointer to the next element
 */

/*
 * Defining a Linked List, Creating a Linked List
 */

/*
 * Box vs. Regular References
 * Instead of a smart pointer like a box, we could've used a
 */

/*
 * Vectors are Smart Pointers
 */

/*
 * Intro to Binary Search Trees and Creating a Binary Search Tree
 * Searches the search area into two parts continuously. Each step in a binary search halves the search area. A binary search tree is a tree data structure that consists of nodes with values.
 * Each node has between 0 and 2 children
 * A node's value is greater than the value of all children in its left subtree. A node's value is smaller than the values of all children in its right subtree
 */

/*
 * The Deref and DerefMut Traits
 * The smart pointer implements the deref trait to enable a type to behave like a reference. We're going to write our own implementation of the Box smart pointer
 *
 */

/*
 * The Drop Trait
 */

use std::cmp::Ordering; // compare and ordering
use std::ops::{Deref, DerefMut};

struct CustomBox2<T, U> {
    data: T,
    more_data: U,
}

impl<T, U> CustomBox2<T, U> {
    // In this case, how is a smart pointer supposed to know which one we're dereferencing to? Type T or Type U?
    fn new(data: T, more_data: U) -> Self {
        Self { data, more_data }
    }
}

impl<T, U> Deref for CustomBox2<T, U> {
    // We create this "Target" return type because we may have more than one generic type we're able to dereference to!
    // We need to let rust know which type we're dereferencing to! We can set what is being dereferenced within the struct
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T, U> DerefMut for CustomBox2<T, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

struct CustomBox<T> {
    data: T, // The data we're storing
}

impl<T> CustomBox<T> {
    fn new(data: T) -> Self {
        Self { data: data }
    }
}

impl<T> Deref for CustomBox<T> {
    type Target = T; // What we're returning

    // Can also have &T instead if we want as the return type in the definition
    fn deref(&self) -> &Self::Target {
        // Target is the data type we're returning
        &self.data
    }
}

impl<T> DerefMut for CustomBox<T> {
    // NOTE: DerefMut is a sub trait of Deref. If we want to implement DerefMut we have to implement Deref!
    // Can also set &mut T
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data // Returning a mutable reference to the data
    }
}

#[derive(Debug)]
enum BinarySearchTree {
    Empty, // End of a given tree branch
    Node {
        value: i32,
        left: Box<BinarySearchTree>,
        right: Box<BinarySearchTree>,
    },
}

impl BinarySearchTree {
    fn new() -> Self {
        BinarySearchTree::Empty
    }

    // NOTE: We're only using a unique BSD - no two elements are the same
    fn insert(&mut self, new_value: i32) {
        // We need to traverse the tree
        match self {
            BinarySearchTree::Empty => {
                *self = BinarySearchTree::Node {
                    value: new_value,
                    left: Box::new(BinarySearchTree::Empty),
                    right: Box::new(BinarySearchTree::Empty),
                }
            }
            BinarySearchTree::Node { value, left, right } => match new_value.cmp(value) {
                Ordering::Equal => (),
                Ordering::Less => left.insert(new_value),
                Ordering::Greater => right.insert(new_value),
            },
            // Can also do below
            // BinarySearchTree::Node { value, left, right } => {
            //     if *value < new_value {
            //         right.insert(new_value);
            //     } else if *value > new_value {
            //         left.insert(new_value);
            //     }
            // }
        }
    }

    fn contains(&mut self, search_value: i32) -> bool {
        match self {
            BinarySearchTree::Empty => {
                return false;
            }
            BinarySearchTree::Node { value, left, right } => {
                if *value == search_value {
                    return true;
                } else if *value < search_value {
                    return right.contains(search_value);
                } else {
                    return left.contains(search_value);
                }
            }
        }
    }
}

// A vector is a smart pointer!
// In this case, our file system can contain other file systems infinitely
#[derive(Debug)]
enum FileSystemEntity {
    Folder {
        name: String,
        content: Vec<FileSystemEntity>,
    },
    File {
        name: String,
    },
}

#[derive(Debug)]
enum LinkedList<T> {
    Empty, // A variant indicating the list is at the end
    // A linked list node needs to be a pointer to another linked list node, otherwise it would be a recursive type. This way, the compiler can allocate the approrpiate amount of memory for this linkedlist enum
    Node { value: T, next: Box<LinkedList<T>> },
}

#[derive(Debug)]
enum LinkedList2<'a, T> {
    Empty, // A variant indicating the list is at the end
    Node {
        value: T,
        next: &'a LinkedList2<'a, T>,
    }, // There's a need for lifetime specifiers here, so that we don't have a dangling reference. The compiler needs a guarantee that our next node has a shorter lifetime than this one
}

// Example - this will not compile no matter how hard we try. This is because even though we return the first node, the second node's lifetime will end and therefore the reference will be dangling!
// fn create_list<'a>() -> LinkedList2<'a, i32> {
//     // The complication with this implementation is that if second node is deallocated before first node, then we have a dangling reference!
//     let second_node = LinkedList2::Node {
//         value: 2,
//         next: &LinkedList2::Empty,
//     };
//     let first_node = LinkedList2::Node {
//         value: 1,
//         next: &second_node,
//     };

//     return first_node;
// }

// Box solution will work
fn create_list() -> LinkedList<i32> {
    // The complication with this implementation is that if second node is deallocated before first node, then we have a dangling reference!
    let second_node = LinkedList::Node {
        value: 2,
        next: Box::new(LinkedList::Empty),
    };
    let first_node = LinkedList::Node {
        value: 1,
        next: Box::new(second_node),
    };

    return first_node;
}

fn main() {
    println!("Raw Pointers and Unsafe Code");

    let sushi = String::from("Yellowtail");
    let sushi_reference = &sushi; // This is a reference
    let sushi_reference_2 = &sushi; // We can make as many references as we want if it's to an immutable variable. This will break if we make the variable mutable

    let sushi_raw_pointer_1 = &raw const sushi; //A raw pointer that points to the address of the sushi variable
    let sushi_raw_pointer_2: *const String = &sushi; // A regular reference being forced into a raw pointer

    // Unsafe example of code! We can make as many raw mutable pointers as we want, but they all point to the same address. The compiler will only show a violation when we try to dereference the pointer, hence
    // why we would need an "unsafe" block to dereference it without the compiler complaining
    let mut sushi = String::from("Yellowtail");
    let sushi_raw_mutable_pointer_1 = &raw mut sushi; // A raw mutable pointer that points to the address of the sushi variable
    let sushi_raw_mutable_pointer_2 = &raw mut sushi; // Another raw mutable pointer!

    unsafe {
        println!("{}, {}", *sushi_raw_pointer_1, *sushi_raw_pointer_2);
    }
    println!("Now we're dropping the variable while keeping the raw pointers dangling");
    drop(sushi); // We drop the variable, leaving the raw pointers dangling. This means our pointer is now pointing to invalid memory! It may contain something totally different from before now!
    // This keyword acknowledges the possibility of undefined behavior so the compiler allows us to do whatever we want.
    unsafe {
        println!("{}, {}", *sushi_raw_pointer_1, *sushi_raw_pointer_2);
    }

    println!("Smart Pointers");

    println!("The Box Smart Pointer");
    // Now 100 is stored on the heap instead of the stack. The pointer to that box struct is stored on the stack
    let my_box = Box::new(100);
    println!("{}", *my_box);
    println!("{}", my_box); // Gives us the same as the above

    // Because the box is heap-allocated, it's an owned type and doesn't implement the copy trait
    let new_box = my_box; // Now my_box has changed ownership
    println!("{}", new_box);
    // println!("{}", my_box); // Will not work

    println!("Intro to Linked Lists");

    println!("Defining a Linked List, Creating a Linked List");
    let list = LinkedList::Node {
        value: 1,
        next: Box::new(LinkedList::Empty),
    };
    println!("{list:#?}");

    let list = LinkedList::Node {
        value: 1,
        next: Box::new(LinkedList::Node {
            value: 2,
            next: Box::new(LinkedList::Empty),
        }),
    };
    println!("{list:#?}");

    let last_node = LinkedList::Node {
        value: String::from("Eminem: Not Afraid"),
        next: Box::new(LinkedList::Empty),
    };

    let second_to_last_node = LinkedList::Node {
        value: String::from("Roy Jones Jr.: Can't be Touched"),
        next: Box::new(last_node),
    };

    let third_to_last_node = LinkedList::Node {
        value: String::from("Eminem: Without Me"),
        next: Box::new(second_to_last_node),
    };

    println!("Box vs. Regular References");
    // The complication with this implementation is that if second node is deallocated before first node, then we have a dangling reference!
    let second_node = LinkedList2::Node {
        value: 2,
        next: &LinkedList2::Empty,
    };
    let first_node = LinkedList2::Node {
        value: 1,
        next: &second_node,
    };

    // drop(second_node); // This creates a dangling reference and causes problems. Lifetimes of these nodes are also coupled
    println!("{:#?}", first_node);

    // Boxes enable us to build a design that plain references do not!
    let out_box = create_list();
    println!("{:#?}", out_box);

    println!("Vectors are Smart Pointers");
    let rust_file = FileSystemEntity::File {
        name: String::from("my_rust_code.rs"),
    };
    let python_file = FileSystemEntity::File {
        name: String::from("my_python_code.py"),
    };
    let code_folder = FileSystemEntity::Folder {
        name: String::from("Code Stuff"),
        content: vec![rust_file, python_file], // NOTE: This takes ownership of the data from rust_file and python file
    };

    println!("Intro to Binary Search Trees and Creating a Binary Search Tree");
    let mut tree = BinarySearchTree::new();
    tree.insert(5);
    tree.insert(8);
    tree.insert(2);
    tree.insert(3);
    tree.insert(4);
    tree.insert(10);
    tree.insert(13);
    tree.insert(12);
    println!("{tree:#?}");

    let contains_4 = tree.contains(4);
    let contains_20 = tree.contains(20);
    let contains_13 = tree.contains(13);
    println!(
        "contains 4: {}, contains 20: {}, contains 13: {}",
        contains_4, contains_20, contains_13
    );

    println!("The Deref and DerefMut Traits");

    let custom_boxy = CustomBox::new(3.14);
    println!("{}", *custom_boxy); // This won't work because we haven't implemented the deref trait! Works once we implement the Deref trait
    println!("{}", custom_boxy.deref()); // Same thing

    let mut custom_boxy = CustomBox::new(3.14);
    *custom_boxy = 6.28;
    println!("{}", *custom_boxy);

    let mut custom_boxy_2 = CustomBox2::new(3.14, "Hello");
    *custom_boxy_2 = 6.28;
    println!("{}", *custom_boxy_2);

    println!("The Drop Trait");
}
