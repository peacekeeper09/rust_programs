use std::collections::HashMap;

const ALERT_THRESHOLD: u32 = 10;

macro_rules! add_item {
    ($inventory:expr, $product:expr, $quantity:expr) => {
        $inventory.insert(String::from($product), $quantity);
    };
}

fn main() {
    let mut inventory = HashMap::<String, u32>::new();
    add_item!(inventory, "Product A", 20);
    add_item!(inventory, "Product B", 5);
    add_item!(inventory, "Product C", 8);

    for (product, &quantity) in &inventory {
        if quantity < ALERT_THRESHOLD {
            println!("Alert: {} is running low on inventory ({} items)", product, quantity);
        } else {
            println!("{}: {} items", product, quantity);
        }
    }
}
