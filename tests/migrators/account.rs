use postgres_lib;
use redis_lib;

use redis_postgres_migrator::migrators;

fn test_redis_addr() -> &'static str {
    "redis://127.0.0.1/"
}

#[test]
fn test_redis_account_create() {
    let session = redis_lib::create_session(String::from("hail2theking"),
                                            64,
                                            String::from("bobo@chef.io"),
                                            String::from("Bobo T. Clown"));

    let account = redis_lib::create_account(test_redis_addr(), session);

    let found_account = redis_lib::find_account_by_id(test_redis_addr(),
                                                        account.get_id().to_string());

    assert_eq!(account.get_id(), found_account.get_id());
}

#[test]
fn test_postgres_account_create() {
    let ds = postgres_lib::create_test_sessionsrv_data_store();
    let ds1 = ds.clone();
    let ds2 = ds.clone();

    let session = postgres_lib::create_session(String::from("hail2theking"),
                                                64,
                                                String::from("bobo@chef.io"),
                                                String::from("Bobo T. Clown"));

    let account = postgres_lib::create_account(ds1, session);
    let found_account = postgres_lib::get_account(ds2, account.get_name()).unwrap();

    assert_eq!(account.get_id(), found_account.get_id());
}

#[test]
fn test_redis_to_postgres_account1() {
    // Create account in redis
    let session = redis_lib::create_session(String::from("scopuli"),
                                            64,
                                            String::from("julie.mao@chef.io"),
                                            String::from("Julie Mao"));


    let redis_account = redis_lib::create_account(test_redis_addr(), session);

    // Set up postgres datastore
    let ds = postgres_lib::create_test_sessionsrv_data_store();
    let ds1 = ds.clone();
    let ds2 = ds.clone();
    let ds3 = ds.clone();

    // Check that account does not exist in postgres
    let not_found = postgres_lib::get_account(ds1, redis_account.get_name());
    assert_eq!(not_found, None);

    migrators::account::redis_to_postgres_account(test_redis_addr(),
                                ds2,
                                redis_account.get_id());

    // check that account is now in postgres
    let postgres_account = postgres_lib::get_account(ds3, redis_account.get_name()).unwrap();

    assert_eq!(redis_account.get_name(), postgres_account.get_name());
}

#[test]
fn test_redis_to_postgres_accounts() {
    let redis_addr = "redis://127.0.0.1/";
    // Create account in redis
    let session = redis_lib::create_session(String::from("scopuli"),
                                            64,
                                            String::from("julie.mao@chef.io"),
                                            String::from("Julie Mao"));


    let redis_account = redis_lib::create_account(redis_addr, session);

    let session2 = redis_lib::create_session(String::from("canterbury"),
                                                64,
                                                String::from("james.holden@chef.io"),
                                                String::from("James Holden"));

    let redis_account2 = redis_lib::create_account(redis_addr, session2);

    let session3 = redis_lib::create_session(String::from("canterbury"),
                                                64,
                                                String::from("james.holden@chef.io"),
                                                String::from("James Holden"));

    let redis_account3 = redis_lib::create_account(redis_addr, session3);

    // Set up postgres datastore
    let ds = postgres_lib::create_test_sessionsrv_data_store();

    // Check that account does not exist in postgres
    let not_found1 = postgres_lib::get_account(ds.clone(), redis_account.get_name());
    let not_found2 = postgres_lib::get_account(ds.clone(), redis_account2.get_name());

    assert_eq!(not_found1, None);
    assert_eq!(not_found2, None);

    // transfer account to postgres
    migrators::account::redis_to_postgres(redis_addr, ds.clone());

    // check that accounts are now in postgres
    let postgres_account = postgres_lib::get_account(ds.clone(), redis_account.get_name())
        .unwrap();
    assert_eq!(redis_account.get_name(), postgres_account.get_name());

    let postgres_account2 = postgres_lib::get_account(ds.clone(), redis_account2.get_name())
        .unwrap();
    assert_eq!(redis_account2.get_name(), postgres_account2.get_name());
}
