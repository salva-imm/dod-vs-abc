#![feature(portable_simd)]
use std::simd::*;
use std::time::Instant;

fn main() {
    let mut balances: Vec<f64> = vec![100.0; 10_000];

    let start = Instant::now();

    let mut chunks = balances.chunks_exact_mut(8);

    for chunk in &mut chunks {
        let mut vec = f64x8::from_slice(chunk);
        vec += f64x8::splat(1.0);
        vec.copy_to_slice(chunk);
    }

    // Step 3: handle leftover elements
    let remainder = chunks.into_remainder(); // now safe, after iteration
    for r in remainder {
        *r += 1.0;
    }

    println!("SIMD DoD pattern took {:?}", start.elapsed());
}
