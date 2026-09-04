fn main() {
    // A closure that uses the keyword "yield" is called a generator.
    // The unstable `Generator` trait is not available on stable Rust,
    // so we simulate the same behavior with a tiny state machine.
    let mut generator = SimpleGenerator::new(|state| match *state {
        0 => {
            *state = 1;
            Yielded(1)
        }
        1 => {
            *state = 2;
            Yielded(2)
        }
        _ => Complete(()),
    });
    if let Some(value) = generator.next() {
        println!("The generator yielded: {}", value);
    }
    if let Some(value) = generator.next() {
        println!("The generator yielded: {}", value);
    }
    // When there is nothing left to yield,
    // a generator will automatically return an empty tuple
    if generator.next().is_none() {
        println!("The generator completed with: ()");
    }

    // A generator can conceptually return a different type than it yields.
    let mut generator = SimpleGenerator::new(|state| match *state {
        0 => {
            *state = 1;
            Yielded(100)
        }
        1 => {
            *state = 2;
            Yielded(200)
        }
        2 => {
            *state = 3;
            Yielded(300)
        }
        _ => Complete("I'm a string"),
    });
    loop {
        match generator.next() {
            Some(value) => println!("The generator yielded: {}", value),
            None => {
                println!("The generator completed with: {}", generator.complete().unwrap());
                break;
            }
        }
    }

    // Generators are great for implementing iterators.
    // On stable Rust we can express the same idea directly.
    let fib: Vec<_> = fibonacci().take(10).collect();
    println!("First 10 numbers of the fibonacci sequence: {:?}", fib);
}

enum State<Y, R> {
    Yielded(Y),
    Complete(R),
}

use State::{Complete, Yielded};

/// A tiny stable-Rust stand-in for an unstable generator.
struct SimpleGenerator<S, Y, R, F: FnMut(&mut S) -> State<Y, R>> {
    state: S,
    step: F,
    complete: Option<R>,
}

impl<S, Y, R, F: FnMut(&mut S) -> State<Y, R>> SimpleGenerator<S, Y, R, F> {
    fn new(step: F) -> SimpleGenerator<S, Y, R, F>
    where
        S: Default,
    {
        SimpleGenerator {
            state: S::default(),
            step,
            complete: None,
        }
    }

    fn next(&mut self) -> Option<Y> {
        match (self.step)(&mut self.state) {
            Yielded(y) => Some(y),
            Complete(r) => {
                self.complete = Some(r);
                None
            }
        }
    }

    fn complete(&self) -> Option<&R> {
        self.complete.as_ref()
    }
}

fn fibonacci() -> impl Iterator<Item = u32> {
    let mut curr = 0;
    let mut next = 1;
    std::iter::from_fn(move || {
        let old = curr;
        curr = next;
        next += old;
        Some(old)
    })
}
