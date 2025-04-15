#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use p22::calc::celsius2fahrenheit;
    use test::Bencher;

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| celsius2fahrenheit(test::black_box(1000)));
    }
}
