use std::collections::HashMap;

const ALERT_THRESHOLD: u32 = 10;

fn main() {
    let mut inventory: HashMap<String, u32> = HashMap::new();
    inventory.insert(String::from("Product A"), 15);
    inventory.insert(String::from("Product B"), 5);
    inventory.insert(String::from("Product C"), 8);

    for (product, quantity) in &inventory {
        if *quantity < ALERT_THRESHOLD {
            println!("Alert: {} is running low on inventory ({} items)", product, quantity);
        } else {
            println!("{}: {} items", product, quantity);
        }
    }
}
