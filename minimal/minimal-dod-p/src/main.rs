use std::time::Instant;

fn main() {
    let mut ids: Vec<u32> = (0..10_000).collect();
    let mut balances: Vec<f64> = vec![100.0; 10_000];

    let start = Instant::now();

    for i in 0..balances.len() {
        balances[i] += 1.0;
    }

    println!("DoD pattern took {:?}", start.elapsed());

    // Just to avoid optimizer throwing unused variables away
    println!("First id={}, balance={}", ids[0], balances[0]);
}
