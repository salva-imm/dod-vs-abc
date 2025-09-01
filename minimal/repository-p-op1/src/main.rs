use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug)]
struct User {
    id: u32,
    balance: f64,
}

trait UserRepository {
    fn get_all(&self) -> Vec<&User>; // Changed return type
    fn update_balance(&mut self, id: u32, delta: f64);
}

struct InMemoryUserRepository {
    users: HashMap<u32, User>,
}

impl UserRepository for InMemoryUserRepository {
    fn get_all(&self) -> Vec<&User> {
        self.users.values().collect()
    }

    fn update_balance(&mut self, id: u32, delta: f64) {
        if let Some(user) = self.users.get_mut(&id) {
            user.balance += delta;
        }
    }
}

fn main() {
    let mut repo = InMemoryUserRepository {
        users: (0..10_000)
            .map(|i| (i, User { id: i, balance: 100.0 }))
            .collect(),
    };

    let start = Instant::now();

    for i in 0..10_000 {
        repo.update_balance(i, 1.0);
    }

    println!("HashMap repository took {:?}", start.elapsed());
}
