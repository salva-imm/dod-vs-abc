use std::time::Instant;

#[derive(Debug)]
struct User {
    id: u32,
    balance: f64,
}

trait UserRepository {
    fn get_all(&self) -> &Vec<User>;
    fn update_balance(&mut self, id: u32, delta: f64);
}

struct InMemoryUserRepository {
    users: Vec<User>,
}

impl UserRepository for InMemoryUserRepository {
    fn get_all(&self) -> &Vec<User> {
        &self.users
    }

    fn update_balance(&mut self, id: u32, delta: f64) {
        // Direct indexing - assumes sequential IDs starting from 0
        if let Some(user) = self.users.get_mut(id as usize) {
            user.balance += delta;
        }
    }
}

fn main() {
    let mut repo = InMemoryUserRepository {
        users: (0..10_000)
            .map(|i| User { id: i, balance: 100.0 })
            .collect(),
    };

    let start = Instant::now();

    for i in 0..10_000 {
        repo.update_balance(i, 1.0);
    }

    println!("Direct indexing repository took {:?}", start.elapsed());
}
