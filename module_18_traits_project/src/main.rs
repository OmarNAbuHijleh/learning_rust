use std::fmt::{Debug, Formatter, Display, Result};


trait Drinkable {
    fn consume(&mut self);
    fn get_data(&self) -> String;
    fn stats(&self) {
        println!("{}", self.get_data());
    }
}

#[derive(Debug)]
enum Milk {
    Whole,
    Oat,
    Almond,
}

#[derive(Debug)]
struct Soda {
    calories: u32,
    price: f64,
    flavor: String,
    percentage: u32
}

impl Soda {
    fn new(calories: u32, price: f64, flavor: String) -> Self {
        Self {
            calories: calories,
            price: price,
            flavor: flavor,
            percentage: 100
        }
    }
}

impl PartialEq for Soda {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

impl Eq for Soda {}

impl Clone for Soda {
    fn clone(&self) -> Self {
        Self {
            calories: self.calories,
            price: self.price,
            flavor: self.flavor.clone(),
            percentage: self.percentage
        }
    }
}

impl Drinkable for Soda {
    fn consume(&mut self) {
        self.percentage = 0;
    }
    fn get_data(&self) -> String {
        format!("Flavor: {}, Calories: {}", self.flavor, self.calories)
    }
}

impl Display for Soda {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "** {} Soda **", self.flavor)
    }
}
struct Coffee<T> {
    kind: T,
    milk: Milk,
    ounces: u32,
}

impl <T> Coffee<T> {
    fn new(kind: T, milk: Milk, ounces: u32) -> Self {
        Self {
            kind: kind,
            milk: milk,
            ounces: ounces
        }
    }
}

impl<T: Debug> Debug for Coffee<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        return f.debug_struct("Coffee").field("kind", &self.kind).field("milk", &self.milk).field("ounces", &self.ounces).finish();

    }
}

impl<T: Display> Drinkable for Coffee<T> {
    fn consume(&mut self) {
        self.ounces = 0;
    }
    fn get_data(&self) -> String {
        format!("A delicious {} ounce {}", self.ounces, self.kind)
    }
}

fn main() {
    let mut latte = Coffee::new("Black", Milk::Almond, 10);
    println!("Latte Variable in Debug Format {:?}", latte);
    latte.consume();
    println!("Latte Variable in Debug Format {:?}", latte);

    let cappuccino = Coffee::new("Arabica", Milk::Oat, 20);
    println!("{}", cappuccino.get_data());

    let pepsi = Soda::new(160, 3.00, "Cherry".to_string());
    println!("{pepsi}");

    let mut coke = pepsi.clone();

    println!("{}", pepsi==coke);
    coke.consume();

    println!("{:?}", coke);

}
