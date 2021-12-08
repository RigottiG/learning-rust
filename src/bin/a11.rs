// Topic: Ownership

struct GroceryItem {
    id: i32,
    quantity: i32,
}

fn print_quantity(item: &GroceryItem) {
    println!("The quantity is {}", item.quantity);
}

fn print_id(item: &GroceryItem) {
    println!("The id is {}", item.id);
}

fn main() {
    let milk = GroceryItem {
        id: 1,
        quantity: 99,
    };

    print_quantity(&milk);
    print_id(&milk);
}
