#[derive(Debug)]
struct SupermarketItem {
    name: String,
    price: f64
}

#[derive(Debug)]
struct ShoppingCart {
    items: Vec<SupermarketItem>
}

impl ShoppingCart {
    fn traverse_items<F>(&mut self, mut operation: F)
    where F: FnMut(&mut SupermarketItem)
    {
        // Iterate over all items in the items vector
        let len_items = self.items.len();
        let mut current_idx = 0_usize;
        while current_idx != len_items {
            // Invoke the closure on the item
            operation(&mut self.items[current_idx]);
            current_idx += 1;
        }
    }

    fn checkout<F: FnOnce(ShoppingCart)>(self, operation: F) {
        operation(self);
    }
}

fn main() {
    let cart_items = vec![
        SupermarketItem{ name: String::from("APPLE"), price: 3.99 },
        SupermarketItem{ name: String::from("BANANA"), price: 2.99 }
    ];
    let mut our_cart = ShoppingCart {
        items: cart_items
    };

    let discount = |item: &mut SupermarketItem| {
        item.price *= 0.85; // NOTE: Omar check this
    };
    our_cart.traverse_items(discount);

    let lowercase_closure = |item: &mut SupermarketItem| {
        item.name = item.name.to_lowercase();
    };

    our_cart.traverse_items(lowercase_closure);

    let mut total_price = 0.0;
    our_cart.checkout(|mut cart: ShoppingCart| {
        println!("{:?}", cart);
        cart.traverse_items(|cart_item: &mut SupermarketItem| {
            total_price += cart_item.price;
        });
    });

    println!("${total_price:.2}");
}
