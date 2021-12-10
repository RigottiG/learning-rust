// Topic: Option
struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let student = Student {
        name: String::from("Guilherme"),
        locker: Some(22312),
    };

    match student.locker {
        Some(locker) => println!("{:?} student's locker is {:?}", student.name, locker),
        None => println!("student {:?} doesn't have a locker", student.name),
    }
}
