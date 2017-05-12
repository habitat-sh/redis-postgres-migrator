extern crate habitat_builder_sessionsrv as hab_sessionsrv;
extern crate habitat_builder_protocol as protocol;
extern crate habitat_core as hab_core;
extern crate habitat_net as hab_net;
extern crate habitat_builder_originsrv as hab_originsrv;

#[macro_use]
extern crate habitat_builder_db as hab_db;

use hab_sessionsrv::data_store::DataStore as sessionsrv_data_store;
use hab_originsrv::data_store::DataStore as originsrv_data_store;

use protocol::sessionsrv::Session;
use hab_db::pool::Pool;
use std::ops::Deref;

extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate num_cpus;
use std::net::{Ipv4Addr, IpAddr};
use std::time::Duration;
use std::error;
use std::thread;

use postgres::Connection;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use postgres::params::IntoConnectParams;
use r2d2::ManageConnection;

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

pub fn create_sessionsrv_data_store() -> sessionsrv_data_store {
    let sessionsrv_config = data_store_config("builder_sessionsrv");

    let pool = create_pool(sessionsrv_config);

    let sessionsrv_data_store = sessionsrv_data_store {
                                    pool: pool
                                };
    sessionsrv_data_store
}

pub fn create_originsrv_data_store() -> originsrv_data_store {
    let originsrv_config = data_store_config("builder_originsrv");
    let pool = create_pool(originsrv_config);

    let ap = pool.clone();

    let originsrv_data_store = originsrv_data_store {
                                   pool: pool,
                                   async: hab_db::async::AsyncServer::new(ap)
                               };
    originsrv_data_store
}

fn data_store_config(database_name: &str) -> hab_db::config::DataStoreCfg {
    let config = hab_db::config::DataStoreCfg {
        host: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        port: 5432,
        user: String::from("hab"),
        password: None,
        database: String::from(database_name),
        connection_retry_ms: 300,
        connection_timeout_sec: 3600,
        connection_test: false,
        pool_size: (num_cpus::get() * 2) as u32,
    };

    config
}

fn create_pool_config_builder(config: hab_db::config::DataStoreCfg) -> r2d2::config::Builder<postgres::Connection, r2d2_postgres::Error> {
    let pool_builder = r2d2::Config::<postgres::Connection, r2d2_postgres::Error>::builder()
        .pool_size(config.pool_size)
        .connection_timeout(Duration::from_secs(config.connection_timeout_sec));
    pool_builder
}

fn r2d2_pool(config: hab_db::config::DataStoreCfg) -> r2d2::Pool<r2d2_postgres::PostgresConnectionManager> {
    let pool_config_builder = create_pool_config_builder(config.clone());
    let pool_config = pool_config_builder.build();

    let manager = PostgresConnectionManager::new(&config, TlsMode::None).unwrap();
    let r2d2_pool = r2d2::Pool::new(pool_config, manager).unwrap();
    r2d2_pool
}

fn create_pool(config: hab_db::config::DataStoreCfg) -> hab_db::pool::Pool {
    let mut shards: Vec<protocol::sharding::ShardId> = (1..128).collect();

    let r2d2_pool = r2d2_pool(config);

    let pool = hab_db::pool::Pool {
                   inner: r2d2_pool,
                   shards: shards
               };
    pool
}
