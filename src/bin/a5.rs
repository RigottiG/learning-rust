// Topic: Looping using the loop statement

fn main() {
  let mut i = 0;

  loop {
    println!("{:?}", i);
    if i == 5 {
      break;
    }

    i += 1;
  }
}
