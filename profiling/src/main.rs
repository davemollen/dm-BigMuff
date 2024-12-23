mod utils;
use big_muff::{BigMuff, Params};
use utils::generate_signal;

fn main() {
  let mut big_muff = BigMuff::new(44100.);
  let mut params = Params::new(44100.);
  params.set(0.5, 0.5, 0.5);

  loop {
    let input = generate_signal();
    big_muff.process(input, &mut params);
  }
}
