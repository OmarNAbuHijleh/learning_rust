/*
 * Manual Iteration
 * Iteration: repeating the same operation on a sequence of items one item at a time
 */
/*
 * The Iterator and Intolterator Traits
 * All iterators rely on the "next" method behind the scenes, and it is how they navigate to the next sequential item regardless of the exact iterator type that we have
 * The "next" method returns an option enum because it's not always guaranteed that the we'll get something, especially if we go out of bounds
 * The benefit of the iterator trait is consistency
 *
 * A type can be converted into a collection by using the "IntoIterator" trait, and by doing so it'll become something that implements the Iterator trait
 */

/*
 * The IntoIterator Trait in Action
 * When a type implements the "IntoIter" trait, the type promises to be convertable to an iterator
 */

/*
 * Exhausting the Iterator
 * Iterators in rust are lazy. They don't start iterating automatically until explicitly told to do so. Iteration cannot occur again after the iterator is "exhausted"
 */

/*
 * The for Loop with Iterator
 * The "for" loop automatically calls the "into_iter" function and takes ownership of the iterator in the background
 */

/*
 * Why Iterator Can Be Immutable
 */

/*
 * The iter Method
 * The iter method will NOT take ownership away from the original data source. It does this by creating an iterator of immutable references to the values of the collection type
 */

/*
 * The iter_mut Method
 * This will create an iterator that yields a mutable reference to each element
 */

/*
 * Hashmap Iteration
 * Hashmap iteration won't inherently be ordered the same every time, because hashmaps don't have an order
 * You'll have 2 pieces of data in every value from the hashmap
 */

/*
 * String Iteration
 * A string character we see is not always equivalent to an individual byte in memory. An english alphabetic character occupies a byte but an emoji might occupy 2 or 4 bytes. Since for strings, we can either iterate over the characters or the bytes in the string.
 *
 * Due to the ambiguity, we can't use the "iter" method or the "iter_mut" method. Instead, we can use the "chars" or the "bytes" method
 */

/*
 * Solving a Problem with Iteration"
 */

/*
 * The for_each Method
 * Instead of a for loop, we can invoke the "for_each" method on an iterator and pass in a closure to that method. It will operate on each element of the iterator
 */

/*
 * The map Method
 * Once we have an iterator, we can invoke methods on it to transform it into another iterator.
 * An "adapter" method is one that transforms an iterator into another iterator based on some logic
 * The methods consume the iterator. The contents however, are not consumed until the original iterator is exhausted
 *
 * map is the most popular adapter method. The "map" method applies a closure onto the original iterator to arrive at a new iterator of values
 *
 * "map" is lazy --> We may create an iterator with it but until we explicitly execute it the orignal data lives in it's previous owner!
 *
 * We can chain the map command multiple times in a chain of sequential transformations
 */

/*
 * The collect Method
 * Say we wanted to take a vector of numbers and use it as the basis of creating a new vector holding the squares.
 *
 * The collect method exhausts the iterator and gathers the resulting values in a new collection type, such as a vector. Think of it as going through a for loop and pushing each value into a new vector
  */

/*
 * The Filter and Find Methods
 * The filter method extracts a subset of values that satisfy a condition. Pass a closure that returns true for the elements to keep and false for the elements to exclude. It's similar to the "retain" method on strings
 */

/*
 * The any and all Methods
 * Sometimes we don't care about extracting every element that satisfies a condition, or the first element. Sometimes we just want to validate if all or one of the elements satisfy a condition (a boolean). Not the data itself
 *
 * We can still do that with the filter and find methods but we could also just make our lives easier by using the "any" and "all" methods
 *
 * A "predicate" in rust is a closure that returns a boolean. You'll sometimes see this in documentation
 */

/*
 * The cloned Method
 * Before, we introduced the "copied" method, which converts an iterator of references to a type to an iterator of the type itself. The caveat is that the type must implement the Copy trait.
 *
 * The cloned method similarly converts an iterator of references to a type into an iterator of the type itself. The type must implement the Clone trait. This is usually going to be heap based data.
 */

/*
 * The filter_map Method
 * This allows us to filter and transform a subset of elements from an iterator
 */

/*
 * The flatten Method
 * This is an adapter that returns an iterator that flattens nested data structures
 * The result is an iterator with a single value at each index
 */

