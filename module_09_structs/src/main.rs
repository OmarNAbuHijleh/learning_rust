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
        Struct methods - we can have more parameters than just self
    */

    let tmp_song = EminemSong {
        title: String::from("Without Me"),
        release_year: 2003,
        duration_secs: 180,
    };

    let tmp_song_2 = EminemSong {
        title: String::from("Lose Yourself"),
        release_year: 2003,
        duration_secs: 179,
    };

    let bool_result = (&tmp_song).is_longer_than(&tmp_song_2);
    println!(
        "{} is longer than {}: {}",
        tmp_song.title, tmp_song_2.title, bool_result
    );

    /*
        Calling methods from other methods
        We can call multiple struct methods in the same struct method
    */
    tmp_song_2.years_since_release();

    println!("{:?}", tmp_song_2);

    /*
        Associated Functions and Constructor Functions
        These are functions attached to a type

        A type creates what is often referred to as a "namespace" for associated functions. A namespace is a boundary of associated functions. Analagous to a folder and it's file contents

        Examples include String::from() or String::new(). These functions live on the string type

        We often use associated functions for constructors. Constructors are functions that create an instance of the associated type. An example is the String::new() function

        The community convention is to give the constructor the namne "new". Rust will know that it's an associated method if we avoid adding the "self" parameter as an argument into the function
    */

    let new_song = EminemSong::new(String::from("Not Afraid"), 2010, 240);
    println!("{:#?}", new_song);

    /*
        Multiple Impl Blocks
        A struct can define multiple impl blocks. Rust will combine all the contents into a single type definition. Some concepts in Rust will require this from a technical perspective
    */

    /*
        The Builder Pattern
        This is a design pattern.

        Allows us to chain multiple methods in sequence
    */

    let mut computer_instance = Computer::new(String::from("i3"), 20, 500);
    computer_instance
        .update_cpu(String::from("M4"))
        .update_memory(32)
        .upgrade_capacity(1000);

    println!("{:#?}", computer_instance);

    /*
        Tuple Structs
        There are 3 types of Structs in Rust - Named Field, Tuple Like, and Unit Like
        So far, we've been dealing with named field, where the data is associated with a variable name. Now we'll look at Tuple-Like structs

        A tuple struct is a struct that assigns each piece of data an order in line rather than a name

        To declare a struct as tuple like we use the '()' as part of the struct definition with data types defined in the "()"

        We use this because it allows us to ensure that function declarations take the desired type!
    */
    let work_shift = ShortDuration(8, 0);
    println!("{} hours and {} minutes", work_shift.0, work_shift.1);

    let era = LongDuration(5, 3);
    println!("{} years and {} months", era.0, era.1);

    /*
        Unit-Like Structs
        A unit is a tuple that contains no values. A struct without any fields may seem pointless but it has its use cases in design patterns
    */
    let val = Empty;

    /*
        Project
        Use struct update syntax to copy the `price` and `passengers`
        fields to a new Flight struct instance. Make sure to provide
        new Strings for the remaining fields to ensure ownership
        doesn't transfer. Assign the new Flight to a separate variable.
    */

    let mut flight_instance = Flight::new(
        String::from("America"),
        String::from("Palestine"),
        500.0,
        500,
    );
    flight_instance
        .change_destination(String::from("Mecca"))
        .increase_price()
        .itinerary();
    println!("{:#?}", flight_instance);

    let new_flight_instance = Flight {
        origin: String::from("Canada"),
        destination: String::from("America"),
        ..flight_instance
    };
    println!("{:#?}", new_flight_instance);
}

#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passengers: u32,
}

impl Flight {
    fn new(origin: String, destination: String, price: f64, passengers: u32) -> Self {
        return Self {
            origin,
            destination,
            price,
            passengers,
        };
    }

    fn change_destination(&mut self, new_destination: String) -> &mut Self {
        self.destination = new_destination;
        return self;
    }

    fn increase_price(&mut self) -> &mut Self {
        self.price *= 1.2;
        return self;
    }

    fn itinerary(&self) -> &Self {
        println!("{} -> {}", self.origin, self.destination);
        return self;
    }
}

struct Empty; // a unit like struct

// Hours and minutes
struct ShortDuration(u32, u32); // A tuple like struct
// Years and Months
struct LongDuration(u32, u32);

#[derive(Debug)]
struct Computer {
    cpu: String,
    memory: u32,
    hard_drive_capacity: u32,
}

impl Computer {
    fn new(cpu: String, memory: u32, hard_drive_capacity: u32) -> Self {
        return Self {
            cpu,
            memory,
            hard_drive_capacity,
        };
    }

    fn update_cpu(&mut self, new_cpu: String) -> &mut Self {
        self.cpu = new_cpu;
        return self; // Note that returning the struct reference is what allows us to chain multiple methods together (Builder Pattern)
    }

    fn update_memory(&mut self, new_amount: u32) -> &mut Self {
        self.memory = new_amount;
        return self;
    }

    fn upgrade_capacity(&mut self, new_amount: u32) -> &mut Self {
        self.hard_drive_capacity = new_amount;
        return self;
    }
}

#[derive(Debug)]
struct EminemSong {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

// This is where we declare all of our methods for our struct. We do this so that we have them all in one place
impl EminemSong {
    fn new(title: String, release_year: u32, duration_secs: u32) -> Self {
        //We can also replace "Self" with "EminemSong" here. this is for both the return type and the struct call
        return Self {
            title,
            release_year,
            duration_secs,
        };
    }
}

impl EminemSong {
    fn years_since_release(&self) {
        self.display_song_info_3();
        let num_years_since_release = self.calc_years_since_release();
        println!("Years since release {}", num_years_since_release);
    }
    fn calc_years_since_release(&self) -> u32 {
        return 2025 - self.release_year;
    }
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

    fn is_longer_than(self: &Self, other_song: &EminemSong) -> bool {
        // Arguments can also be (&self, other_song: &Self) where "&Self" let's us know it's in reference to the struct "EminemSong"
        if self.duration_secs > other_song.duration_secs {
            return true;
        } else {
            return false;
        }
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
