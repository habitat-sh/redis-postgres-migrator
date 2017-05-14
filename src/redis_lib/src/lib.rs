extern crate habitat_builder_sessionsrv_redis as hab_sessionsrv;
extern crate habitat_builder_protocol_redis as protocol;
extern crate habitat_builder_dbcache_redis as dbcache;
extern crate habitat_builder_vault as vault;
extern crate r2d2;
extern crate r2d2_redis;

use std::ops::Deref;
use std::sync::Arc;
use dbcache::data_store::Pool;
use dbcache::InstaSet;
use vault::data_store::DataStore as vault_datastore;
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

pub fn create_origin(redis_addr: &str, name: &str, owner_id: u64) -> protocol::vault::Origin {
    let mut origin = protocol::vault::Origin::new();
    origin.set_owner_id(owner_id);
    origin.set_name(name.to_string());
    let datastore = vault_datastore::init(create_pool(redis_addr));
    datastore.origins.write(&mut origin);
    origin
}

pub fn find_account_by_id(redis_addr: &str, id: String) -> protocol::sessionsrv::Account {
    let pool = create_pool(redis_addr);
    let ds = DataStore::new(pool);

    let value = account_value(id);
    let account = ds.accounts.find(&value).unwrap();

    account
}

pub fn find_origin_by_id(redis_addr: &str, id: u64) -> protocol::vault::Origin {
    let ds = vault_datastore::init(create_pool(redis_addr));
    ds.origins.find(&id).unwrap()
}

pub fn create_pool(redis_addr: &str)
                   -> std::sync::Arc<r2d2::Pool<r2d2_redis::RedisConnectionManager>> {
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
