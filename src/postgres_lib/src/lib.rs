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
use std::net::{Ipv4Addr, IpAddr};
use std::time::Duration;
use std::error;
use std::thread;

use postgres::{Connection};
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

pub fn create_real_data_store() {
    let address = "postgres://hab@127.0.0.1/builder_sessionsrv";

    let config = builder_sessionsrv_config();

    let pool_config_builder =
        r2d2::Config::<postgres::Connection, r2d2_postgres::Error>::builder()
				    .pool_size(config.pool_size)
             .connection_timeout(Duration::from_secs(config.connection_timeout_sec));

println!("================");
println!("one");
println!("{:?}", pool_config_builder);

     let pool_config = pool_config_builder.build();

println!("================");
println!("two");
println!("{:?}", pool_config);

//     let manager = PostgresConnectionManager::new(&config, TlsMode::None);
     let manager = PostgresConnectionManager::new(&config, TlsMode::None).unwrap();

println!("================");
println!("three");
println!("{:?}", manager);

let pool = r2d2::Pool::new(pool_config, manager);
println!("================");
println!("three");
println!("{:?}", pool);

//let my_thing: () = manager;

//    match r2d2::Pool::new(pool_config, manager) {
//		Ok(pool) => {
//        println!("{:?}", pool);
//		 }
//		Err(e) => {
//       println!("ERRORZ");
//			error!("Error initializing connection pool to Postgres, will retry: {}",
//				 e)
//    }
//}

//println!("================");
//println!("four");
//println!("{:?}", pool);

//		let r2_config = r2d2::Config::<(), r2d2_postgres::Error>::default();
//    let r2_manager = PostgresConnectionManager::new(address, TlsMode::None).unwrap();

//    let r2_pool = r2d2::Pool::new(r2_config, r2_manager).unwrap();

//    let postgres_connection = Connection::connect(address, postgres::TlsMode::None).unwrap();

//    postgres_connection.query("set search_path to shard_0", &[]);

//    let result = postgres_connection.query("SELECT * FROM ACCOUNTS", &[]);
//    println!("{:?}", result);

}

    fn builder_sessionsrv_config() -> hab_db::config::DataStoreCfg {
        let config = hab_db::config::DataStoreCfg {
            host: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 5432,
            user: String::from("hab"),
            password: None,
            database: String::from("builder_sessionsrv"),
            connection_retry_ms: 300,
            connection_timeout_sec: 3600,
            connection_test: false,
            pool_size: (num_cpus::get() * 2) as u32
        };

        config
    }
