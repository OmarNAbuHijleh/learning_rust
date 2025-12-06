fn main() {
    /*
        Vectors
        A vector is a data structure similiar to an array. While an array is fixed in length, a vector is a container that stores values of the same type in order but the advantage is that a vector can grow and shrink in size. Rust stores the vector on a heap at run time for this reason.

        The type of a vector in Rust is Vec, and it is included in the Rust prelude so you do not need to import it to use it. 

        Vectors don't implement the display trait, but they do implement the debug trait, so you can print them using the {:?} format specifier.
    */
    let rick_moranis_movies: [&str; 3] = ["Ghostbusters", "Honey I Shrunk the Kids", "Little Shop of Horrors"]; //This is an array

    let pizza_diameters: Vec<i32> = Vec::new(); //This creates a new empty vector
    let pizza_diameters = Vec::<i32>::new(); //This creates a new empty vector - this one uses the turbofish operator
    println!("Pizza diameters: {:?}", pizza_diameters);

    let pastas = Vec::<&str>::new();
    println!("{:?}", pastas);

    // If we know the values we want to populate the vector at compile time, we can use the vec! macro
    let pizza_diameters = vec![12, 16, 18, 24]; // We're populating the vector with initial values and letting it infer the data type. If we do not want it to infer the data type, we can use the turbofish operator like so: let pizza_diameters = vec::<i32>[12, 16, 18, 24]; or we could just specify the type when we declare the variable like so: let pizza_diameters: Vec<i32> = vec![12, 16, 18, 24];
    println!("Pizza diameters: {:?}", pizza_diameters);

    /*
        Adding and Removing Elements
        - push adds an element to the end of the vector. Note that for us to be able to add/remove elements to a vector, it must be mutable.
        - the insert method adds an element at a specified index, shifting all elements after it to the right.
        - pop removes the last element from the vector and returns it as an Option<T> enum. This is because the vector could be empty, in which case it would return None.
        - the remove element removes an element at a specified index, shifting all elements after it to the left. Unlike the pop, if the element doesn't exist it will panic at runtime
     */
    let mut pizza_diameters = vec![12, 16, 18];
    pizza_diameters.push(24); //Adds 24 to the end of the vector
    pizza_diameters.push(30); //Adds 24 to the end of the vector
    println!("Pizza diameters: {:?}", pizza_diameters);
    pizza_diameters.insert(1, 14);
    println!("Pizza diameters: {:?}", pizza_diameters);

    let pop_result = pizza_diameters.pop();
    println!("Popped diameter: {:?}, Pizza diameters: {:?}", pop_result, pizza_diameters);

    let remove_result = pizza_diameters.remove(2);

    println!("Removed diameter: {}, Pizza diameters: {:?}", remove_result, pizza_diameters);

    /*
        Reading the Vector Elements
        Just like in an array, we can access a vector element by its position. We can do it by value or by reference to that value
        Attempting to read an index that does not exist will cause a panic at runtime, not compile time. This is not the case with an array because an array has a fixed size known at compile time.        

        We can also slice a vector
     */
    let pizza_diameters = vec![12, 16, 18, 24];

    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let beef = String::from("Beef");

    let pizza_toppings = vec![pepperoni, mushroom, beef]; // When we do this, ownership moves from the variables to the vector
    // println!("{:?}", pepperoni); // This will cause a compile time error because the ownership of the variable pepperoni has moved to the vector pizza_toppings
    let value = pizza_diameters[2]; //Accessing by value - this implements the copy trait since the i32 type implements the copy trait
    // let value2 = pizza_toppings[1]; // Accessing by value - this does not copy since String does not implement the copy trait, so this will cause a compile time error, as ownership will move from pizza toppings to value2 for "Mushroom"

    let reference = &pizza_toppings[2]; //Accessing by reference - this does not move ownership, so it's safe

    let pizza_slice = &pizza_diameters[1..3]; //This creates a slice of the vector from index 1 to index 2 (3 is exclusive)
    let pizza_slice = &pizza_diameters[1..]; //This creates a slice of the vector from index 1 to the ending index

    /*
        The Get Method
        The get method returns an Option<&T> enum. If the index exists, it returns Some(&T), otherwise it returns None. This is useful for handling cases where the index may not exist without causing a panic at runtime.
    */
    let optional_item = pizza_diameters.get(2);
    println!("Optional item at index 2: {:?}", optional_item);
    match optional_item {
        Some(topping) => println!("Topping is: {}", topping),
        None => println!("No topping found at that index"),
    }

    /*
        Ownership with Vectors
        Recall that if we have a mutable reference, we cannot have any other references to that data at the same time!

        Whenever we insert or move an element on a vector, that may require to memory allocation on the heap. Sometimes Rust will move or copy new elements to the heap. In this scenario, a reference to an element could find itself pointing to a deallocated piece of memory. Rust prevents this by enforcing the borrowing rules.

        When we move ownership of a vector to another variable, the original variable can no longer be used.
    */
    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let beef = String::from("Beef");
    let pizza_toppings = vec![pepperoni, mushroom, beef]; // When we do this, ownership moves from the variables to the vector
    let mut delicious_toppings = pizza_toppings; //Ownership of the vector moves to delicious_toppings

    let topping_reference = &delicious_toppings[1]; // We're borrowing an immutable reference

    delicious_toppings.push(String::from("Pineapple")); // topping_reference is a mutable reference borrow, but we're not using it further down in the program, so this is allowed. 
    // println!("Topping reference: {}", topping_reference); // Now we're using a mutable and immutable reference at the same time, which is not allowed

    /*
        Writing Vector Elements 
        We can overwrite a vector element at a specific index by using mutable references.
    */
    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let beef = String::from("Beef");
    let mut pizza_toppings = vec![pepperoni, mushroom, beef]; // When we do this, ownership moves from the variables to the vector

    pizza_toppings[1] = String::from("Olives"); //Overwriting the element at index 1
    println!("Pizza toppings: {:#?}", pizza_toppings);

    //Up to this point, the vector is the owner of the strings contained within it
    // let target_topping = pizza_toppings[2]; // This takes ownership of the string at index 2
    let target_topping = &mut pizza_toppings[2]; // This performs a mutable borrow
    // let another_reference = &mut pizza_toppings[2]; // This performs a mutable borrow, but fails because a mutable borrow already exists in target_topping
    target_topping.push_str(" and Sausage"); // Modifying the string via the mutable reference - now the pizza_toppings vector will reflect this change
    let another_reference = &mut pizza_toppings[2]; // before the above line, this code would fail because a mutable borrow already exists in target_topping, but now it works because target_topping is no longer used after the push_str call (i.e. the lifetime of "target_topping" ends there. If we use target_topping again after this line, the compiler would throw an error again)
    println!("Pizza toppings: {:#?}", pizza_toppings);

    /*
        Vector Capacity Behind the Scenes
        Every vector has a capacity (the max number of elements it can contain). When it hits full capacity, it'll find a larger memory space on the heap and move all of the previous elements over there. This is why Rust wants to prevent multiple mutable references to the vector at the same time, because if the vector reallocates, those references could point to invalid memory.

        We can set the vector capacity ahead of time using the with_capacity method. This is useful when we know how many elements we will need to store in the vector, as it can help prevent multiple reallocations.

        When we hit capacity, we double the size of the current capacity
    */
    let mut seasons: Vec<&str> = Vec::with_capacity(4);
    println!("Seasons length is {} and capacity is {}", seasons.len(), seasons.capacity());
    seasons.push("Spring"); 
    seasons.push("Summer"); 
    seasons.push("Fall");
    seasons.push("Winter"); 
    // Now we've hit the capacity of the vector. We can still continue growing it if we like though!
    seasons.push("Rainy");
    println!("Seasons length is {} and capacity is {}", seasons.len(), seasons.capacity());


    /*
        Mini Project
    */
    let mut my_folder = Folder::new(String::from("My Documents"));
    my_folder.create_file(String::from("File 1"));
    my_folder.create_file(String::from("File 2"));
    println!("Folder contents: {:#?}", my_folder);
    let removed_file = my_folder.delete_file(0);
    println!("Folder contents: {:#?}", my_folder);

    let gotten_file = my_folder.get_file(0);
    let missed_gotten_file = my_folder.get_file(100);

    match gotten_file {
        Some(file) => println!("Got file: {:?}", file),
        None => println!("No file found at that index"),
    }
    match missed_gotten_file {
        Some(file) => println!("Got file: {:?}", file),
        None => println!("No file found at that index"),
    }
    // println!("Gotten file: {:?}, Missed gotten file: {:?}", gotten_file, missed_gotten_file);

}

# [derive(Debug)]
struct File {
    name: String,
}

# [derive(Debug)]
struct Folder {
    name: String,
    contents: Vec<File>,
}

impl Folder {
    fn new(name: String) -> Self {
        return Folder {
            name, 
            contents: Vec::new(),
        }
    }

    fn create_file(&mut self, file_name: String) {
        let new_file = File {
            name: file_name,
        };
        self.contents.push(new_file);
    }

    fn delete_file(&mut self, index: usize) -> File {
        let removed_file = self.contents.remove(index);
        return removed_file;
    }

    fn get_file(&self, index: usize) -> Option<&File> {
        return self.contents.get(index);
    }
}
