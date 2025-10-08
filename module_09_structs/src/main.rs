fn main() {
    /*
        Define a Struct

        A struct is a container for related pieces of data. They're used to represent complex data types with multiple pieces of information.
        Rust has 3 kinds of structs
        - Name Field Structs
        - Tuple Like
        - Unit Like

        We're going to start with name field structs. Note that they require all fields to be defined when creating an instance of the struct!
    */
    //    struct Coffee {
    //        //Pascal Case for the name of the struct, snake case for the individual fields
    //        name: String,
    //        price: f32,
    //        is_hot: bool,
    //    }

    // Creating an instance of the struct
    let moca: Coffee = Coffee {
        name: String::from("Moca"),
        price: 4.99,
        is_hot: true,
    };

    /*
        Accessing Struct Fields

        To access a struct field, we reference a struct variable, followed by a dot and the field name.

        Regarding ownership - a struct is the owner of its fields. Each of its fields are the owner of its value.

        When the struct goes out of scope, it's responsible for cleaning its fields. Each of its fields are responsible for cleaning their respective values. It's a cascading cleanup
    */

    let moca: Coffee = Coffee {
        name: String::from("Moca"),
        price: 4.99,
        is_hot: true,
    };

    println!("{}", moca.name);

    let favorite_coffee = moca.name; // Since the name field of the struct is heap allocated data, we've now moved ownership from that field to the new variable

    let favorite_coffee_price = moca.price; // This is the copy trait in action so we're good

    /*
    Overwriting Struct Fields

    To change a field in a struct, the struct instance must be mutable and all of the fields become mutable with it - i.e. it's all or nothing
    */

    let mut beverage: Coffee = Coffee {
        name: String::from("Beverage"),
        price: 4.99,
        is_hot: true,
    };

    println!("{}", beverage.name);
    beverage.name = String::from("Caramel Machiato");
    println!("{}", beverage.name);

    /*
        Create Structs in a Function
        We're going to create a function that will make structs of a specific format

        We first have to define the struct format and then from there we can create a function to create more instances of the struct
    */
    let new_coffee = make_coffee(String::from("Omar's Coffee"), 100.00, true);

    println!("We made new coffee");

    // We need to briefly discuss ownership in a case like the following:
    let coffee_name = String::from("Caramel");
    let new_coffee = make_coffee(coffee_name, 5.00, true); // This results in ownership of "coffee_name" moving to the "name" argument in the "make_coffee" function, which then moves to the "name" field in the coffee struct that we're returning

    /*
     *       Struct Field Initiialization Shorthand
     *       When a parameter name or field name is the same as in the struct, we can just write it
     *       as-is without the colon
     *   */

    /*
     *       Struct Field Initiialization Shorthand Syntax
     *
     *       Look at make_coffee_2
     *
     */

    /*
        Struct Update Syntax

        If we want to create a new struct and copy some of the fields values from an old struct, we can do it as follows

        NOTE: We need to be careful of the ownership issues that may appear as a result of this! Fields that implement the copy trait won't have issues, but the heap allocated contents won't be good
    */

    let new_coffee_2 = Coffee {
        name: String::from("Omar's New Coffee"),
        ..new_coffee // This is how we let rust know that we want to copy the other values that are in the Coffee instance "coffee_name" so long as it's not already defined
    };

    let new_coffee_3 = Coffee {
        name: new_coffee.name.clone(), // This one gets around issues related to borrowing, because we make a copy of the heap allocated value
        ..new_coffee // This is how we let rust know that we want to copy the other values that are in the Coffee instance "coffee_name" so long as it's not already defined
    };

    /*
        Passing Structs into a Function
        A fucntion can recieve a struct in 4 possible ways:
        1.) Defining a parameter that receives the struct as an immutable value (drink_coffee)
        2.) Defining a parameter that receives the struct as a mutable value (drink_coffee_2)
        3.) Passing in a reference instead of the instance itself (drink_coffee_3)
        4.) Passing in a mutable reference instead of the instance itself (drink_coffee_4)
    */

    /*
        Deriving Debug Trait for Structs
        Reminder: A trait is basically an interface (a contract that a type will implement a method)

        The display trait can be printed
        The debug trait - less human readable

        display trait example
        println!("{}", variable_name)

        debug trait example
        println!("{:?}", variable_name)

        pretty-print debug trait example
        printlnQ("{:#?}", variable_name)

        Structs do not implement either trait, but we can derive one. We'll do that later

        There's a quick and dirty debug trait method we can do above the definition for the struct. It's a default one that Rust implements and is somewhat reasonable as a debug trait
    */
    println!("{:?}", new_coffee_3);

    /*
        Defining Struct Methods

        A method is a function that belongs to an instance. Fields of a struct represent data while methods of a struct represent behavior
    */

    let tmp_song = EminemSong {
        title: String::from("Without Me"),
        release_year: 2003,
        duration_secs: 180,
    };

    tmp_song.display_song_info(); // This one changes ownership and then is cleaned up 
    // println!("{:?}", tmp_song); // Fails due to no longer existing

    let mut tmp_song = EminemSong {
        title: String::from("Without Me"),
        release_year: 2003,
        duration_secs: 180,
    };
    tmp_song.display_song_info_2(); // This will change the song and then deallocate from memory
    // println!("{:?}", tmp_song); // Fails due to no longer existing

    let tmp_song = EminemSong {
        title: String::from("Without Me"),
        release_year: 2003,
        duration_secs: 180,
    };
    tmp_song.display_song_info_3();
    println!("{:?}", tmp_song); // Works due to borrow

    let mut tmp_song = EminemSong {
        title: String::from("Without Me"),
        release_year: 2003,
        duration_secs: 180,
    };
    tmp_song.display_song_info_4();
    println!("{:?}", tmp_song); // Works due to borrow

    /*
        Methods and Multiple Parameters

    */
}

