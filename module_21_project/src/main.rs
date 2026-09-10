#![allow(unused, dead_code)]

use std::env;
use std::io;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Product {
    Blender,
    Microwave,
    Toaster,
    Fridge,
}

#[derive(Debug)]
struct CustomerOrder {
    product: Product,
    quantity: u32,
    shipped: bool,
}

impl CustomerOrder {
    fn new(product: Product, quantity: u32, shipped: bool) -> Self {
        Self {
            product,
            quantity,
            shipped,
        }
    }
}

#[derive(Debug)]
struct Customer {
    id: u32,
    orders: Vec<CustomerOrder>,
}

fn main() -> io::Result<()> {
    let mut orders = vec![
        CustomerOrder::new(Product::Blender, 3, false),
        CustomerOrder::new(Product::Microwave, 1, true),
        CustomerOrder::new(Product::Toaster, 2, false),
        CustomerOrder::new(Product::Microwave, 5, true),
        CustomerOrder::new(Product::Blender, 1, false),
        CustomerOrder::new(Product::Fridge, 10, false),
    ];

    let customer_ids_by_order = [2, 1, 2, 3, 4, 1];
    let blender_orders = orders.iter().filter(|customer_order| customer_order.product==Product::Blender).collect::<Vec<&CustomerOrder>>();
    println!("{blender_orders:#?}");

    let total_microwaves = orders.iter().fold(0_u32, |current_value, customer_order| -> u32 {
        if customer_order.product == Product::Microwave {
            return current_value + customer_order.quantity;
        }
        current_value
    });
    println!("{total_microwaves}");

    let mut args_in = env::args().skip(1); // We don't want the first element
    let quantity_value = args_in.next().unwrap_or(String::from("2")).parse::<u32>().unwrap();
    for customer_order in orders.iter() {
        if customer_order.quantity >= quantity_value {
            println!("{customer_order:#?}");
        }
    }

    let unshipped_orders = orders.iter().filter(|customer_order| !customer_order.shipped)
        .fold(HashMap::<&Product, u32>::new(), |mut current_map: HashMap<&Product, u32>, customer_order: &CustomerOrder| -> HashMap<&Product, u32> {
            let mut value = current_map.entry(&customer_order.product).or_insert(0);
            *value += &customer_order.quantity;
            return current_map;
    }); // .collect::HashMap<&Product, u32>();
    println!("{unshipped_orders:#?}");

    // Our warehouse worker informs us they've shipped
    // the next unshipped order. Find the first
    // unshipped order among the customer orders and
    // change its `shipped` field to `true`. Print out
    // the customer orders to confirm.

    println!("\n\n\n");
    let shipped_order = orders.iter_mut().find(|customer_order| !customer_order.shipped).unwrap();
    shipped_order.shipped = true;
    println!("{orders:#?}");

    println!("\n\n\n");
    let mut our_hashmap: HashMap<u32, Vec<CustomerOrder>> = HashMap::new();
    let mut customer_struct_vector = orders.into_iter().zip(customer_ids_by_order)
        .fold(our_hashmap, |mut current_hashmap: HashMap<u32, Vec<CustomerOrder>>, current_tuple: (CustomerOrder, u32) | {
            let current_order = current_tuple.0;
            let current_id = current_tuple.1;
            let value_vector = current_hashmap.entry(current_id).or_insert(Vec::<CustomerOrder>::new());
            value_vector.push(current_order);
            return current_hashmap;
        }).into_iter()
        .fold(Vec::<Customer>::new(), |mut current_vec, hashmap_tuple|{
            current_vec.push(Customer{id: hashmap_tuple.0, orders: hashmap_tuple.1});
            current_vec
        });
    customer_struct_vector.sort_by_key(|input| input.id);
    println!("{customer_struct_vector:#?}");
    Ok(())
}
