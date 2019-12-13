use easybench::{bench,bench_env};

pub fn foo1() -> usize {
    const CAPACITY : usize = 239;
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


pub fn foo2() -> usize {
    const CAPACITY : usize = 239;
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

fn main() {
    // Simple benchmarks are performed with `bench`.
    // println!("foo1 (loop): {}", bench(|| foo1() ));
    // println!("foo2 (.sum()): {}", bench(|| foo2() ));
    println!("foo1 (loop)    : {}", bench(|| foo1() ));
    println!("foo2 ( .sum() ): {}", bench(|| foo2() ));

    // If a function needs to mutate some state, use `bench_env`.
    // println!("reverse: {}", bench_env(vec![1,2,3], |xs| xs.reverse()));
    // println!("sort:    {}", bench_env(vec![1,2,3], |xs| xs.sort()));
}