/*
 * The flat_map Method
 * This combines the idea of a map and a flatten. It transforms the iterator
 */

/*
 * The enumerate Method
 * The enumerate adapter transforms an iterator such that the new iterator yields the index position along with the current element. It essentially gives you a tuple of the two
 */

/*
 * The partition Method
 * It's similar to filter in that it accepts a closure with a boolean, but the difference is that it groups and returns the values for which the closure returns true and false
 */

/*
 * The zip Method
 * This combines two iterators together if they have the same index positions. If they are different sizes, then it'll only zip to be the size of the smaller of the two, ignoring the elements that do not have a matching pair
 */

/*
 * The fold Method
 * This method exhausts an iterator to build up an produce a single value at the end of your iteration
 */

/*
 * The reduce Method
 * This is similar to the fold method, but it only accepts a closure as a single argument. It supplies the first element as the starter value automatically
 *
 * This method returns an Option enum to account for the possibility of an empty iterator. The fold method, on the other hand, will return your provided starter value if the iterator is empty
 */

/*
 * The sum, product, max, min, and count Methods
 * Adapter methods produce new iterators from other iterators. We also have functions that exhaust the iterator and produce a single result, like in reduce or fold. We also have the sum, product, max, min, and counts
 *
 */

/*
 * The last, nth, nth_back, and position Methods
 * Sometimes we want to target an element based on it's position in the iterator
 */

/*
 * the take, rev, skip, and step_by Methods
 * Sometimes we want to exlcude or skip certain elements, or start from a specific index
 */

/*
 * The sort and sort_by_key Methods
 * With a vector, we don't need to create an iterator. We have sorting methods
 */

/*
 * The lines Method
 * The string type supports the lines Method, which returns an iterator of the string's individual lines.
 */

/*
 * Collecting Command Line Arguments
 * these are values passed into a program from the terminal when the executable runs.
 * The rust standard library includes an env sub module for the environment.
 */

/*
 * Reading Directory
 * We can open a directory in our file system and read the contents as an iterator
 */

/*
 * The FromIterator Trait
 * An iterator can be converted into another type. It takes in an iterator and returns a collection type
 */

use std::{collections::HashMap, collections::HashSet, iter::zip, fs, io, env, process};

#[derive(Debug)]
struct Playlist {
    songs: Vec<String>,
    users: HashSet<String>
}

impl FromIterator<(String, String)> for Playlist {
    fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
        let mut songs = Vec::new();
        let mut users = HashSet::new();
        for (song, user) in iter {
            songs.push(song);
            users.insert(user);
        }
        Self {songs, users}
    }
}

#[derive(Debug)]
struct Settings {
    video_name: String,
    subtitles: bool,
    high_definition: bool,
}

#[derive(Debug)]
struct GasStation {
    snack_count: u32,
    manager: String,
    employee_count: u32
}

struct SupportStaff {
    day: String,
    employee: String
}

fn count_words(text: &str) -> HashMap<&str, u32> {
    // split the spaces
    let words = text.split_whitespace();
    let mut ret_hashmap = HashMap::new();
    for word in words {
        ret_hashmap.entry(word).and_modify(|input_word| {*input_word += 1}).or_insert(1_u32);
    }
    return ret_hashmap;
}

#[derive(Debug, PartialEq, Eq)]
enum ChannelType {
    Comedy,
    News,
    ProgrammingTutorials,
}

#[derive(Debug)]
struct TVChannel {
    name: String,
    channel_type: ChannelType
}

