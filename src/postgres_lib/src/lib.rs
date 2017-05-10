extern crate habitat_builder_sessionsrv as hab_sessionsrv;
extern crate habitat_builder_protocol as protocol;
extern crate habitat_core as hab_core;
extern crate habitat_net as hab_net;
#[macro_use]
extern crate habitat_builder_db as hab_db;
use hab_sessionsrv::data_store::DataStore as sessionsrv_data_store;
use protocol::sessionsrv::Session;
use std::ops::Deref;
extern crate num_cpus;
extern crate postgres;
use std::net::{Ipv4Addr, IpAddr};
use postgres::params::{ConnectParams, Host, IntoConnectParams};

extern crate r2d2;
extern crate r2d2_postgres;

use std::thread;
use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use std::time::Duration;

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


pub fn create_account(data_store: sessionsrv_data_store,
                      session: protocol::sessionsrv::SessionCreate)
                      -> protocol::sessionsrv::Session {
    let config = hab_sessionsrv::config::Config::default();
    let account_creation =
        data_store.find_or_create_account_via_session(&session, false, false, false);
    let account = account_creation.unwrap();
    account
}

pub fn get_account(data_store: sessionsrv_data_store,
                   account_name: &str)
                   -> std::option::Option<protocol::sessionsrv::Account> {
    let mut ag = protocol::sessionsrv::AccountGet::new();
    ag.set_name(account_name.to_string());
    let account_get = data_store.get_account(&ag);
    let account = account_get.unwrap();
    account
}

pub fn create_test_data_store() -> sessionsrv_data_store {
    let ds = datastore_test!(sessionsrv_data_store);
    ds
}

pub fn create_real_data_store() {
    let config = hab_db::config::DataStoreCfg {
        host: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 5432,
            user: String::from("hab"),
            password: None,
            database: String::from("builder-sessionsrv"),
            connection_retry_ms: 300,
            connection_timeout_sec: 3600,
            connection_test: false,
						pool_size: (num_cpus::get() * 2) as u32
    };

    let mut shards: Vec<protocol::sharding::ShardId> = (1..128).collect();


    let config = r2d2::Config::default();
//    let pool_config_builder = r2d2::Config::builder()
//        .pool_size(config.pool_size)
//        .connection_timeout(Duration::from_secs(config.connection_timeout_sec));

//    let builder_session_srv_pool = Pool::new(
//        &builder_session_srv_db_config,
//        shards
//    );

//println!("{}", builder_session_srv_db_config);
//    let builder_session_srv_data_store = session_srv::data_store::DataStore::new(&builder_session_srv_config);
//println!("two");
//    println!("{:?}", builder_session_srv_data_store);


}
