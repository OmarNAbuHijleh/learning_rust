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

use std::cell::Cell;
use std::cell::RefCell;

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
}
