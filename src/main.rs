extern crate redis_postgres_migrator;

fn main() {
// redis_postgres_migrator::builder-sessionsrv::data_store::
    println!("bite me");
}

pub fn redis_to_postgres(thing: String) {
    println!("{}", thing);
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_data_transfer() {
        redis_to_postgres("Bite me!".to_string())
    }
}
