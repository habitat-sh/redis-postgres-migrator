extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;
extern crate habitat_builder_dbcache as dbcache;
extern crate habitat_builder_protocol as protocol;
extern crate habitat_core as hab_core;
extern crate habitat_net as hab_net;
extern crate habitat_builder_sessionsrv as hab_sessionsrv;
extern crate hyper;
extern crate log;
extern crate protobuf;
extern crate rustc_serialize;
extern crate time;
extern crate toml;
extern crate zmq;

use std::ops::Deref;
use std::sync::Arc;

use self::r2d2_redis::RedisConnectionManager;

fn create_session(token: String, extern_id: i64, email: String, name: String) -> protocol::sessionsrv::SessionCreate {
    let mut sc = protocol::sessionsrv::SessionCreate::new();
    sc.set_token(String::from("hail2theking"));
    sc.set_extern_id(64);
    sc.set_email(String::from("bobo@chef.io"));
    sc.set_name(String::from("Bobo T. Clown"));
    sc.set_provider(protocol::sessionsrv::OAuthProvider::GitHub);
    sc
}

fn create_account(session: protocol::sessionsrv::SessionCreate) -> protocol::sessionsrv::Account {
    let config = Default::default();
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = Arc::new(r2d2::Pool::new(config, manager).unwrap());

	  let account_table = hab_sessionsrv::data_store::AccountTable::new(pool);
    let account = hab_sessionsrv::data_store::AccountTable::find_or_create(&account_table, &session).unwrap();
    account
}

fn find_account(user_name: &str) -> protocol::sessionsrv::Account {
    let config = Default::default();
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = Arc::new(r2d2::Pool::new(config, manager).unwrap());
	  let account_table = hab_sessionsrv::data_store::AccountTable::new(pool);
    let account = hab_sessionsrv::data_store::AccountTable::find_by_username(&account_table, user_name).unwrap();
    account
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_transfer() {
        let session = create_session(
                      String::from("hail2theking"),
                      64,
                      String::from("bobo@chef.io"),
                      String::from("Bobo T. Clown"));


        let account = create_account(session);
        let found_account = find_account(account.get_name());

        assert_eq!(account.get_id(), found_account.get_id());
    }
}
