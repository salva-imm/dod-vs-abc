use std::time::Instant;
use rand::prelude::*;

struct UsersView<'a> {
    ids: &'a [i32],
    balances: &'a [f32],
    active: &'a [u8],
    count: usize,
}

#[inline(never)]
fn sum_active_balances(users_view: &UsersView, minimum_balance: f32) -> f32 {
    let mut accumulated_balance = 0.0f32;
    let threshold_balance = minimum_balance;

    for i in 0..users_view.count {
        let balance_value = users_view.balances[i];
        let take_value = if users_view.active[i] != 0 && balance_value >= threshold_balance {
            1.0f32
        } else {
            0.0f32
        };
        accumulated_balance += balance_value * take_value;
    }

    accumulated_balance
}

fn measure_execution_time<F, R>(iterations: usize, mut f: F) -> f64
where
    F: FnMut() -> R,
{
    let start = Instant::now();

    for _ in 0..iterations {
        let _ = f();
    }

    start.elapsed().as_secs_f64()
}

fn main() {
    const ELEMENTS_COUNT: usize = 10_000;
    const MINIMUM_BALANCE: f32 = 250.0;
    const RANDOM_SEED: u64 = 17;
    const WARMUP_ITERATIONS: usize = 2;
    const ITERATIONS: usize = 8;

    println!();
    println!("[ DoD Benchmark ]");
    println!("Elements Count    : {}", ELEMENTS_COUNT);
    println!("Minimum Balance   : {:.2}", MINIMUM_BALANCE);
    println!("Random Seed       : {}", RANDOM_SEED);
    println!("Warmup Iterations : {}", WARMUP_ITERATIONS);
    println!("Iterations        : {}", ITERATIONS);

    let mut rng = StdRng::seed_from_u64(RANDOM_SEED);
    let balance_dist = rand::distributions::Uniform::new(0.0f32, 1000.0f32);
    let active_dist = rand::distributions::Bernoulli::new(0.6).unwrap();

    println!();
    println!("Generating elements...");

    let mut user_ids = Vec::with_capacity(ELEMENTS_COUNT);
    let mut user_balances = Vec::with_capacity(ELEMENTS_COUNT);
    let mut user_active_flags = Vec::with_capacity(ELEMENTS_COUNT);

    for i in 0..ELEMENTS_COUNT {
        user_ids.push(i as i32);
        user_balances.push(rng.sample(balance_dist));
        user_active_flags.push(if rng.sample(active_dist) { 1u8 } else { 0u8 });
    }

    let users_view = UsersView {
        ids: &user_ids,
        balances: &user_balances,
        active: &user_active_flags,
        count: ELEMENTS_COUNT,
    };

    println!();
    println!("Warming up...");

    let mut checksum = 0.0f32;
    for _ in 0..WARMUP_ITERATIONS {
        checksum = sum_active_balances(&users_view, MINIMUM_BALANCE);
    }

    println!();
    println!("Benchmarking...");

    let total_time_seconds = measure_execution_time(ITERATIONS, || {
        sum_active_balances(&users_view, MINIMUM_BALANCE)
    });

    let average_time_seconds = total_time_seconds / ITERATIONS as f64;
    let elements_per_second = ELEMENTS_COUNT as f64 / average_time_seconds;
    let nanoseconds_per_element = (average_time_seconds * 1e9) / ELEMENTS_COUNT as f64;

    println!();
    println!("[ DoD Results ]");
    println!("Checksum                   : {:.8}", checksum);
    println!("Total Time                 : {:.2} s", total_time_seconds);
    println!("Average Time per Iteration : {:.2} s", average_time_seconds);
    println!("Elements per Second        : {:.2} M", elements_per_second / 1e6);
    println!("Nanoseconds per Element    : {:.2}", nanoseconds_per_element);
    println!();
}
