mod utils;
use big_muff::BigMuff;
use utils::generate_signal;

fn main() {
  let mut big_muff = BigMuff::new(44100.);

  loop {
    let input = generate_signal();
    big_muff.process(input, 0.5, 0.5, 0.5);
  }
}
