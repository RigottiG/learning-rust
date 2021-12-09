// Topic Strings

struct Person {
    age: i32,
    name: String,
    favorite_color: String,
}

fn print(data: &str) {
    println!("{}", data)
}

fn main() {
    let people = vec![
        Person {
            age: 22,
            name: String::from("Guilherme"),
            favorite_color: "Blue".to_owned(),
        },
        Person {
            age: 20,
            name: String::from("Katlen"),
            favorite_color: "Purple".to_owned(),
        },
        Person {
            age: 10,
            name: String::from("John"),
            favorite_color: "Black".to_owned(),
        },
    ];

    for person in people {
        if person.age <= 10 {
            println!("{}", person.age);
            print(&person.favorite_color);
            print(&person.name);
        }
    }
}
