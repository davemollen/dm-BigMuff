#[path = "../src/utils.rs"]
mod utils;
use big_muff::{BigMuff, Params};
use criterion::{criterion_group, criterion_main, Criterion};
use utils::generate_signal_stream;

fn big_muff_bench(c: &mut Criterion) {
  let mut big_muff = BigMuff::new(44100.);
  let mut params = Params::new(44100.);
  params.set(0.5, 0.5, 0.5);
  let signal_stream = generate_signal_stream(44100);

  c.bench_function("big_muff", |b| {
    b.iter(|| {
      for signal in &signal_stream {
        big_muff.process(*signal, &mut params);
      }
    })
  });
}

criterion_group!(benches, big_muff_bench);
criterion_main!(benches);
