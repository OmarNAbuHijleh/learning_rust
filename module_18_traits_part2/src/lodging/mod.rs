use std::collections::HashMap;
use std::fmt::Display;

pub trait Accommodation {
    fn book(&mut self, name: &str, nights: u32); // A mutable reference to an instance
}

pub trait Description {
    fn get_description(&self) -> String {
        return String::from("A wonderful place to stay");
    }
}


#[derive(Debug)]
pub struct Hotel<T> {
    name: T,
    reservations: HashMap<String, u32>,
}

impl<T> Hotel<T> {
    pub fn new(name: T) -> Self {
        Self {
            name: name,
            reservations: HashMap::new(),
        }
    }
}

impl<T: Display> Hotel<T> {
    pub fn summarize(&self) -> String {
        return format!("{}: {}", self.name, self.get_description())
    }
}

impl<T> Description for Hotel<T> {}

impl<T> Accommodation for Hotel<T> {
    fn book(&mut self, name: &str, nights:u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

#[derive(Debug)]
pub struct AirBnB {
    host: String,
    guests: Vec<(String, u32)>,
}

impl AirBnB {
    pub fn new(host: String) -> Self {
        Self {
            host: host,
            guests: Vec::new(),
        }
    }
}

impl Accommodation for AirBnB {
    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights))
    }
}

impl Description for AirBnB {
    fn get_description(&self) -> String {
        return format!("Please enjoy {}'s apartment", self.host);
    }
}
