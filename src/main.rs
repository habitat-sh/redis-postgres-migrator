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
        let session = redis_postgres_migrator::redis_data_store::create_session(
                          String::from("hail2theking"),
                          64,
                          String::from("bobo@chef.io"),
                          String::from("Bobo T. Clown"));

       let account = redis_postgres_migrator::redis_data_store::create_account(session);

       let found_account = redis_postgres_migrator::redis_data_store::find_account(account.get_name());

       assert_eq!(account.get_id(), found_account.get_id());
    }
}
