extern crate habitat_builder_sessionsrv as hab_sessionsrv;
extern crate habitat_builder_protocol as protocol;
extern crate habitat_core as hab_core;
extern crate habitat_net as hab_net;
#[macro_use]
extern crate habitat_builder_db as hab_db;
use hab_sessionsrv::data_store::DataStore as sessionsrv_data_store;
use protocol::sessionsrv::Session;
use hab_db::pool::Pool;
use std::ops::Deref;

extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate num_cpus;

use postgres::{Connection};
use r2d2_postgres::{PostgresConnectionManager, TlsMode};


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
    let address = "postgres://hab@127.0.0.1/builder_sessionsrv";

//    let pool_size = (num_cpus::get() * 2) as u32;
 //   let connection_timeout_sec = 3600;

//		let r2_config = r2d2::Config::<(), r2d2_postgres::Error>::default();
//    let r2_manager = PostgresConnectionManager::new(address, TlsMode::None).unwrap();

//    let r2_pool = r2d2::Pool::new(r2_config, r2_manager).unwrap();

    let postgres_connection = Connection::connect(address, postgres::TlsMode::None).unwrap();

    postgres_connection.query("set search_path to shard_0", &[]);

    let result = postgres_connection.query("SELECT * FROM ACCOUNTS", &[]);
    println!("{:?}", result);

}