#[derive(Debug)]
struct EminemSong {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

// This is where we declare all of our methods for our struct. We do this so that we have them all in one place
impl EminemSong {
    fn display_song_info(self: Self) {
        // Can also just be "self" for the argument
        // "self" will represent a struct in some way

        /*
            4 ways of passing in the self
            1.) Immutable struct value ("self" takes ownership)
            2.) Mutable struct value ("self" still takes ownership, but the value can be changed)
            3.) Immutable struct reference ("self" borrows struct)
            4.) Mutable struct reference ("self" borrows struct and can change)
        */
        println!(
            "{} was released {} and is {} long.",
            self.title, self.release_year, self.duration_secs
        );
    }

    fn display_song_info_2(mut self: Self) {
        // Can also just be "mut self" for the field
        // "self" will represent a struct in some way

        /*
            4 ways of passing in the self
            1.) Immutable struct value ("self" takes ownership)
            2.) Mutable struct value ("self" still takes ownership, but the value can be changed)
            3.) Immutable struct reference ("self" borrows struct)
            4.) Mutable struct reference ("self" borrows struct and can change)
        */
        self.title = String::from("lose yourself");
        println!(
            "{} was released {} and is {} long.",
            self.title, self.release_year, self.duration_secs
        );
    }

    fn display_song_info_3(self: &Self) {
        // Can also just be "&self" for the field
        // "self" will represent a struct in some way

        /*
            4 ways of passing in the self
            1.) Immutable struct value ("self" takes ownership)
            2.) Mutable struct value ("self" still takes ownership, but the value can be changed)
            3.) Immutable struct reference ("self" borrows struct)
            4.) Mutable struct reference ("self" borrows struct and can change)
        */
        println!(
            "{} was released {} and is {} long.",
            self.title, self.release_year, self.duration_secs
        );
    }

    fn display_song_info_4(self: &mut Self) {
        // Can also just be "&mut self" for the field
        // "self" will represent a struct in some way

        /*
            4 ways of passing in the self
            1.) Immutable struct value ("self" takes ownership)
            2.) Mutable struct value ("self" still takes ownership, but the value can be changed)
            3.) Immutable struct reference ("self" borrows struct)
            4.) Mutable struct reference ("self" borrows struct and can change)
        */
        self.title = String::from("The Real Slim Shady");
        println!(
            "{} was released {} and is {} long.",
            self.title, self.release_year, self.duration_secs
        );
    }
}

// We first have to define the struct at this level of scope
#[derive(Debug)] // NOTE: This is used so that we can have the debug trait on the Coffee struct! This is the standard Debug trait representation
struct Coffee {
    name: String,
    price: f64,
    is_hot: bool,
}

fn drink_coffee(coffee: Coffee) {
    println!("Drinking my delicious {}", coffee.name); // Note that this function calls takes ownership from the original, leading to the memory being de-allocated after usage
}

fn drink_coffee_2(mut coffee: Coffee) {
    // This version makes the coffee vairable mutable
    println!("Drinking my delicious {}", coffee.name); // Note that this function calls takes ownership from the original, leading to the memory being de-allocated after usage
}

fn drink_coffee_3(coffee: &Coffee) {
    // This version makes the coffee vairable mutable
    println!("Drinking my delicious {}", coffee.name); // This is an example of an immutable reference. We borrow the coffee struct, but that's it
}

fn drink_coffee_4(coffee: &mut Coffee) {
    // This version makes the coffee vairable mutable
    println!("Drinking my delicious {}", coffee.name); // This is an example of a mutable reference. We borrow the coffee struct and we can change its contents
}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    return Coffee {
        name: name,
        price: price,
        is_hot: is_hot,
    };
}
// This is a valid shorthand approach if the parameter and field names are the same
fn make_coffee_2(name: String, price: f64, is_hot: bool) -> Coffee {
    return Coffee {
        name,
        price,
        is_hot,
    };
}
