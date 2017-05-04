extern crate habitat_builder_sessionsrv as hab_sessionsrv;
extern crate habitat_builder_protocol as protocol;
extern crate habitat_builder_dbcache as dbcache;
extern crate r2d2;
extern crate r2d2_redis;

use std::ops::Deref;
use std::sync::Arc;
use dbcache::InstaSet;
use hab_sessionsrv::data_store::{AccountTable};

use self::r2d2_redis::RedisConnectionManager;

pub fn create_session(token: String, extern_id: u64, email: String, name: String) -> protocol::sessionsrv::SessionCreate {
    let mut sc = protocol::sessionsrv::SessionCreate::new();
    sc.set_token(token);
    sc.set_extern_id(extern_id);
    sc.set_email(email);
    sc.set_name(String::from(name));
    sc.set_provider(protocol::sessionsrv::OAuthProvider::GitHub);
    sc
}

pub fn create_account(session: protocol::sessionsrv::SessionCreate) -> protocol::sessionsrv::Account {

    let config = Default::default();
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = Arc::new(r2d2::Pool::new(config, manager).unwrap());

	  let account_table = hab_sessionsrv::data_store::AccountTable::new(pool);

    let mut account = protocol::sessionsrv::Account::new();
    account.set_email(session.get_email().to_string());
    account.set_name(session.get_name().to_string());
    account_table.write(&mut account);

    account
}

pub fn find_account(id: u64, email: &str, name: &str) -> protocol::sessionsrv::Account {
    let config = Default::default();
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = Arc::new(r2d2::Pool::new(config, manager).unwrap());
	  let account_table = hab_sessionsrv::data_store::AccountTable::new(pool);

    let mut sc = protocol::sessionsrv::SessionCreate::new();
    sc.set_email(email.to_string());
    sc.set_name(name.to_string());
    sc.set_extern_id(id);
    let account = hab_sessionsrv::data_store::AccountTable::find_or_create(&account_table, &sc).unwrap();
    account
}
