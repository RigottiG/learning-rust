// Topic: Advanced match

enum Ticket {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
}

fn main() {
    let tickets = vec![
        Ticket::Backstage(50.0, "Guilherme".to_owned()),
        Ticket::Standard(15.0),
        Ticket::Vip(30.0, "Katlen".to_owned()),
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price, holder) => {
                println!("Backstage ticket holder: {:?}, price {:?}", holder, price)
            }
            Ticket::Vip(price, holder) => {
                println!("Vip ticket holder: {:?}, price {:?}", holder, price)
            }
            Ticket::Standard(price) => println!("Standard ticket price {:?}", price),
        }
    }
}
