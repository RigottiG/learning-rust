// Topic: Working with an Enum

enum Colors {
  Blue,
  Black,
  White,
}

fn print_color(color: Colors) {
  match color {
    Colors::Black => println!("color: Black"),
    Colors::Blue => println!("color: Blue"),
    Colors::White => println!("color: White"),
  }
}

fn main() {
  print_color(Colors::Black);
  print_color(Colors::Blue);
  print_color(Colors::White);
}
