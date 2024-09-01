mod utils;
use sd1::SD1;
use utils::generate_signal;

fn main() {
  let mut sd1 = SD1::new(44100.);

  loop {
    let input = generate_signal();
    sd1.process(input, 0.5, 0.5, 0.5);
  }
}
