/*
 * The cell type and interior mutability pattern
 *
 * This section will cover Cell, RefCell, OnceCell, LazyCell, and Rc
 *
 * interior mutability is a rust design pattern where a value can mutate its internal state even when accessed through an immutable reference.
 *
 * We usually need something to be mutable to change it, but this relaxes that constraint. For example, our "ConcertTicket" struct may have some fields that are mutable and others that are not - so we don't want to make the entire struct mutable!
 *
 * We can use the Cell smart pointer for this reason. It's designed for struct fields that implement the copy trait, so things like integers, floats, booleans, characters, basic primitive values.
 */

/*
 * The RefCell Type
 * Heap types are large and expensive to copy. When working with heap data it's cheapter to use references, so we can use the RefCell to use the interior mutability pattern to heap data. Rust enforces the borrowing rules at run time, so we may get a run time panic!
 */

/*
 * Rc<T> - shared Ownership
 * Box<T> has one owner, but sometimes multiple parts of a signle-threaded program need to own the same object. That's "Rc" --> Reference Counted
 */

/*
 * Arc<T> - Shared Ownership Across Threads
 * Arc - "Atomically Reference Counted"
 * Reference count uses atomic operations so that multiple threads can safely modify the count.
 * Rc -> shared ownership single threaded
 * Arc -> shared ownership multi-threaded
 *
 * Arc does NOT make the contained object mutable. for that you need something like
 * Arc<Mutex<T>>
 */

/*
 * Weak<T> - avoiding reference cycles
 *
 * if A owns B and B owns A using Rc:
 * Rc<A>
 * Rc<B>
 * You can create something called a reference cycle, and the Reference Count will never equal 0, meaning you have a memory leak.
 *
 * Weak<T> sovles this. A weak reference DOES NOT contribute to the strong reference count.
 *
 * Rule of thumb is to use Rc<T> when a relationship represents shared ownership and Weak<T> when a relationship should allow access without keeping the target alive
 *
 * Rc::downgrade(&value) creates a weak reference and upgrade gives a strong reference
 *
 *
 * The main idea: Smart pointers are abstractions that combine pointer-like behavior with ownership, lifetime, sharing, synchronization, or other resource-management semantics
 */

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct ConcertTicket {
    section: String,
    seat: String,
    scanned: Cell<bool>,
}

// Immutable by default, as well as all fields. It's all or nothing on struct mutability by default. We use the Cell smart pointer to get around this
impl ConcertTicket {
    fn new(section:String, seat: String, scanned: bool) -> Self {
        Self {
            section: section,
            seat: seat,
            scanned: Cell::new(scanned)
        }
    }

    fn admit_attendee(&self) {
        self.scanned.set(true); // We're allowed to do this now! There's also a corresponding "get" function as well for Cell types
    }
}

#[derive(Debug)]
struct ConcertTicket2 {
    section: String,
    seat: String,
    scanned: bool,
}

impl ConcertTicket2 {
    fn new(section:String, seat: String, scanned: bool) -> Self {
        Self {
            section: section,
            seat: seat,
            scanned: scanned
        }
    }

}

#[derive(Debug)]
struct Node {
    name: String,
    next: RefCell<Option<Rc<Node>>>
}

#[derive(Debug)]
struct Node2 {
    next: RefCell<Option<Rc<Node2>>>,
    previous: RefCell<Option<Weak<Node2>>> // use weak here
}
fn main() {
    println!("The Cell Type and Interior Mutability Pattern");

    let ticket = ConcertTicket::new(String::from("A"), String::from("3"), false);
    println!("{ticket:#?}");
    ticket.admit_attendee();
    println!("{ticket:#?}");

    println!("\n\nThe RefCell Type");
    let ticket2 = RefCell::new(ConcertTicket2::new(String::from("A"), String::from("3"), false));
    println!("{ticket2:#?}");

    let my_borrow = ticket2.borrow(); // Borrowing our refcell. For a mutable reference, borrowmut
    println!("{:#?}", my_borrow);
    drop(my_borrow); // NOTE: this needs to be done so that we don't have two coexisting borrows at once when one is mutable. Can also do the above in its own {} and that will handle the lifetime for us

    let mut my_borrow = ticket2.borrow_mut(); // Borrowing our refcell. For a mutable reference, borrowmut
    my_borrow.scanned = true;
    println!("{:#?}", my_borrow);

    println!("Rc<T> - shared Ownership");

    let data = Rc::new(String::from("Hello"));
    let a = Rc::clone(&data);
    let b = Rc::clone(&data);
    // Each allocation keeps a reference count. When one Rc disappears, it decrements. When we hit 0 the data is dropped.
    // NOTE: Rc is intentionally not thread safe! For multiple threads, we use Arc<T> instead.


    println!("Arc<T> - Shared Ownership Across Threads");
    // NOTE: Rc -> shared ownership single threaded. Arc -> shared ownership, multi-threaded
    let data = Arc::new(String::from("Hello"));
    let data2 = Arc::clone(&data);

    thread::spawn(move || {
        println!("{data2}");
    });

    //NOTE: Arc does NOT make the contained object mutable
    let data = Arc::new(Vec::<i32>::new());
    // You can't have multiple threads do
    // data.push(42);

    let data = Arc::new(Mutex::new(Vec::<i32>::new())); // A thread needs to acquire the mutex. this is common in concurrency
    let mut data = data.lock().unwrap();
    data.push(42);

    println!("Weak<T> - avoiding reference cycles");
    // Below is the creation of a reference cycle - a owns a strong reference to b and b owns a strong reference to a
    // Creating the Node and denoting that it's ownership will be shared using Rc
    let a = Rc::new(Node {
        name: String::from("A"),
        next: RefCell::new(None), // We have an option enum value for the next node
    });

    let b = Rc::new(Node {
        name: String::from("B"),
        next: RefCell::new(None),
    });

    // Creating the strong references to one another
    *a.next.borrow_mut() = Some(Rc::clone(&b)); // Borrow_mut let's us obtain a mutable borrow at run time.
    *b.next.borrow_mut() = Some(Rc::clone(&a)); // Borrow_mut lets us obtain a mutable borrow at run time
    println!("A strong count: {}", Rc::strong_count(&a));
    println!("B strong count: {}", Rc::strong_count(&b));


    let a = Rc::new(Node2 {
        next: RefCell::new(None), // We have an option enum value for the next node
        previous: RefCell::new(None)
    });

    let b = Rc::new(Node2 {
        next: RefCell::new(None),
        previous: RefCell::new(None)
    });

    *a.next.borrow_mut() = Some(Rc::clone(&b));
    *b.previous.borrow_mut() = Some(Rc::downgrade(&a)); // "downgrade" indicates the creation of a "Weak" node which doesn't increment A's strong reference count. When the owning reference disappears, the nodes can be freed.

    if let Some(a) = b.previous.borrow().as_ref().unwrap().upgrade() {
        println!("Previous node is still alive");
    }


}
