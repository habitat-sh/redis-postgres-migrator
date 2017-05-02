extern crate redis_postgres_migrator;

fn main() {
    println!("bite me");
}

pub fn redis_to_postgres(thing: String) {
    println!("{}", thing);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redis_to_postgres() {
    }
}
