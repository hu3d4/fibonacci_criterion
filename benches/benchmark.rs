use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fibonacci_number::array_fib;

fn criterion_benchmark(c: &mut Criterion) {
    const NUMBER: usize = 186;
    let mut array: [Option<u128>; NUMBER + 1] = [None; NUMBER + 1];
    c.bench_function("array_ver: fib 186", |b| {
        b.iter(|| array_fib(black_box(NUMBER), &mut array))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

////////////////////

// use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use fibonacci_number::vec_fib;

// fn criterion_benchmark(c: &mut Criterion) {
//     const NUMBER: usize = 186;
//     let mut vec: Vec<Option<u128>> = vec![None; NUMBER + 1];
//     c.bench_function("vec_ver: fib 186", |b| {
//         b.iter(|| vec_fib(black_box(NUMBER), &mut vec))
//     });
// }

// criterion_group!(benches, criterion_benchmark);
// criterion_main!(benches);
