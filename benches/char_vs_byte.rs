#[macro_use]
extern crate bencher;

use bencher::Bencher;
use manger::{bytes, chars, Consumable};

fn char(bench: &mut Bencher) {
    bench.iter(|| {
        (0..1000).fold(0, |_, _| {
            <chars::alphabet::A>::consume_till_end_from("AaAaA")
                .map(|_| 1)
                .unwrap()
        })
    })
}

benchmark_group!(benches, char);
benchmark_main!(benches);
