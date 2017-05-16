use redis_lib;
use postgres_lib;
use habitat_builder_sessionsrv as session_srv;
use redis_extraction;
use regex::Regex;

pub fn redis_to_postgres(redis_addr: &str, data_store: session_srv::data_store::DataStore) {
    let accounts = redis_extraction::extract_accounts(redis_addr);

    let re = Regex::new(r":(\d+)").unwrap();
    for x in accounts {
        for cap in re.captures_iter(&x) {
            let ds = data_store.clone();
            let account_id_string = &cap[1];
            let account_id = account_id_string.parse::<u64>();

            println!("{:?}", cap);
            redis_to_postgres_account(redis_addr, ds, account_id.unwrap())
        }
    }
}

pub fn redis_to_postgres_account(redis_addr: &str,
                                 data_store: session_srv::data_store::DataStore,
                                 id: u64) {
    let redis_account = redis_lib::find_account_by_id(redis_addr, id.to_string());
    if postgres_lib::get_account(data_store.clone(), redis_account.get_name()).is_some() {
        return;
    }
    println!("Migrating account {}", redis_account.get_name());
    let config = session_srv::config::Config::default();

    let session = postgres_lib::create_session("pretend_session".to_string(),
                                               redis_account.get_id(),
                                               redis_account.get_email().to_string(),
                                               redis_account.get_name().to_string());

    let account = postgres_lib::create_account(data_store, session);
}
