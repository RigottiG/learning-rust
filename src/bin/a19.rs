use std::collections::HashMap;

// Topic: HashMap

fn main() {
    let mut stock = HashMap::new();
    stock.insert("chairs", 5);
    stock.insert("beds", 3);
    stock.insert("tables", 2);
    stock.insert("couches", 0);

    let mut total_stock = 0;
    for (item, quantity) in stock.iter() {
        total_stock += quantity;

        let stock_count = if quantity == &0 {
            "out of stock";
        } else {
            format!("{:?}", quantity);
        };

        println!("item={:?}, stock={:?}", item, stock_count);
    }
    println!("total stock={:?}", total_stock);
}
