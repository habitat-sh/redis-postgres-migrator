extern crate redis_postgres_migrator_redis_lib;
extern crate redis_postgres_migrator_postgres_lib;
extern crate habitat_builder_sessionsrv;
extern crate redis_extraction;
extern crate regex;

use redis_postgres_migrator_redis_lib as redis_lib;
use redis_postgres_migrator_postgres_lib as postgres_lib;
use habitat_builder_sessionsrv as session_srv;
use regex::Regex;

fn main() {
	println!("bite me");
}

pub fn redis_to_postgres(data_store :session_srv::data_store::DataStore) {
	let accounts = redis_extraction::extract_accounts();

	let re = Regex::new(r":(\d+)").unwrap();
	for x in accounts {
		for cap in re.captures_iter(&x) {
			let ds = data_store.clone();
      let account_id = &cap[1];
      let account = redis_lib::find_account_by_id(account_id.to_string());
println!("here is the account {:?}", account);

//			redis_to_postgres_account(ds, account_id.to_be());
		}
	}
}

pub fn redis_to_postgres_account(data_store :session_srv::data_store::DataStore, id: u64, email: String, name: String ) {
	let redis_account = redis_lib::find_account(id, &email, &name);
	let config = session_srv::config::Config::default();

	let session = postgres_lib::create_session(
			"bite me".to_string(),
			redis_account.get_id(),
			redis_account.get_email().to_string(),
			redis_account.get_name().to_string()
			);

	let account = postgres_lib::create_account(data_store, session);
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

		let found_account = redis_lib::find_account(account.get_id(), account.get_email(), account.get_name());

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

#[test]
	fn test_redis_to_postgres_account1() {
		// Create account in redis
		let session = redis_lib::create_session(
				String::from("scopuli"),
				64,
				String::from("julie.mao@chef.io"),
				String::from("Julie Mao"));


		let redis_account = redis_lib::create_account(session);

		// Set up postgres datastore
		let ds = postgres_lib::create_test_data_store();
		let ds1 = ds.clone();
		let ds2 = ds.clone();
		let ds3 = ds.clone();

		// Check that account does not exist in postgres
		let not_found = postgres_lib::get_account(ds1, redis_account.get_name());
		assert_eq!(not_found, None);

		redis_to_postgres_account(ds2, redis_account.get_id(), redis_account.get_email().to_string(), redis_account.get_name().to_string());

		// check that account is now in postgres
		let postgres_account = postgres_lib::get_account(ds3, redis_account.get_name()).unwrap();

			assert_eq!(redis_account.get_name(), postgres_account.get_name());
    }

    #[test]
    fn test_redis_to_postgres_accounts() {
         // Create account in redis
         let session = redis_lib::create_session(
                          String::from("scopuli"),
                          64,
                          String::from("julie.mao@chef.io"),
                          String::from("Julie Mao"));


  			 let redis_account = redis_lib::create_account(session);

         // Set up postgres datastore
				 let ds = postgres_lib::create_test_data_store();
				 let ds1 = ds.clone();
				 let ds2 = ds.clone();
				 let ds3 = ds.clone();

         // Check that account does not exist in postgres
         let not_found = postgres_lib::get_account(ds1, redis_account.get_name());
         assert_eq!(not_found, None);


        // transfer account to postgres
        redis_to_postgres(ds2);

        // check that account is now in postgres
        let postgres_account = postgres_lib::get_account(ds3, redis_account.get_name()).unwrap();
        assert_eq!(redis_account.get_name(), postgres_account.get_name());
    }

}
