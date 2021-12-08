// Topic: Vectors

fn main() {
    let numbers = vec![10, 20, 30, 40];
    println!("Total number of elements in vector is: {}", numbers.len());

    for number in numbers {
        match number {
            30 => println!("thirty"),
            _ => println!("{}", number),
        }
    }
}
