#[derive(Debug)]
struct DeliSandwich {
    sandwich_name: String
}

#[derive(Debug)]
struct Treasure<T> {
    captain: String, 
    treasure: T, // We can use some generic type for the struct definition
}

impl Treasure<String> { // Defines the instances where the generic data type passed is a string only
    fn clean_treasure(&mut self) {
        self.treasure = self.treasure.trim().to_string()
    }
}

impl Treasure<i32> {
    fn set_bounty(&mut self, value_amount: i32) {
        self.treasure = value_amount;
    }
}

impl Treasure<[&str; 3]> {
    fn set_jewels(&mut self) {
        self.treasure = ["Diamond", "Ruby", "Sapphire"];
    }
}

impl <T> Treasure<T> { // methods that exist for all data types
    fn change_captain(&mut self, new_captain: String) {
        self.captain = new_captain;
    }

    fn get_treasure(self) -> T {
        return self.treasure;
    }
}

#[derive(Debug)]
enum CheeseSteak<T> {
    Plain,
    Topping(T),
}

fn main() {
    /*
        Intro to Generics
        A generic is a type argument. It is a placeholder for a future type

        For example, a function parameters is defined for the data type even though it's actual data is not yet determined. A generic is the same idea - we have the data type but not the value.
        Ex:
            fn identity(value: i32) -> i32 {
                value
            }

        We set our function to return an i32. What if we want to define the same function to return an i16 or a boolean? Normally, we have to go and define another version of the same function for those data types. Generic types enable us to solve this problem
        We use the fn keyword followed by the desired function name, but we also use the <> symbols with some alias for the generic data type (commonly, T). We use that T as the datatype for the rest of the function definition

        When compiling, the compiler will look at all data types that are used in the generic and define it's own version of each! Generics are only used in our code! This is referred to as Monomorphization
     */

    println!("{}", identity(5));
    println!("{}", identity(13.45));
    println!("{}", identity(true));

    println!("{:?}", identity(DeliSandwich{sandwich_name: String::from("Turkey")}));

    /*
        The Turbofish Operator

        The turbofish operator is "::<i32>" and is used when specifying the actual data type for our generic.
    */

    println!("{}", identity::<i32>(5)); // This is an example of using the turbofish operator


    /*
        Multiple Generics
        We're allowed to combine generic and concrete types together.
     */
    let tup = make_tuple("Omar", 13);
    let tup = make_tuple_2("Omar", "Abu-Hijleh"); // Note - for this one, we are mandated by the definition to ensure that the first and second arguments are of the same type
    let tup = make_tuple_3("Omar", 18.0); // Can have any two types
    let tup = make_tuple_3(12, 18.0); // 
    let tup = make_tuple_3(3.0, 18.0); // We can still have the same type for both!

    /*
        Generics in Structs
        We can add generics to structs
     */
    let mut treasure_instance = Treasure{captain: String::from("Blackbeard"), treasure: String::from("     The One Piece")};
    let mut treasure_instance_2 = Treasure{captain: String::from("Luffy"), treasure: 56};
    let mut treasure_instance_3 = Treasure{captain: String::from("Luffy"), treasure: ["Gold", "Silver", "Platinum"]};

    /*
        Generics and Impl Blocks

        When dealing with structs containing a generic type, we have to honor that with a block. We have multiple impl blocks for multiple types that can be passed into the generic.

        For methods that can have any type at all, we add <> after the impl keyword
     */
    treasure_instance.clean_treasure();
    treasure_instance_2.set_bounty(100000);
    treasure_instance_3.set_jewels();

    println!("\n\n\n");
    println!("Treasure 1 is {:?}", treasure_instance);
    println!("Treasure 2 is {:?}", treasure_instance_2);
    println!("Treasure 3 is {:?}", treasure_instance_3);
    treasure_instance.change_captain(String::from("Monkey D. Luffy"));
    println!("\n\n\n");
    println!("Treasure 1 is {:?}", treasure_instance);

    println!("\n\n\n");
    println!("Treasure 1 is {:?}", treasure_instance.get_treasure());
    println!("Treasure 2 is {:?}", treasure_instance_2.get_treasure());
    println!("Treasure 3 is {:?}", treasure_instance_3.get_treasure());


    /*
        Generics in Enums

     */

    let mut item: CheeseSteak<String> = CheeseSteak::Plain; // We explicity have to set the data type of the variable declaration here because it's an enum that allows for a generic, even if the variant doesn't implement that generic. The compiler has no way of knowing otherwise!
    let item2 = CheeseSteak::Topping("Onions");
    // We had to explicitly set the generic data type despite not having one for the variant we used. Rust will hold us accountable if the data type doesn't use that variant in the future
    item = CheeseSteak::Topping(String::from("Lettuce")); // This will work
    // item = CheeseSteak::Topping("Gold"); // This will not, since item was already defined as a CheeseSteak using a String, not an &str


    /*
    Invoke the `consume_entertainment` method on the
    ChatMessage storing a DigitalContent enum.
     
    Invoke the `retrieve_time` method on all 3 ChatMessage
    instances and print out each String's content.
    */

    let message =  ChatMessage{content: "Salaam Bro", time: String::from("2:39 AM")};
    let message2 =  ChatMessage{content: String::from("Salaam Dude"), time: String::from("2:39 AM")};
    let message3 =  ChatMessage{content: DigitalContent::AudioFile, time: String::from("2:39 AM")};

    message3.consume_entertainment();

    println!("message 1 time {}", message.retrieve_time());
    println!("message 2 time {}", message.retrieve_time());
    println!("message 3 time {}", message.retrieve_time());
}
#[derive(Debug)]
enum DigitalContent {
    AudioFile, 
    VideoFile,
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T, 
    time: String
}

impl ChatMessage <DigitalContent> {
    fn consume_entertainment(self) {
        println!("Consuming Contents {:?}", self.content);
    }
}

impl <T> ChatMessage<T> {
    fn retrieve_time(&self) -> &String {
        return &self.time;
    }
}

fn make_tuple_3<T, U>(first: T, second: U) -> (T, U) { // NOTE: This is an example of a tuple using 2 different generics in teh same function definiton
    return (first, second);
}

fn make_tuple_2<T>(first: T, second: T) -> (T, T) {
    return (first, second);
}

fn make_tuple<T>(first: T, second: i32) -> (T, i32) {
    return (first, second);
}

fn identity<T>(value: T) -> T{
    value
}