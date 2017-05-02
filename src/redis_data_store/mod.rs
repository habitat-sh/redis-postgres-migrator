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

#[cfg(test)]
mod tests {
//    use super::*;
    use std::sync::Arc;
		use redis_data_store::protocol::sessionsrv as proto_session;
    use redis_data_store::hab_sessionsrv::data_store as sessionsrv_data_store;
    use redis_data_store::dbcache::data_store as dbcache_data_store;
    use redis_data_store::hab_sessionsrv::config as session_srv_config;
    use redis_data_store::hab_net::routing::{Broker, BrokerConn};

		extern crate r2d2;
		extern crate r2d2_redis;
		extern crate redis;

		use std::default::Default;
		use std::thread;

		use self::r2d2_redis::RedisConnectionManager;

		use self::redis::Commands;

    fn create_account() -> proto_session::Account {
				let mut sc = proto_session::SessionCreate::new();
				sc.set_token(String::from("hail2theking"));
				sc.set_extern_id(64);
				sc.set_email(String::from("bobo@chef.io"));
				sc.set_name(String::from("Bobo T. Clown"));
				sc.set_provider(proto_session::OAuthProvider::GitHub);

				let config = Default::default();
				let manager = RedisConnectionManager::new("redis://localhost").unwrap();
				let pool = Arc::new(r2d2::Pool::new(config, manager).unwrap());

				let account_table = sessionsrv_data_store::AccountTable::new(pool);
				let account = sessionsrv_data_store::AccountTable::find_or_create(&account_table, &sc).unwrap();
        account

    }

    #[test]
    fn test_data_transfer() {
        let account = create_account();

				let config = Default::default();
				let manager = RedisConnectionManager::new("redis://localhost").unwrap();
				let pool = Arc::new(r2d2::Pool::new(config, manager).unwrap());

				let account_table = sessionsrv_data_store::AccountTable::new(pool);
        let found_account = sessionsrv_data_store::AccountTable::find_by_username(&account_table, account.get_name()).unwrap();

        assert_eq!(account.get_id(), found_account.get_id());
    }
}
