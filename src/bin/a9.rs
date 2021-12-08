// Topic: Data management using tuples

fn create_coordinate() -> (i32, i32) {
    (5, 5)
}

fn main() {
    let (_, y) = create_coordinate();

    if y == 5 {
        println!("equal 5");
    } else if y > 5 {
        println!("greater than 5");
    } else {
        println!("less than 5");
    }
}
