use module_18_traits_part2::lodging::{Hotel, AirBnB, Accommodation, Description};
use module_18_traits_part2::utils;


fn main() {
    // We have to make the traits public to use their methods here! If a trait is public, its methods are also public. The same is not the case for structs!
    let mut hotel = Hotel::new(String::from("The Luxe"));
    println!("{}", hotel.summarize());
    hotel.book("Dana", 5);

    let mut airbnb = AirBnB::new("Parker".to_string());
    println!("{}", airbnb.get_description());
    utils::book_for_one_night(&mut airbnb, "Dan");
    utils::mix_and_match(&mut hotel, &mut airbnb, "Phil");

    println!("Associated Constraints in a Trait");
}
