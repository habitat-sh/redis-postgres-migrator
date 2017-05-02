extern crate redis_postgres_migrator_redis_lib;
use redis_postgres_migrator_redis_lib as redis_lib;

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
    fn test_redis_account_create() {
        let session = redis_lib::create_session(
                          String::from("hail2theking"),
                          64,
                          String::from("bobo@chef.io"),
                          String::from("Bobo T. Clown"));

       let account = redis_lib::create_account(session);

       let found_account = redis_lib::find_account(account.get_name());

       assert_eq!(account.get_id(), found_account.get_id());
    }

    #[test]
    fn test_postgres_account_create() {
//        let session = postgres_data_store::create_session(
//            String::from("hail2theking"),
//            64,
//            String::from("bobo@chef.io"),
//            String::from("Bobo T. Clown"),
//        );

        //let account = redis_postgres_migrator::postgres_data_store::create_account()

    }
}
