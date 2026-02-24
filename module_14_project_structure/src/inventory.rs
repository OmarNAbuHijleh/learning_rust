// Everything within here is part of the inventory module
pub const FLOOR_SPACE: i32 = 10000;
// const FLOOR_SPACE: i32 = 10000; // NOTE: This is how a private constant would be defined
pub const MANAGER: &str = "Ivan Inventory";

pub fn talk_to_manager() {
    println!("Hello, {}!", MANAGER);
}
