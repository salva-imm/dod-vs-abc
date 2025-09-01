use std::time::Instant;
use rand::prelude::*;

#[derive(Debug, Clone)]
struct User {
    id: i32,
    balance: f32,
    active: bool,
}

trait UserRepository {
    fn get_all(&self) -> &Vec<User>;
    fn find_by_id(&self, id: i32) -> Option<&User>;
    fn count(&self) -> usize;
}

struct VectorUserRepository {
    users: Vec<User>,
}

impl VectorUserRepository {
    fn new(users: Vec<User>) -> Self {
        Self { users }
    }
}

impl UserRepository for VectorUserRepository {
    fn get_all(&self) -> &Vec<User> {
        &self.users
    }

    fn find_by_id(&self, id: i32) -> Option<&User> {
        self.users.iter().find(|user| user.id == id)
    }

    fn count(&self) -> usize {
        self.users.len()
    }
}

fn qualifies(user: &User, minimum_balance: f32) -> bool {
    user.active && user.balance >= minimum_balance
}

#[inline(never)]
fn sum_active_balances(repository: &dyn UserRepository, minimum_balance: f32) -> f32 {
    let mut accumulated_balance = 0.0;

    for i in 0..repository.count() {
        if let Some(user) = repository.find_by_id(i as i32) {
            if qualifies(user, minimum_balance) {
                accumulated_balance += user.balance;
            }
        }
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
    println!("[ Repository Benchmark ]");
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

    let mut users = Vec::with_capacity(ELEMENTS_COUNT);
    for i in 0..ELEMENTS_COUNT {
        let user = User {
            id: i as i32,
            balance: rng.sample(balance_dist),
            active: rng.sample(active_dist),
        };
        users.push(user);
    }

    let repository = VectorUserRepository::new(users);

    println!();
    println!("Warming up...");

    let mut checksum = 0.0f32;
    for _ in 0..WARMUP_ITERATIONS {
        checksum = sum_active_balances(&repository, MINIMUM_BALANCE);
    }

    println!();
    println!("Benchmarking...");

    let total_time_seconds = measure_execution_time(ITERATIONS, || {
        sum_active_balances(&repository, MINIMUM_BALANCE)
    });

    let average_time_seconds = total_time_seconds / ITERATIONS as f64;
    let elements_per_second = ELEMENTS_COUNT as f64 / average_time_seconds;
    let nanoseconds_per_element = (average_time_seconds * 1e9) / ELEMENTS_COUNT as f64;

    println!();
    println!("[ Repository Results ]");
    println!("Checksum                   : {:.8}", checksum);
    println!("Total Time                 : {:.2} s", total_time_seconds);
    println!("Average Time per Iteration : {:.2} s", average_time_seconds);
    println!("Elements per Second        : {:.2} M", elements_per_second / 1e6);
    println!("Nanoseconds per Element    : {:.2}", nanoseconds_per_element);
    println!();
}
