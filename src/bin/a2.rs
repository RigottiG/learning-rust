// Function to add two numbers together
fn sum(a: i32, b: i32) -> i32 {
  a + b
}

fn display_result(result: i32) {
  println!("{}", result);
}

fn main() {
  display_result(sum(5, 10));
  display_result(sum(2, 2));
}
