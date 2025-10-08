// Module 7: References and Borrowing
fn main() {
    /*
        Immutable and Mutable Reference Parameters
        As it stands, when we pass a value through the function (as shown in the example below), we need a return value because of the ownership change that occurs.
        We can fix this problem through the use of references! as shown in the "show_meal" string. We can see the effects when we use it on a getter and mutator method

        We can define a parameter in the following possible ways:
        meal: String - Cannot change the original string
        mut meal: String - Can change the original string
        meal: &String - Cannot change the original string
        meal: &mut String - Can change the original string
    */

    let mut current_meal = String::new();
    current_meal = add_flour(current_meal);
    show_meal(&current_meal);

    add_flour_2(&mut current_meal); // Notice how we call an immutable reference here

    show_meal(&current_meal);

    /*
        Multiple Immutable References
        The benefit of a reference is that it allows a program to access and use a piece of data without duplicating it. 
        If one reference changes a value, it can cause problems with another reference in memory for that same data.

        Rust permits any number of immutable references to the same value at the same time - there is no limit!
        For mutable references, however, this is not the case. We're only allowed to have a single mutable reference at a time in Rust! This is restrictive but for the sake of memory safety
    */
    let car = String::from("red");
    // None of the below references are mutable, so we're allowed to have as many borrows as we want!
    let ref1 = &car;
    let ref2 = &car;
    let ref3 = &car;
    println!("{} and {} and {}", ref1, ref2, ref3);

    // Now doing the same thing but with mutable references
    let mut car = String::from("blue");
    let ref1 = &mut car;
    let ref2 = &car; // This  works for a specific reason - Rust knows that there was no change in the data from ref1, so it allows us to declare the reference variable "ref2"!
    //println!("{ref1} and {ref2}"); // This does not work because we're doing something with ref1 and it has the potential to change
    // This displays an advanced feature of Rust called "lifetimes". It's when, even though a variable's scope ends later, it's never used later and so it's "lifetime" ends early. In our case, line 40 for ref1
    println!("{ref2}"); // This works because we still haven't done anything to ref1 and therefore are in the clear at this point in the code

    // // An example where the lifetime is in conflict and therefore we cannot have multiple borrows at once!
    // let mut car = String::from("yellow");
    // let ref1 = &mut car;
    // let ref2 = &car; // Doesn't work because the lifetime of ref1 extends past this line!
    // ref1.push_str(" and Silver");
    // println!("{ref2}"); // This will not work because ref2 is dealing with a reference that has already changed!

    /*
        Ownership with Immutable and Mutable References
        Immutable references implement the copy trait if it's a stack item
    */
    let coffee = String::from("coffee");
    let a = &coffee;
    let b = a;
    let c = &coffee; // This is functionally equivalent to what happens in the above line
    println!("{a} and {b}");

    // // An example that doesn't work - this happens because b takes ownership from a, so the print statement doesn't work!
    // let mut coffee = String::from("coffee");
    // let a = &mut coffee;
    // let b = a;
    // println!("{a} and {b}");

    /*
        Dangling References
        This is a pointer to an address in memory that has been deallocated. The Rust compiler saves us from this - we can see an example in the create_city function defintiion
    */

    /* 
        Ownership with Arrays and Tuples 
        
        Note that the lesson here applies to all composite types!
    */ 
    let registrations = [true, false, true]; 
    let first = registrations[0]; // This is a boolean, which implements the copy trait. Rust is copying the value here and so we're not concerned about having anything that would change in the array if we change the "first" value!
    println!("{first} and {}", registrations[0]); // there's no issue here with ownership 

    let languages = [String::from("Arabic"), String::from("English"), String::from("Japanese")]; // Now we're storing a heap data type, since
                                                       // we're not using the fixed string version
    // let first_language = languages[0]; // This causes ownership conflict. The "languages" variable
                                       // owns the array and the array owns the string, but now the
                                       // "first_language" variable is fighting for ownership of
                                       // the string with the array! This is because they are both
                                       // on heap data 
    
    let first_language = &languages[0]; // Here, we can either implement the "clone" command to make
                                       // a brand new instance in memory or we can just borrow the
                                       // array content! This prevents a change in ownership
    
    // Project example
    let mut trip = start_trip(); 
    visit_philadelphia(&mut trip);
    visit_new_york(&mut trip);
    visit_boston(&mut trip);

    println!("{trip}");

}

fn start_trip() -> String {
    return String::from("The plan is ");
}

fn visit_philadelphia(input_string: &mut String) {
    // Concatenate "Philadelphia" to the end of the input_string
    input_string.push_str(" and Philadelphia");
}

fn visit_new_york(input_string: &mut String) {
    input_string.push_str(" and New York");
}

fn visit_boston(input_string: &mut String) {
    input_string.push_str(" and Boston");
}

fn add_flour(mut input_meal: String) -> String {
    input_meal.push_str("\nAdd Flour");
    return input_meal
}

// This alone causes a major issue - we're returning a reference to a piece of memory that is deallocated! The Rust compiler does not allow for this!
// This is an instance where we want to return the actual value directly for a transfer of ownership instead!
// fn create_city() -> &String{
    // let city = String::from("New York");
    // return &city;
// }


// An example using a mutator method - we now have a mutable reference to a string!
fn add_flour_2(input_meal: &mut String) {
    input_meal.push_str("\nAdd More Flour");
}

fn show_meal(meal: &String){
    println!("Meal Steps are: {meal}");

}


