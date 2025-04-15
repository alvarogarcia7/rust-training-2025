#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests_benchmark_fibonacci {
    use p22::calc::{fibonacci_loop, fibonacci_rec};
    use test::Bencher;

    #[bench]
    fn bench_rec(b: &mut Bencher) {
        b.iter(|| fibonacci_rec(test::black_box(30)));
    }
    #[bench]
    fn bench_loop(b: &mut Bencher) {
        b.iter(|| fibonacci_loop(test::black_box(30)));
    }
}
