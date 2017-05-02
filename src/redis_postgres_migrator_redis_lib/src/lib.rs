extern crate habitat_builder_sessionsrv as hab_sessionsrv;
extern crate habitat_builder_protocol as protocol;
extern crate r2d2;
extern crate r2d2_redis;

use std::ops::Deref;
use std::sync::Arc;

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
    let account = hab_sessionsrv::data_store::AccountTable::find_or_create(&account_table, &session).unwrap();
    account
}

pub fn find_account(user_name: &str) -> protocol::sessionsrv::Account {
    let config = Default::default();
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = Arc::new(r2d2::Pool::new(config, manager).unwrap());
	  let account_table = hab_sessionsrv::data_store::AccountTable::new(pool);
    let account = hab_sessionsrv::data_store::AccountTable::find_by_username(&account_table, user_name).unwrap();
    account
}
