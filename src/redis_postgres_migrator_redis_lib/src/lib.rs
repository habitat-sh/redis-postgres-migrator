extern crate habitat_builder_sessionsrv as hab_sessionsrv;
extern crate habitat_builder_protocol as protocol;
extern crate habitat_builder_dbcache as dbcache;
extern crate r2d2;
extern crate r2d2_redis;

use std::ops::Deref;
use std::sync::Arc;
use dbcache::InstaSet;
use hab_sessionsrv::data_store::{DataStore, AccountTable};

use self::r2d2_redis::RedisConnectionManager;
use std::str::FromStr;

pub fn create_session(token: String,
                      extern_id: u64,
                      email: String,
                      name: String)
                      -> protocol::sessionsrv::SessionCreate {
    let mut sc = protocol::sessionsrv::SessionCreate::new();
    sc.set_token(token);
    sc.set_extern_id(extern_id);
    sc.set_email(email);
    sc.set_name(String::from(name));
    sc.set_provider(protocol::sessionsrv::OAuthProvider::GitHub);
    sc
}

pub fn create_account(redis_addr: &str,
                      session: protocol::sessionsrv::SessionCreate)
                      -> protocol::sessionsrv::Account {

    let pool = create_pool(redis_addr);
    let account_table = hab_sessionsrv::data_store::AccountTable::new(pool);

    let mut account = protocol::sessionsrv::Account::new();
    account.set_email(session.get_email().to_string());
    account.set_name(session.get_name().to_string());
    account_table.write(&mut account);

    account
}

pub fn find_account_by_id(redis_addr: &str, id: String) -> protocol::sessionsrv::Account {
    let pool = create_pool(redis_addr);
    let ds = DataStore::new(pool);

    let value = account_value(id);
    let account = ds.accounts.find(&value).unwrap();

    account
}

fn create_pool(redis_addr: &str) -> std::sync::Arc<r2d2::Pool<r2d2_redis::RedisConnectionManager>> {
    let config = Default::default();
    let manager = RedisConnectionManager::new(redis_addr).unwrap();
    let mut pool = Arc::new(r2d2::Pool::new(config, manager).unwrap());
    pool
}

fn account_value(id: String) -> u64 {
    let account_search_key = protocol::sessionsrv::AccountSearchKey::Id;
    let mut account_search = protocol::sessionsrv::AccountSearch::new();

    account_search.set_key(account_search_key);
    account_search.set_value(id.clone());

    let value: u64 = account_search.take_value().parse().unwrap();
    value
}
