// Topic: Result

struct Customer {
    age: i32,
}

fn try_purchase(customer: &Customer) -> Result<(), String> {
    if customer.age >= 21 {
        return Ok(());
    }

    return Err("customer must be at least 21 years old".to_owned());
}

fn main() {
    let guilherme = Customer { age: 22 };
    let purchased = try_purchase(&guilherme);
    println!("{:?}", purchased);
}
