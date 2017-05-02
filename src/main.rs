extern crate redis_postgres_migrator;
extern crate habitat_builder_sessionsrv as hab_sessionsrv;
extern crate habitat_builder_protocol as hab_protocol;
extern crate habitat_builder_dbcache as hab_dbcache;
extern crate habitat_net as hab_net;

extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;

use std::default::Default;
use std::thread;

use r2d2_redis::RedisConnectionManager;

use redis::Commands;

fn main() {
    println!("bite me");
}

pub fn redis_to_postgres(thing: String) {
// redis_postgres_migrator::builder-sessionsrv::data_store::
    println!("{}", thing);
}

#[cfg(test)]
mod tests {
//    use super::*;
    use std::sync::Arc;
		use hab_protocol::sessionsrv as proto_session;
    use hab_sessionsrv::data_store as sessionsrv_data_store;
    use hab_dbcache::data_store as dbcache_data_store;
    use hab_sessionsrv::config as session_srv_config;
    use hab_net::routing::{Broker, BrokerConn};

		extern crate r2d2;
		extern crate r2d2_redis;
		extern crate redis;

		use std::default::Default;
		use std::thread;

		use r2d2_redis::RedisConnectionManager;

		use redis::Commands;

//    fn test_creating_data() {

//    }

#[test]
fn create_account() {
    let mut sc = proto_session::SessionCreate::new();
    sc.set_token(String::from("hail2theking"));
    sc.set_extern_id(64);
    sc.set_email(String::from("bobo@chef.io"));
    sc.set_name(String::from("Bobo T. Clown"));
    sc.set_provider(proto_session::OAuthProvider::GitHub);

    let config = Default::default();
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = Arc::new(r2d2::Pool::new(config, manager).unwrap());

		let pool1 = pool.clone();

		let account_table = sessionsrv_data_store::AccountTable::new(pool1);
		let account = sessionsrv_data_store::AccountTable::find_or_create(&account_table, &sc);

    println!("{:?}", account);
}


//    fn test_data_transfer() {
//        redis_to_postgres("Bite me!".to_string())
//   }
}
