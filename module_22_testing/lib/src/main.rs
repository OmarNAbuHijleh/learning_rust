/*
 * Intro to Testing
 * You can't test everything in a code base manually, so you need to make sure that functionality doesn't break when you add or make changes
 * Test code validates our program code works as expected
 *
 * "cargo new --lib" creates a new library crate. Recall that library crates are code that are intended for use by binary crates and they have a lib.rs file.
 */

/*
 * Writing a Test and the assert_eq! Macro
 * A test in rust contains assertions. An asseertion is a verification that a statement is valid. To assert means to state a fact.
 *
 * Tests are to find "regressions" in your code - a regression is a bug introduced into working software
 *
 * A test is just a palin Rust function annotated with a "#[test]" attribute
 *
 * to run our tests we run "cargo test" in the terminal
 */

/*
 * The tests Module and the cfg Attribute
 */

#[derive(Debug)]
struct Museum {
    paintings: Vec<String>,
    revenue: u32,
}

impl Museum
{
    fn new() -> Self {
        Self {
            paintings: vec![],
            revenue: 0,
        }
    }

    fn buy_painting(&mut self, painting: &str) {
        self.paintings.push(painting.to_string());
    }

    fn sell_ticket(&mut self) {
        self.revenue += 25;
    }

    fn has_impressive_collection(&self) -> bool {
        self.paintings.len() > 2
    }
}


#[test]
fn museum_sells_ticket_to_increase_revenue() {
    let mut museum_instance = Museum::new();
    museum_instance.sell_ticket();
    assert_eq!(museum_instance.revenue, 25);
}


fn main() {
    println!("Hello, world!");
}