fn main() -> io::Result<()> {
    println!("Manual Iteration");
    let numbers = vec![4, 8, 15, 16, 23, 42];
    // 1.) The loop keyword - continually executes a block until we force termination with the "break" keyword
    let mut current_idx = 0;
    let final_index = numbers.len() - 1;
    loop {
        if current_idx > final_index {
            break;
        }

        println!("{}", numbers[current_idx]);
        current_idx += 1;
    }

    // 2.) The while loop
    let mut current_idx = 0;
    let final_idx = numbers.len() - 1;
    while current_idx <= final_idx {
        println!("{}", numbers[current_idx]);
        current_idx += 1;
    }

    // 3.) The for loop
    for number in numbers {
        println!("{}", number);
    }

    println!("The Iterator and Intolterator Traits");

    println!("The IntoIterator Trait in Action");
    let my_vector = vec![4, 8, 15, 16, 23, 42];
    let my_iterator = my_vector.into_iter(); // This is the "IntoIter" struct - it is different from the "IntoIter" trait!. Note that the <i32> in the "IntoIter" is because our vector was using i32
    // println!("{my_vector:?}"); // This will not work because we have changed the ownership of the data
    let mut my_hashmap = HashMap::new();
    my_hashmap.insert("CBS", 2);
    let my_iterator = my_hashmap.into_iter();

    println!("Exhausting the Iterator");
    let my_vector = vec![4, 8, 15, 16, 23, 42];
    let mut my_iterator = my_vector.into_iter();
    println!("{:?}", my_iterator);
    // We can invoke the "next" method to get the next value as a Some or None depending on if a value exists
    println!("{:?}", my_iterator.next()); // NOTE: Here we see the first value in the iterator provided
    println!("{:?}", my_iterator); // NOTE: Here we see that the iterator is missing the first value

    println!("The for Loop with Iterator");
    // The for loop will automatically call the "next" method repeatedly and unwrapping the value of the "Some" variant. It terminates upon arriving at the "None" variant
    // NOTE: The for loop automatically calls the "into_iter" method behind the scenes
    let my_vector = vec![4, 8, 15, 16, 23, 42];
    let mut my_iterator = my_vector.into_iter(); // This is redundant because of the previous note
    for number in my_iterator { // NOTE: Behind the scenes, ownership of the iterator moved to the for loop
        println!("{number}");
    }

    println!("Why Iterator Can Be Immutable");
    // NOTE: We don't have to make the iterator mutable. We can make the new owner variable mutable if desired. This happens implicitly behind the scenes in a for loop

    println!("The iter Method");
    let my_vector = vec![4, 8, 15, 16, 23, 42];
    let my_iterator = my_vector.iter(); // This is redundant because of the previous note
    for number in my_iterator { // NOTE: Behind the scenes, ownership of the iterator moved to the for loop
        println!("{number}");
    }
    println!("{my_vector:?}"); // Ownership has not transferred

    // NOTE: We can also achieve the same result by doing this:
    let my_vector = vec![4, 8, 15, 16, 23, 42];
    for number in &my_vector { // NOTE: Behind the scenes, ownership of the iterator moved to the for loop
        println!("{number}");
    }

    let cities = vec![String::from("Chicago"), String::from("Los Angeles")];
    for city in cities { // Doing this will transfer the ownership of the values in the vector to the "city" variable
        println!("{city}");
    }
    // println!("{cities:?}"); // This won't work because ownership of the values in cities was transferred to "city"

    println!("The iter_mut Method");
    let mut flavors = [String::from("Chocolate"), String::from("Vanilla"), String::from("Strawberry")];
    let iterator = flavors.iter_mut();
    for flavor in iterator {
        flavor.push_str(" Ice Cream"); // Appending to each element of the string
    }
    println!("{flavors:?}");

    // We can also do this
    let mut flavors = [String::from("Chocolate"), String::from("Vanilla"), String::from("Strawberry")];
    // let iterator = flavors.iter_mut();
    for flavor in &mut flavors { // This achieves the same effect as the above code
        flavor.push_str(" Ice Cream"); // Appending to each element of the string
    }
    println!("{flavors:?}");
    // NOTE: With some data types, we still need to use the de-reference operator (the "*") to access the value at the address if we want to modify it, because not every method (like "push_str") will do that for us

    let mut school_grades = [85, 90, 72, 92];
    for grade in &mut school_grades {
        // grade -= 2; // This won't work and will try to perform the actionn on the reference. We have to de-reference it
        *grade -= 2;
    }
    println!("{school_grades:?}");

    println!("Hashmap Iteration");
    let mut todos = HashMap::new();
    todos.insert("Pick up groceries", false);
    todos.insert("Study Rust", true);
    todos.insert("Sleep", false);

    for (todo_key, todo_val) in todos { // moves ownership
        println!("Task: {todo_key}, Complete: {todo_val}");
    }
    // println!("{todos:?}"); // will not work because ownership moved

    let mut todos = HashMap::new();
    todos.insert("Pick up groceries", false);
    todos.insert("Study Rust", true);
    todos.insert("Sleep", false);

    for (todo_key, todo_val) in &todos { // does not move ownership
        println!("Task: {todo_key}, Complete: {todo_val}");
    }
    println!("{todos:?}"); // will work because ownership did not move

    let mut todos = HashMap::new();
    todos.insert("Pick up groceries", false);
    todos.insert("Study Rust", true);
    todos.insert("Sleep", false);

    for (_, todo_val) in &mut todos { // does not move ownership. Can use "_" to throw away a value
        *todo_val = true; // needed to change the value
    }
    println!("{todos:?}"); // will work because ownership did not move. Values successfully changed

    println!("String Iteration");
    let seafood = "Oyster🦪"; // This emoji occupies 4 bytes in memory
    for byte in seafood.bytes() {
        print!("{byte}/");
    }
    println!("{seafood}"); // This will not lose ownership

    for character in seafood.chars() {
        print!("{character}/");
    }
    println!("{seafood}"); // This will not lose ownership

    println!("{}", seafood.bytes().len()); // 10 bytes. These functions exhaust the iterator themselves
    println!("{}", seafood.chars().count()); // 7 chars. These functions exhaust the iterator themselves


    println!("Solving a Problem with Iteration");
    let some_text = "Some text to try testing the iteration test method we made where test shows up twice";
    println!("{:?}", count_words(some_text));

    println!("The for_each Method");
    let mut our_hashmap = HashMap::<&str, i32>::new();
    some_text.split_whitespace().
        for_each(|word| {
                our_hashmap.entry(word).
                    and_modify(|val| {*val += 1}).
                    or_insert(1);
            });
    println!("{:?}", our_hashmap);

    println!("The map Method");
    let numbers = vec![4, 8, 15, 16, 23, 42];
    let my_iterator = numbers.iter();
    let squares = my_iterator.map(|element: &i32| {
        element.pow(2)
    });
    // println!("{my_iterator:?}"); // NOTE: Doesn't work because my_iterator lost ownership
    println!("{squares:?}"); // NOTE: These still only show the original values from the "my_iterator". This is because we haven't executed the "squares" mapping yet - just set it up. We still need to iterate over it for the original values to change. map is a lazy method
    println!("{numbers:?}"); // NOTE: This still works because we borrowed references towards the original values --> the "iter" method

    for number in squares {
        println!("Square: {number:?}"); // NOTE: This will implement the squaring of the values. squares also loses it's values following this
    }

    println!("The collect Method");
    let numbers = vec![4, 8, 15, 16, 23, 42];
    let squares: Vec<i32> = numbers.iter().map(|number: &i32| {number.pow(2)}).collect();
    // let squares: Vec<_> = numbers.iter().map(|number: &i32| {number.pow(2)}).collect(); // Here the "_" tells the compiler to try and figure out the type of data type the vector will contain. You can also use the turbofish operator with the collect method to establish what the data type will be
    println!("{squares:?}");
    println!("{numbers:?}");


    println!("The map method continued");
    let names = [
        String::from("Jimmy"),
        String::from("Cleveland"),
        String::from("Boris"),
    ];
    // We're going to lowercase each string, replace each "i" character with 2 "@" signs, then take the length of each resulting string and return all of those lengths in a new vector

    let collection_results: Vec<i32> = names.iter().map(
        |input_str| {
            input_str.to_lowercase()
        }
    ).map(
        |input_str| {
            input_str.replace("i", "@@")
        }
    ).map(
        |input_str| {
            input_str.chars().count() as i32
        }
    ).collect();

    println!("{collection_results:?}");

    println!("The Filter and Find Methods");
    let numbers = [100, 13, 23, 2, 8, 9, 6];
    // NOTE: The filter method takes in a reference to the values instead of copying them
    // NOTE: We need the "collect" method to actually perform the filtering. The "filter" method is lazy!
    // NOTE: Here, we're not using "*input_val" in the closure because the data type is one that Rust automatically de-references for us
    let evens: Vec<i32> = numbers.into_iter().filter(|input_val| {return (input_val % 2) == 0}).collect();
    println!("{evens:?}");

    // NOTE: Here, we use the "copied" command. This is useful when we have an iterator over a reference to some type instead of the actual value itself.
    let evens: Vec<i32> = numbers.iter().filter(|input_val| {return (*input_val % 2) == 0}).copied().collect();
    println!("{evens:?}");

    // NOTE: The "find" method will return the first element for which the closure is true
    let numbers = [100, 13, 23, 2, 8, 9, 6];
    let first_even = numbers.into_iter().find(|input_val| {return (input_val % 2) == 0}).unwrap();
    println!("{first_even}");

    let nothing = numbers.into_iter().find(|input_val| {return (*input_val > 100) }).unwrap_or(-1);
    println!("{nothing}");
    // NOTE: This is the "rfind" - searches for the first value from the end
    let last_even = numbers.into_iter().rfind(|input_val| {return (input_val % 2) == 0}).unwrap();
    println!("{last_even}");

    // Now we're going to apply these concepts to other data types, like structs!
    let channels = [
        TVChannel{name: String::from("CBS"), channel_type: ChannelType::Comedy},
        TVChannel{name: String::from("RustLive"), channel_type: ChannelType::ProgrammingTutorials},
        TVChannel{name: String::from("NBC"), channel_type: ChannelType::News},
        TVChannel{name: String::from("RustTV"), channel_type: ChannelType::ProgrammingTutorials},
    ];

    let good_channels: Vec<&TVChannel> = channels.iter().filter(|channel| {return channel.channel_type == ChannelType::ProgrammingTutorials}).collect();
    println!("{good_channels:?}");

    let good_channels: Vec<String> = channels.iter().filter(|channel| {return channel.channel_type == ChannelType::ProgrammingTutorials}).map(|channel| {return channel.name.clone()}).collect();
    println!("{good_channels:?}");

    let good_channel = channels.iter().find(|channel|{return channel.channel_type == ChannelType::ProgrammingTutorials}).unwrap();
    println!("{}", good_channel.name);

    let good_channels = channels.into_iter().filter(|channel| {return channel.channel_type == ChannelType::ProgrammingTutorials});
    println!("{good_channels:?}");

    println!("The any and all Methods");
    let channels = [
        TVChannel{name: String::from("CBS"), channel_type: ChannelType::Comedy},
        TVChannel{name: String::from("RustLive"), channel_type: ChannelType::ProgrammingTutorials},
        TVChannel{name: String::from("NBC"), channel_type: ChannelType::News},
        TVChannel{name: String::from("RustTV"), channel_type: ChannelType::ProgrammingTutorials},
    ];
    let any_comedy = channels.iter().any(|channel: &TVChannel| {channel.channel_type == ChannelType::Comedy});
    println!("{any_comedy}");
    let all_news = channels.iter().all(|channel: &TVChannel|{channel.channel_type == ChannelType::News}); // This will give us a false
    println!("{all_news}");

    println!("The Cloned Method");
    let teas = [
        String::from("Hot Earl Gray"),
        String::from("Iced Green"),
        String::from("Hot Matcha")
    ];
    let more_teas: Vec<&String> = teas.iter().collect(); // Note how we have to indicate that this will store string references
    println!("{more_teas:?}");

    println!("References:"); // NOTE: THAT THIS WILL HAVE THE SAME MEMORY ADDRESSES FOR THE STRING DATA AS THE "ORIGINAL TEAS"
    for tea in &more_teas {
        println!(
            "{:?} -> String object: {:p}, string data: {:p}",
            tea,
            *tea as *const String,
            tea.as_ptr()
        );
    }
    // Now, if we decide to use the "cloned" we get the same values as in teas, but they are completely decoupled (i.e. ownership is not passed from "teas" to "more_teas")
    let more_teas: Vec<String> = teas.iter().cloned().collect();
    println!("{more_teas:?}");
    println!("{teas:?}");
    // Printing out the memory addressing for each of these guys
    println!("\nOriginal teas:");
    for tea in &teas {
        println!(
            "{:?} -> String object: {:p}, string data: {:p}",
            tea,
            tea as *const String,
            tea.as_ptr()
        );
    }

    println!("\nCloned more_teas:"); // NOTE: THIS HAS A DIFFERENT MEMORY ADDRESS FOR THE STRING DATA THAN THE ORIGINAL "TEAS"
    for tea in &more_teas {
        println!(
            "{:?} -> String object: {:p}, string data: {:p}",
            tea,
            tea as *const String,
            tea.as_ptr()
        );
    }

    // Pretend we want full copies of the strings that contain the word "Hot" from the original "teas" vector
    let filtered_teas: Vec<&String> = teas.iter().filter(|input| {input.contains("Hot")}).collect();
    println!("{filtered_teas:?}");
    // If we wanted to ensure that we had a Vec<String> instead of a Vec<&String>
    let filtered_teas: Vec<String> = teas.iter().filter(|input| {input.contains("Hot")}).cloned().collect(); // We could also swap the filter and cloned, but then that's more inefficient because we clone the data we don't want too
    println!("{filtered_teas:?}");

    println!("The filter_map Method");
    let stocks = ["nvda", "", "aapl", "", "mst", "goog"];
    // We want to get the stocks with the actual tickers and capitalize them as well
    let transformed_and_filtered: Vec<String> = stocks.iter().filter_map(|stock|{
        if stock.is_empty(){
            None
        } else {
            Some(stock.to_uppercase())
        }
    }).collect();

    println!("{transformed_and_filtered:?}");

    println!("The flatten Method");
    let spreadsheet = vec![
        [100, 200, 300],
        [123, 456, 789],
        [987, 654, 321]
    ];
    let value: Vec<i32> = spreadsheet.into_iter().flatten().collect();
    println!("{value:?}");

    println!("The flatten_map Method");
    let attendees = [
        "Bob, Mary, Kevin",
        "Mike, Robby, Matt, Austin",
        "Piers, Liam"
    ];

    // let attendees: Vec<&str> = attendees.iter().map(|group| {
    //     group.split(", ")
    // }).flatten().collect();

    // the below flat_map call gives us the same result
    let attendees: Vec<&str> = attendees.iter().flat_map(|group| group.split(", ")).collect();
    println!("{attendees:?}");

    println!("The enumerate Method");
    // let's make it so that every third person in this vector is the winner
    let applicants = vec!["Rob", "Bob", "Cob", "Alex", "Piers", "John", "Dan"];
    let winners: Vec<&str> = applicants.into_iter().enumerate().filter_map(|person_tuple| {
        if (person_tuple.0+1_usize) % 3 == 0 {
            Some(person_tuple.1)
        } else {
            None
        }
    }).collect();
    println!("{winners:?}");

    println!("The partition Method");
    let numbers = [4, 8, 15, 16, 23, 42];
    let grouped:  (Vec<i32>, Vec<i32>) = numbers.into_iter().partition(|number| { number % 2 == 0 });
    println!("{grouped:?}");

    println!("The zip Method");
    let first_names = ["Omar", "Mahmoud", "Zaid", "Amir"];
    let last_names = ["Abu-Hijleh", "Ahmed", "Abuhashish", "Breadman"];
    let combined = zip(first_names, last_names).map(|tuple_input|{
        // let mut return_string = String::new();
        // return_string.push_str(tuple_input.0);
        // return_string.push_str(" ");
        // return_string.push_str(tuple_input.1);
        // return_string
        let first_name = tuple_input.0;
        let last_name = tuple_input.1;
        return format!("{first_name} {last_name}");
    }).collect::<Vec<String>>();
    println!("{combined:?}");

    // NOTE: Can also call ".zip" on the first_names variable and pass in the last_names variable as an argument


    println!("The fold Method");
    let numbers = [3, 5, 7, 25, 8, 93, -3];
    let result_sum = numbers.into_iter().fold(0, |total, input_val| -> i32 {
        // This is a closure that adds all of the elements together
        return total + input_val;
    });
    println!("The total value is {result_sum}");
    // NOTE: We could also just use the "sum" method
    // We can also do something like constructing a hashmap

    let week = [
        SupportStaff{day: String::from("Monday"), employee: String::from("Brian")},
        SupportStaff{day: String::from("Tuesday"), employee: String::from("Cam")},
        SupportStaff{day: String::from("Wednesday"), employee: String::from("Walter")}
    ];

    let new_hashmap: HashMap<String, String> = week.into_iter().fold(HashMap::new(), |mut hashmap_so_far, support_staff_element|{
        hashmap_so_far.insert(support_staff_element.day, support_staff_element.employee);
        hashmap_so_far
    });
    println!("{new_hashmap:?}");


    println!("The reduce Method");
    let earnings = [4, 7, 9, 13];
    let sum = earnings.into_iter().reduce(|total, input| {total+input}).unwrap_or(0_i32);
    println!("{sum}");


    println!("The sum, product, max, min, and count Methods");
    let numbers = vec![4, 8, 15, 16, 23, 42];
    let total: i32 = numbers.iter().sum();
    println!("{total}");
    let product: i32 = numbers.iter().product();
    println!("{product}");
    let max: i32 = *numbers.iter().max().unwrap_or(&0_i32);
    println!("{max}");
    let min: i32 = *numbers.iter().min().unwrap_or(&0_i32);
    println!("{min}");
    let count = numbers.iter().count();
    println!("{count}");
    // NOTE: Some of these methods do not work on iterators of floating point values, because they can contain NaN. Though the NaN is the result of an invalid mathematical operation, the NaN value itself is of type floating point! That's why floats implement the partial order but not the order trait
    let invalid = 0.0 / 0.0;
    let numbers = vec![4.6, 8.8, 0.0/0.0, 6.2, f64::NAN];
    println!("{numbers:?}");
    let total: f64 = numbers.iter().sum(); // gives NAN since it's already adding NAN together
    println!("{total}");
    // let max = numbers.iter().max().unwrap(); // This doesn't work on floats iterators
    // THE FIX for total
    let total: f64 = numbers.iter().filter(|number| !number.is_nan()).sum();
    println!("{total}");
    // THE FIX FOR MAX - max works on the individual floats
    let max_of_numbers = numbers.iter().filter(|number| !number.is_nan()).copied().reduce(|accumulator, current| accumulator.max(current)).unwrap();
    println!("{max_of_numbers}");


    println!("The last, nth, nth_back, and position Methods");
    let performers = ["Rustful Five", "Rust in Peace", "Rustin Beiber"];
    let last_element = performers.into_iter().last().unwrap(); // Get's the last element of an iterator
    println!("{last_element}");
    // nth - gets the nth position element
    let second_element = performers.into_iter().nth(1).unwrap();
    println!("{second_element}");
    let second_to_last = performers.into_iter().nth_back(1).unwrap(); // NOTE: 0 indexing is also used when counting from the back in rust
    println!("{second_to_last}");
    // Sometimes we don't want the element, we want the index that contains an element fulfilling some requirement
    let target_index = performers.into_iter().position(|element|{element=="Rustin Beiber"}).unwrap();
    println!("{target_index}");

    println!("The take, rev, skip, and step_by Methods");
    let fifty_numbers = 1..50; // Everything up to 50 (but not including it)
    // The "take" method let's us limit the scope of the iteration. In this case, the first 15 elements
    for number in fifty_numbers.take(15) {
        print!("{number}/");
    }
    println!();
    // The "rev" method reverses the string
    let fifty_numbers = 1..50; // Everything up to 50 (but not including it)
    for number in fifty_numbers.rev() {
        print!("{number}/");
    }
    println!();
    // the "skip" method let's us skip the first n elements
    let fifty_numbers = 1..50; // Everything up to 50 (but not including it)
    for number in fifty_numbers.skip(5) {
        print!("{number}/");
    }


    println!("The sort and sort_by_key Methods");
    let mut points = [3, 8, 1, 11, 5];
    println!("{}", points.is_sorted()); // Gives back a boolean indicating if the vector/array is sorted
    points.sort(); // sorts the array
    println!("{}", points.is_sorted()); // Should be sorted now in decending order

    points.reverse();
    println!("{points:?}");
    println!("{}", points.is_sorted());

    let mut exercises = ["squat" , "bench", "Deadlift"]; // NOTE: Capital letters come before lowercase letters
    exercises.sort();
    println!("{exercises:?}");

    // We can sort using the ord trait or we can just sort by a key
    let mobile = GasStation{snack_count: 100, manager: String::from("Meg Mobil"), employee_count: 3};
    let exxon = GasStation{snack_count: 130, manager: String::from("Eric Exxon"), employee_count: 4};
    let shell = GasStation{snack_count: 50, manager: String::from("Shelly Shell"), employee_count: 2};

    let mut stops = [mobile, exxon, shell];
    // stops.sort(); // This doesn't work until we implement the ORD trait
    stops.sort_by_key(|station|{station.snack_count}); // Orders in ascending order based on what our closure returns
    println!("{stops:?}");


    println!("The lines Method");
    let contents = fs::read_to_string("story.txt")?; // Will return an Error if there's an issue
    for line in contents.lines() {
        println!("{line}");
    }

    println!("Collecting Command Line Arguments");
    // for arg in args {
    //     println!("{arg}");
    // }

    // Video Player Application command line arguments: video file name (string), subtitles (bool), high definition (bool)
    let settings = collect_settings();
    println!("{settings:?}");


    println!("Reading Directory");
    let directory = fs::read_dir("./").unwrap_or_else(|error| {
        eprintln!("could not read directory: {error}");
        process::exit(1);
    }); // Reads the current directory

    for entry_result in directory {
        // Each of these entries will either be a file or folder. This is a Result enum value because of the possibility something may go wrong (a file being deleted after our iterator is made, permissions, etc)
        match entry_result {
            Ok(entry) => println!("{:?}", entry.path()),
            Err(error) => eprintln!("Could not read entry: {error}")
        }
    }
    // Alternatively, we can also do this
    for entry_result in fs::read_dir("./")? { // Recall that the ? is the try operator. An error would terminate earlier and propogate errors upwards. The Ok variant is just unwrapped
        // match entry_result {
        //     Ok(entry) => println!("{:?}", entry.path()),
        //     Err(error) => {
        //         eprintln!("Could not read entry: {error}");
        //     }
        // }

        // We can also do this
        if let Ok(entry) = entry_result {
            // println!("{:?}", entry.path());

            let metadata = fs::metadata(entry.path())?; // Get the file's metadata
            if metadata.is_file() {
                println!("{entry:?}\n----------");
                let contents = fs::read_to_string(entry.path())?;
                println!("{contents}");
            }
        }
    }

    println!("The FromIterator Trait");
    let fifty_numbers = 1..=50;
    let results = Vec::from_iter(fifty_numbers.clone());
    println!("{results:?}");

    let results = fifty_numbers.clone().collect::<Vec<i32>>();
    println!("{results:?}");
    let unique_set: HashSet<_> = HashSet::from_iter(fifty_numbers.clone());
    println!("{unique_set:?}");

    let unique_set = fifty_numbers.clone().collect::<HashSet<i32>>();
    println!("{unique_set:?}");

    let chars = ['H', 'e', 'l','l','o'];
    let greeting = String::from_iter(chars);
    println!("{greeting}");

    let songs = [
        (String::from("I rust go on"), String::from("Bob")),
        (String::from("A rust of wind"), String::from("Bob")),
        (String::from("A rustworthy man"), String::from("Sheila")),
    ];

    let playlist: Playlist = Playlist::from_iter(songs.clone()); // Builds a playlist from an iterable of 2 string tuples
    println!("{playlist:?}");

    let playlist: Playlist = songs.into_iter().collect::<Playlist>();
    println!("{playlist:?}");


    Ok(()) // NOTE: Keep this because our main is returning an IO result
}

fn collect_settings() -> Settings {
    let args = env::args(); // The args struct returns an iterator. Note that the first value in the iterator is always the file name
    // NOTE: We pass in arguments from the command line as follows "cargo run -- <arg1>"
    // The "--" are to distinguish our command line arguments in our program from command line arguments for cargo
    let mut relevant_args = args.skip(1).take(3); // skips the first element, since that is just the file name. Then we only want the first 3 elements - any extra command line arguments are just dead
    let video_file = relevant_args.next().unwrap_or_else(||{
        eprintln!("No video file specified!");
        process::exit(1);
    });

    let mut settings = relevant_args.map(|setting| setting.parse::<bool>().unwrap_or(false));
    let subtitles = settings.next().unwrap_or(false);
    let high_definition = settings.next().unwrap_or(false);

    Settings {video_name: video_file, subtitles: subtitles, high_definition: high_definition}
}
