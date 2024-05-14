use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();
    let sum = a_loop();
    let duration = start.elapsed();

    println!("Time elapsed is {:?}", duration);
    println!("Sum is {}", sum);
}

fn a_loop() -> u64 {
    let mut sum = 0;
    for _ in 0..20_0000_0000 {
        sum += 1
    }
    sum
}
