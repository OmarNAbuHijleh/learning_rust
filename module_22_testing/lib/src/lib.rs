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
 * there are several categories of tests:
 * unit tests - target a small component of a program in isolation
 * integration test - tests the interaction of multiple components within the program
 *
 * It's common to write unit tests along with implementation code in Rust. We still want to isolate the code and identify it separately from our source code though, so we'll usually define the module "tests" to contain our unit tests.
 *
 *
 * Another problem - we don't want our test code included in the binary executable when the compiler builds. We use the "#[cfg(test)]" to fix this problem
 */

/*
 * Test Failures
 * When the assertion fails
 */

/*
 * The assert! Macro
 * This validates that some condition or value is true. If it's false the assertion will fail
 */

/*
 * Testing Inequality with the assert_ne! Macro
 * Self explanatory
 */

/*
 * The pretty_assertions Crate
 * Sometimes viewing the deltas between an assert_eq! inputs can be hard. This crate makes it easier by coloring the diffs so that it's easier to determine what the differences are in the terminal
 *
 * "use pretty_assertions::{assert_eq, assert_ne};"
 */

/*
 * Trait Requirements for Types in Testing
 * The macros print in debug format. Custom Structs and enums don't implement the debug trait by default, but if we don't do that the macro won't work. We also need to define things like equality
 */

/*
 * Custom Failure Messages
 * The final argument to any assertion macro is a custom failure messages. It is optional. Remember, optional arguments don't exist in functions but macros are not functions!
 */

/*
 * The should_panic Attribute
 * We use this when something is expected to raise an error
 */

/*
 * Using Result Enum in Tests
 */

#[derive(Debug, Eq, PartialEq)]
struct Museum {
    paintings: Vec<String>,
    revenue: u32,
}

impl Museum
{
    const MAXIMUM_CAPACITY: usize = 3;
    fn new() -> Self {
        Self {
            paintings: vec![],
            revenue: 0,
        }
    }

    fn buy_painting(&mut self, painting: &str) {
        if self.paintings.len() >= Self::MAXIMUM_CAPACITY {
            panic!("Museum does not have storage space for another painting");
        }
        self.paintings.push(painting.to_string());
    }

    fn sell_ticket(&mut self) {
        self.revenue += 25;
    }

    fn has_impressive_collection(&self) -> bool {
        self.paintings.len() > 2
    }
}

// Define a new module "tests"
#[cfg(test)] // We need this to avoid including this in the comiplation of the module
mod tests {
    use super::*; // The use keyword makes a name available in the current scope or module. The super keyword reaches one module up into the library crate root and then the "*" (also known as the "blob" operator) brings all of the names from the module you are accessing into the current scope
    use pretty_assertions::assert_eq;

    #[test]
    fn museum_sells_ticket_to_increase_revenue() {
        let mut museum_instance = super::Museum::new(); // NOTE: We use the super keyword here because this module is sub module. We don't have to declare Museum to be public thanks to this module being in the same file. We can also use the "crate" keyword so that we can get something from within the lib directory
        let mut museum_instance = Museum::new(); // NOTE: We can just do this thanks to our "use" at the top of the module
        museum_instance.sell_ticket();
        assert_eq!(museum_instance.revenue, 25, "The revenue from selling 1 ticket did not match expectations");
    }

    // Example of test failure
    // #[test]
    // fn museum_can_sell_multiple_tickets() {
    //     let mut museum = Museum::new();
    //     museum.sell_ticket();
    //     museum.sell_ticket();
    //     assert_eq!(museum.revenue, 25);
    // }

    #[test]
    fn museum_can_sell_multiple_tickets() {
        let mut museum = Museum::new();
        museum.sell_ticket();
        assert_eq!(museum.revenue, 25);
    }

    #[test]
    fn museum_can_have_impressive_art_collection() {
        let mut museum = Museum::new();
        museum.buy_painting("Mona Lisa");
        museum.buy_painting("Monte Carlo");
        museum.buy_painting("Deep Learning");
        assert!(museum.has_impressive_collection(), "The museum did not have an impressive collection despite having more than 2 paintings");
    }

    #[test]
    fn new_museums_are_equal() {
        let museum_1 = Museum::new();
        let mut museum_1 = Museum::new();
        // museum_1.sell_ticket();
        let museum_2 = Museum::new();
        assert_eq!(museum_1, museum_2, "Two new musuem instances were not found to be equal: {museum_1:?} and {museum_2:?}");
    }


    #[test]
    // #[should_panic] // This is how we inform cargo that this test should panic
    #[should_panic(expected = "storage space")] // We do this when there's a specific type of panic we want to check for (in this case, specific string in the error message)
    fn museum_prohibits_adding_painting_when_capacity_has_been_reached() {
        let mut museum = Museum::new();
        museum.buy_painting("Mona Lisa");
        museum.buy_painting("Monte Carlo");
        museum.buy_painting("Deep Learning");
        museum.buy_painting("Flying Squirrels");
    }
}
