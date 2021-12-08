// Topic: Implementing functionality with the impl keyword

enum Color {
    Black,
    White,
}

impl Color {
    fn print(&self) {
        match self {
            Color::White => println!("White"),
            Color::Black => println!("Black"),
        }
    }
}
struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}

struct ShippingBox {
    dimensions: Dimensions,
    color: Color,
    weight: f64,
}

impl ShippingBox {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            color,
            weight,
            dimensions,
        }
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight)
    }
}

fn main() {
    let small_dimensions = Dimensions {
        width: 1.0,
        height: 2.0,
        depth: 3.0,
    };

    let small_box = ShippingBox::new(1.0, Color::Black, small_dimensions);

    small_box.print();
}
