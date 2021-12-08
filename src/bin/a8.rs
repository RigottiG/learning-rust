// Topic: Organizing similar data using structs

enum Flavor {
    Sparking,
    Sweet,
    Fruity,
}

struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Fruity => println!("Fruity"),
        Flavor::Sparking => println!("Sparking"),
        Flavor::Sweet => println!("sweet"),
    }

    println!("oz: {}", drink.fluid_oz)
}

fn main() {
    let fruity = Drink {
        flavor: Flavor::Fruity,
        fluid_oz: 0.5,
    };
    let sparking = Drink {
        flavor: Flavor::Sparking,
        fluid_oz: 0.9,
    };

    let sweet = Drink {
        flavor: Flavor::Sweet,
        fluid_oz: 1.2,
    };

    print_drink(fruity);
    print_drink(sparking);
    print_drink(sweet);
}
