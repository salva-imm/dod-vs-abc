use std::time::Instant;

#[derive(Debug)]
struct User {
    id: u32,
    balance: f64,
}

trait UserRepository {
    fn get_all(&self) -> Vec<User>; // Return owned users
    fn update_balance(&mut self, id: u32, delta: f64);
    fn get_user(&self, id: u32) -> Option<User>;
}

struct InMemoryUserRepository {
    ids: Vec<u32>,
    balances: Vec<f64>,
}

impl InMemoryUserRepository {
    fn new(users: Vec<User>) -> Self {
        let mut ids = Vec::with_capacity(users.len());
        let mut balances = Vec::with_capacity(users.len());

        for user in users {
            ids.push(user.id);
            balances.push(user.balance);
        }

        Self { ids, balances }
    }
}

impl UserRepository for InMemoryUserRepository {
    fn get_all(&self) -> Vec<User> {
        self.ids
            .iter()
            .zip(self.balances.iter())
            .map(|(&id, &balance)| User { id, balance })
            .collect()
    }

    fn update_balance(&mut self, id: u32, delta: f64) {
        // Assuming sequential IDs for O(1) access
        if let Some(balance) = self.balances.get_mut(id as usize) {
            *balance += delta;
        }
    }

    fn get_user(&self, id: u32) -> Option<User> {
        if let Some(&balance) = self.balances.get(id as usize) {
            Some(User { id, balance })
        } else {
            None
        }
    }
}

fn main() {
    let users: Vec<User> = (0..10_000)
        .map(|i| User { id: i, balance: 100.0 })
        .collect();

    let mut repo = InMemoryUserRepository::new(users);

    let start = Instant::now();

    for i in 0..10_000 {
        repo.update_balance(i, 1.0);
    }

    println!("DoD repository took {:?}", start.elapsed());

    // Verify it works
    let user = repo.get_user(0).unwrap();
    println!("First user: id={}, balance={}", user.id, user.balance);
}
