extern crate redis_postgres_migrator_redis_lib;
extern crate redis_postgres_migrator_postgres_lib;

use redis_postgres_migrator_redis_lib as redis_lib;
use redis_postgres_migrator_postgres_lib as postgres_lib;

fn main() {
    println!("bite me");
}

pub fn redis_to_postgres(thing: String) {
    println!("{}", thing);
}

#[cfg(test)]
mod tests {
    use super::*;
    //use redis_lib::hab_sessionsrv::data_store::DataStore as sessionsrv_data_store;


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
        let ds = postgres_lib::create_test_data_store();
        let ds1 = ds.clone();
        let ds2 = ds.clone();

        let session = postgres_lib::create_session(
            String::from("hail2theking"),
            64,
            String::from("bobo@chef.io"),
            String::from("Bobo T. Clown"),
        );

        let account = postgres_lib::create_account(ds1, session);
        let found_account = postgres_lib::get_account(ds2, account.get_name()).unwrap();

        assert_eq!(account.get_id(), found_account.get_id());
    }
}
