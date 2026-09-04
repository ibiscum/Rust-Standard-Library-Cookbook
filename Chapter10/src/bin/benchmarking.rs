use std::time::Instant;

pub fn slow_fibonacci_recursive(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => slow_fibonacci_recursive(n - 1) + slow_fibonacci_recursive(n - 2),
    }
}

pub fn fibonacci_imperative(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut penultimate;
            let mut last = 1;
            let mut fib = 0;
            for _ in 0..n {
                penultimate = last;
                last = fib;
                fib = penultimate + last;
            }
            fib
        }
    }
}

pub fn memoized_fibonacci_recursive(n: u32) -> u32 {
    fn inner(n: u32, penultimate: u32, last: u32) -> u32 {
        match n {
            0 => penultimate,
            1 => last,
            _ => inner(n - 1, last, penultimate + last),
        }
    }
    inner(n, 0, 1)
}

pub fn fast_fibonacci_recursive(n: u32) -> u32 {
    fn inner(n: u32, penultimate: u32, last: u32) -> u32 {
        match n {
            0 => last,
            _ => inner(n - 1, last, penultimate + last),
        }
    }
    match n {
        0 => 0,
        _ => inner(n - 1, 0, 1),
    }
}

fn time_it<F: FnMut() -> u32>(name: &str, mut f: F, n: u32, iterations: u32) {
    let start = Instant::now();
    let mut result = 0;
    for _ in 0..iterations {
        result = f();
    }
    let elapsed = start.elapsed();
    println!(
        "{}: fib({}) = {}, {} iterations in {:?}",
        name, n, result, iterations, elapsed
    );
}

fn main() {
    const N: u32 = 20;
    const ITERATIONS: u32 = 10_000;

    time_it("slow_fibonacci_recursive", || slow_fibonacci_recursive(N), N, ITERATIONS);
    time_it("fibonacci_imperative", || fibonacci_imperative(N), N, ITERATIONS);
    time_it("memoized_fibonacci_recursive", || memoized_fibonacci_recursive(N), N, ITERATIONS);
    time_it("fast_fibonacci_recursive", || fast_fibonacci_recursive(N), N, ITERATIONS);
}
