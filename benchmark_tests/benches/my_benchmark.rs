use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn foo1() -> usize {
    const CAPACITY : usize = 240;
    const IN_LOOPS : usize = 5000;

    let mut arr = [0; CAPACITY];
    for i in 0..CAPACITY {
        arr[i] = i;
    }
    let mut sum = 0;
    for _ in 0..IN_LOOPS {
        let mut s = 0;
        for i in 0..arr.len() {
            s += arr[i];
        }
        sum += s;
    }
    sum
}

fn foo2() -> usize {
    const CAPACITY : usize = 240;
    const IN_LOOPS : usize = 5000;

    let mut arr = [0; CAPACITY];
    for i in 0..CAPACITY {
        arr[i] = i;
    }
    let mut sum = 0;
    for _ in 0..IN_LOOPS {
        let s: usize = arr.iter().sum();
        sum += s;
    }
    sum
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("foo",  |b| b.iter(|| foo2() ));
    // c.bench_function("foo sum()", |b| b.iter(|| foo2() ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);