use std::time::Instant;
use rand::prelude::*;

#[derive(Debug, Clone)]
struct User {
    id: i32,
    balance: f32,
    active: bool,
}

/// PURE Repository - Only data access concerns
trait UserRepository {
    /// Find by ID
    fn find_by_id(&self, id: i32) -> Option<&User>;

    /// Get all users (iterator for memory efficiency)
    fn find_all(&self) -> std::slice::Iter<User>;

    /// Count total users
    fn count(&self) -> usize;
}

/// PURE Repository Implementation - No business logic
struct VectorUserRepository {
    users: Vec<User>,
}

impl VectorUserRepository {
    fn new(users: Vec<User>) -> Self {
        Self { users }
    }
}

impl UserRepository for VectorUserRepository {
    fn find_by_id(&self, id: i32) -> Option<&User> {
        self.users.iter().find(|user| user.id == id)
    }

    fn find_all(&self) -> std::slice::Iter<User> {
        self.users.iter()
    }

    fn count(&self) -> usize {
        self.users.len()
    }
}

/// DOMAIN SERVICE - Contains business logic
struct UserService<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> UserService<R> {
    fn new(repository: R) -> Self {
        Self { repository }
    }

    /// PROPER: Business logic in service layer
    fn sum_active_balances(&self, minimum_balance: f32) -> f32 {
        self.repository
            .find_all()
            .filter(|user| self.qualifies_for_sum(user, minimum_balance))
            .map(|user| user.balance)
            .sum()
    }

    /// PROPER: Business rules encapsulated in domain service
    fn qualifies_for_sum(&self, user: &User, minimum_balance: f32) -> bool {
        user.active && user.balance >= minimum_balance
    }

    /// Additional business operations
    fn get_high_value_users(&self, minimum_balance: f32) -> Vec<&User> {
        self.repository
            .find_all()
            .filter(|user| self.qualifies_for_sum(user, minimum_balance))
            .collect()
    }
}

/// APPLICATION LAYER - Orchestrates the flow
#[inline(never)]
fn sum_active_balances<R: UserRepository>(service: &UserService<R>, minimum_balance: f32) -> f32 {
    service.sum_active_balances(minimum_balance)
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
    println!("[ Clean Architecture Repository Benchmark ]");
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

    // Clean Architecture Layers
    let repository = VectorUserRepository::new(users);
    let service = UserService::new(repository);

    println!();
    println!("Warming up...");

    let mut checksum = 0.0f32;
    for _ in 0..WARMUP_ITERATIONS {
        checksum = sum_active_balances(&service, MINIMUM_BALANCE);
    }

    println!();
    println!("Benchmarking...");

    let total_time_seconds = measure_execution_time(ITERATIONS, || {
        sum_active_balances(&service, MINIMUM_BALANCE)
    });

    let average_time_seconds = total_time_seconds / ITERATIONS as f64;
    let elements_per_second = ELEMENTS_COUNT as f64 / average_time_seconds;
    let nanoseconds_per_element = (average_time_seconds * 1e9) / ELEMENTS_COUNT as f64;

    println!();
    println!("[ Clean Architecture Results ]");
    println!("Checksum                   : {:.8}", checksum);
    println!("Total Time                 : {:.2} s", total_time_seconds);
    println!("Average Time per Iteration : {:.2} s", average_time_seconds);
    println!("Elements per Second        : {:.2} M", elements_per_second / 1e6);
    println!("Nanoseconds per Element    : {:.2}", nanoseconds_per_element);
    println!();
}
